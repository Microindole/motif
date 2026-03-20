import { spawn } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import path from 'node:path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

export const repoRoot = path.resolve(__dirname, '..', '..');
export const npmProgram = 'npm';

export function pathFromRepo(relativePath) {
  return path.join(repoRoot, ...relativePath.split('/'));
}

export function parseFlag(name, fallback = false) {
  return process.argv.includes(name) ? true : fallback;
}

export function parseOption(name, fallback) {
  const prefix = `${name}=`;
  const match = process.argv.find((arg) => arg.startsWith(prefix));
  return match ? match.slice(prefix.length) : fallback;
}

export async function runStep(name, command, args, cwd = repoRoot) {
  console.log(`==> ${name}`);

  await new Promise((resolve, reject) => {
    const child = spawn(command, args, {
      cwd,
      stdio: 'inherit',
      shell: true,
    });

    child.on('error', (error) => {
      reject(new Error(`failed to run \`${command}\`: ${error.message}`));
    });

    child.on('exit', (code) => {
      if (code === 0) {
        resolve();
        return;
      }
      reject(
        new Error(`${name} failed with exit code ${code === null ? 'null' : code}`)
      );
    });
  });
}
