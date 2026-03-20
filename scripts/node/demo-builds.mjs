import { fileURLToPath } from 'node:url';
import { npmProgram, parseOption, pathFromRepo, repoRoot, runStep } from './common.mjs';

const scriptPath = fileURLToPath(import.meta.url);

const tsDemos = [
  ['ts-basic', 'demo/ts/basic'],
  ['ts-variants', 'demo/ts/variants'],
  ['ts-theme', 'demo/ts/theme'],
  ['ts-workspace', 'demo/ts/workspace'],
];

const webDemos = [
  ['react-basic', 'demo/react/basic'],
  ['react-variants', 'demo/react/variants'],
  ['react-theme', 'demo/react/theme'],
  ['react-workspace', 'demo/react/workspace'],
  ['vue-basic', 'demo/vue/basic'],
  ['vue-variants', 'demo/vue/variants'],
  ['vue-theme', 'demo/vue/theme'],
  ['vue-workspace', 'demo/vue/workspace'],
];

async function buildTsDemo(name, relativePath) {
  const demoDir = pathFromRepo(relativePath);
  await runStep(
    `${name}: generate motif.css`,
    'cargo',
    ['run', '-p', 'motif-core', '--', '.'],
    demoDir
  );
  await runStep(
    `${name}: install dependencies`,
    npmProgram,
    ['install', '--no-package-lock'],
    demoDir
  );
  await runStep(`${name}: build`, npmProgram, ['run', 'build'], demoDir);
}

async function buildWebDemo(name, relativePath) {
  const demoDir = pathFromRepo(relativePath);
  await runStep(
    `${name}: install dependencies`,
    npmProgram,
    ['install', '--no-package-lock'],
    demoDir
  );
  await runStep(`${name}: build`, npmProgram, ['run', 'build'], demoDir);
}

async function main() {
  const profile = parseOption('--profile', 'full');

  if (profile !== 'full' && profile !== 'fast') {
    throw new Error(`unknown demo-builds profile \`${profile}\``);
  }

  if (profile === 'full') {
    for (const [name, relativePath] of tsDemos) {
      await buildTsDemo(name, relativePath);
    }
  }

  for (const [name, relativePath] of webDemos) {
    await buildWebDemo(name, relativePath);
  }
}

main().catch((error) => {
  console.error(`demo-builds failed: ${error.message}`);
  console.error(`hint: rerun with \`node ${pathFromRepo('scripts/node/demo-builds.mjs')} --profile=full\``);
  process.exit(1);
});
