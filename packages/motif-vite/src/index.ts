import { promises as fs } from 'node:fs';
import path from 'node:path';
import type { ModuleNode, Plugin, ViteDevServer } from 'vite';
import { createTsCoreAdapter } from './core.js';
import type { MotifCompileAdapter, MotifCompileResult, MotifSourceFile } from './types.js';

const DEFAULT_EXTENSIONS = new Set(['.html', '.jsx', '.ts', '.tsx', '.vue', '.svelte']);
const DEFAULT_IGNORED_DIRECTORIES = new Set([
  '.git',
  'node_modules',
  'target',
  'dist',
  'build',
  'coverage',
  '.vite',
]);
const VIRTUAL_ID = 'virtual:motif.css';
const RESOLVED_VIRTUAL_ID = '\0virtual:motif.css';

export type { MotifCompileAdapter, MotifCompileResult, MotifSourceFile } from './types.js';
export { createTsCoreAdapter } from './core.js';

export interface MotifVitePluginOptions {
  adapter?: MotifCompileAdapter;
  include?: (id: string) => boolean;
  extensions?: string[];
  virtualId?: string;
}

interface MotifPluginState {
  root: string;
  stylesheet: string | null;
  server: ViteDevServer | null;
  extensions: Set<string>;
  include: (id: string) => boolean;
  virtualId: string;
  resolvedVirtualId: string;
}

export function motifVite(options: MotifVitePluginOptions = {}): Plugin {
  const virtualId = options.virtualId ?? VIRTUAL_ID;
  const resolvedVirtualId = virtualId === VIRTUAL_ID ? RESOLVED_VIRTUAL_ID : `\0${virtualId}`;
  const adapter = options.adapter ?? createTsCoreAdapter();
  const state: MotifPluginState = {
    root: process.cwd(),
    stylesheet: null,
    server: null,
    extensions: new Set(options.extensions ?? Array.from(DEFAULT_EXTENSIONS)),
    include: options.include ?? defaultInclude,
    virtualId,
    resolvedVirtualId,
  };

  return {
    name: 'motif-vite',
    enforce: 'pre',
    configResolved(config) {
      state.root = config.root;
    },
    configureServer(server) {
      state.server = server;
    },
    resolveId(id) {
      if (id === state.virtualId) {
        return state.resolvedVirtualId;
      }
      return null;
    },
    async load(id) {
      if (id !== state.resolvedVirtualId) {
        return null;
      }

      if (state.stylesheet === null) {
        state.stylesheet = await compileStylesheet(state, adapter);
      }
      return state.stylesheet;
    },
    async handleHotUpdate(ctx) {
      if (!state.include(ctx.file)) {
        return;
      }

      state.stylesheet = await compileStylesheet(state, adapter);
      const virtualModule = state.server?.moduleGraph.getModuleById(state.resolvedVirtualId);
      if (!virtualModule) {
        triggerStylesheetReload(state.server);
        return;
      }

      invalidateVirtualModule(state.server, virtualModule);
      triggerStylesheetReload(state.server);
      return [virtualModule];
    },
  };
}

function defaultInclude(id: string): boolean {
  return DEFAULT_EXTENSIONS.has(path.extname(id));
}

async function compileStylesheet(
  state: MotifPluginState,
  adapter: MotifCompileAdapter,
): Promise<string> {
  const sources = await collectSources(state.root, state.include, state.extensions);
  const result = await adapter.compileSources(sources);
  return result.stylesheet;
}

async function collectSources(
  root: string,
  include: (id: string) => boolean,
  extensions: Set<string>,
): Promise<MotifSourceFile[]> {
  const sources: MotifSourceFile[] = [];
  await visitDirectory(root, include, extensions, sources);
  sources.sort((left, right) => left.path.localeCompare(right.path));
  return sources;
}

async function visitDirectory(
  current: string,
  include: (id: string) => boolean,
  extensions: Set<string>,
  sources: MotifSourceFile[],
): Promise<void> {
  const entries = await fs.readdir(current, { withFileTypes: true });

  for (const entry of entries) {
    const fullPath = path.join(current, entry.name);
    if (entry.isDirectory()) {
      if (DEFAULT_IGNORED_DIRECTORIES.has(entry.name)) {
        continue;
      }
      await visitDirectory(fullPath, include, extensions, sources);
      continue;
    }

    if (!extensions.has(path.extname(entry.name)) || !include(fullPath)) {
      continue;
    }

    const content = await fs.readFile(fullPath, 'utf8');
    sources.push({ path: fullPath, content });
  }
}

function invalidateVirtualModule(server: ViteDevServer | null, moduleNode: ModuleNode): void {
  if (!server) {
    return;
  }
  server.moduleGraph.invalidateModule(moduleNode);
}

function triggerStylesheetReload(server: ViteDevServer | null): void {
  if (!server) {
    return;
  }

  // Start with the simplest reliable dev loop: rebuild the virtual stylesheet
  // and force the browser to pick it up immediately.
  server.ws.send({ type: 'full-reload' });
}
