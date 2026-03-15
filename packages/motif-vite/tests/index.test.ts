import test from 'node:test';
import assert from 'node:assert/strict';
import os from 'node:os';
import path from 'node:path';
import { promises as fs } from 'node:fs';
import type { ModuleNode, ViteDevServer } from 'vite';
import { motifVite } from '../src/index.js';
import type { MotifCompileAdapter, MotifSourceFile } from '../src/types.js';

function hookHandler<T extends (...args: any[]) => any>(
  hook: T | { handler: T } | undefined,
): T | undefined {
  if (!hook) {
    return undefined;
  }
  return typeof hook === 'function' ? hook : hook.handler;
}

function callHook<T extends (this: any, ...args: any[]) => any>(
  hook: T | { handler: T } | undefined,
  ...args: Parameters<T>
): ReturnType<T> | undefined {
  const handler = hookHandler(hook);
  if (!handler) {
    return undefined;
  }
  return handler.call({} as never, ...args);
}

test('motifVite resolves and loads the virtual stylesheet through the adapter', async () => {
  const root = await fs.mkdtemp(path.join(os.tmpdir(), 'motif-vite-plugin-'));
  await fs.writeFile(
    path.join(root, 'App.tsx'),
    '<div className="f-surface m-title"></div>',
    'utf8',
  );

  const seenSources: MotifSourceFile[][] = [];
  const adapter: MotifCompileAdapter = {
    compileSources(sources) {
      seenSources.push(sources);
      return { stylesheet: '/* test stylesheet */\n.f-surface { color: red; }\n' };
    },
  };

  const plugin = motifVite({ adapter });
  callHook(plugin.configResolved, { root } as never);

  const resolvedId = callHook(plugin.resolveId, 'virtual:motif.css', undefined, {
    attributes: {},
    custom: undefined,
    isEntry: false,
    ssr: false,
  });
  assert.equal(resolvedId, '\0virtual:motif.css');

  const stylesheet = await callHook(plugin.load, '\0virtual:motif.css', {
    ssr: false,
  });
  assert.equal(stylesheet, '/* test stylesheet */\n.f-surface { color: red; }\n');
  assert.equal(seenSources.length, 1);
  assert.equal(seenSources[0]?.length, 1);
  assert.equal(seenSources[0]?.[0]?.path, path.join(root, 'App.tsx'));
});

test('motifVite invalidates the virtual module and triggers a reload on hot updates', async () => {
  const root = await fs.mkdtemp(path.join(os.tmpdir(), 'motif-vite-hot-'));
  await fs.writeFile(path.join(root, 'App.tsx'), '<div className="f-surface"></div>', 'utf8');
  const moduleNode = { id: '\0virtual:motif.css' } as ModuleNode;
  const invalidated: ModuleNode[] = [];
  const wsMessages: unknown[] = [];

  const server = {
    moduleGraph: {
      getModuleById(id: string) {
        return id === '\0virtual:motif.css' ? moduleNode : null;
      },
      invalidateModule(node: ModuleNode) {
        invalidated.push(node);
      },
    },
    ws: {
      send(payload: unknown) {
        wsMessages.push(payload);
      },
    },
  } as unknown as ViteDevServer;

  const plugin = motifVite({
    adapter: {
      compileSources() {
        return { stylesheet: '.f-surface { color: red; }' };
      },
    },
  });

  callHook(plugin.configResolved, { root } as never);
  callHook(plugin.configureServer, server);

  const modules = await callHook(plugin.handleHotUpdate, {
    file: path.join(root, 'App.tsx'),
  } as never);

  assert.deepEqual(modules, [moduleNode]);
  assert.deepEqual(invalidated, [moduleNode]);
  assert.deepEqual(wsMessages, [{ type: 'full-reload' }]);
});
