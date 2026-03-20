import { execFileSync } from 'node:child_process';
import { pathFromRepo, repoRoot, runStep } from './common.mjs';
import { runMetaChecks } from './quality/meta-checks.mjs';
import { runRepoChecks } from './quality/repo-checks.mjs';

function git(args, required = true) {
  try {
    return execFileSync('git', args, { cwd: repoRoot, encoding: 'utf8' });
  } catch (error) {
    if (!required) return null;
    throw new Error(`git ${args.join(' ')} failed: ${error.message}`);
  }
}

function trackedFiles() {
  return git(['ls-files']).split(/\r?\n/).map((line) => line.trim().replaceAll('\\', '/')).filter(Boolean);
}

async function runCommandChecks() {
  await runStep('cargo fmt --all --check', 'cargo', ['fmt', '--all', '--check']);
  await runStep(
    'cargo clippy --workspace --all-targets --all-features -- -D warnings',
    'cargo',
    ['clippy', '--workspace', '--all-targets', '--all-features', '--', '-D', 'warnings']
  );
  await runStep('cargo test -p motif-core', 'cargo', ['test', '-p', 'motif-core']);

  const motifViteDir = pathFromRepo('packages/motif-vite');
  await runStep('motif-vite: install dependencies', 'npm', ['install', '--no-package-lock'], motifViteDir);
  await runStep('motif-vite: typecheck', 'npm', ['run', 'typecheck'], motifViteDir);
  await runStep('motif-vite: test', 'npm', ['test'], motifViteDir);
  await runStep('demo-builds', 'node', [pathFromRepo('scripts/node/demo-builds.mjs'), '--profile=full']);
}

async function main() {
  const failures = [];
  const warnings = [];
  const files = trackedFiles();

  runRepoChecks({ repoRoot, trackedFiles: files, failures, warnings });
  runMetaChecks({ repoRoot, trackedFiles: files, env: process.env, git, failures, warnings });

  try {
    await runCommandChecks();
  } catch (error) {
    failures.push(error.message);
  }

  if (warnings.length) {
    console.log('\nSoft warnings:');
    for (const warning of warnings) console.log(`- ${warning}`);
  }

  if (failures.length) {
    console.log('\nHard gate failures:');
    for (const failure of failures) console.log(`- ${failure}`);
    process.exit(1);
  }

  console.log('\nquality checks passed');
}

main().catch((error) => {
  console.error(`quality failed: ${error.message}`);
  process.exit(1);
});
