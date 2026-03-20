import { fileURLToPath } from 'node:url';
import { npmProgram, pathFromRepo, repoRoot, runStep } from './common.mjs';

const scriptPath = fileURLToPath(import.meta.url);

async function main() {
  const motifViteDir = pathFromRepo('packages/motif-vite');

  await runStep(
    'motif-vite: install dependencies',
    npmProgram,
    ['install', '--no-package-lock'],
    motifViteDir
  );
  await runStep('motif-vite: typecheck', npmProgram, ['run', 'typecheck'], motifViteDir);
  await runStep('motif-vite: test', npmProgram, ['test'], motifViteDir);
  await runStep(
    'web demo builds (fast profile)',
    'node',
    [scriptPath.replace('quality-fast.mjs', 'demo-builds.mjs'), '--profile=fast'],
    repoRoot
  );

  console.log('\nfast quality checks passed');
  console.log('full repo gates remain available via `cargo run -p xtask -- quality`');
}

main().catch((error) => {
  console.error(`quality-fast failed: ${error.message}`);
  process.exit(1);
});
