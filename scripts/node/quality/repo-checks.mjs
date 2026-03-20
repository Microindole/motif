import fs from 'node:fs';
import path from 'node:path';
import { ARCHITECTURE_BOUNDARIES, FORBIDDEN_LIBRARY_PATTERNS, isGeneratedPath, isSourceFile, normalizePath, sourceFileLineLimit } from './source-rules.mjs';

const SUSPICIOUS_NAMES = ['helper', 'util', 'utils', 'manager', 'service', 'handler', 'processor', 'temp', 'misc', 'final', 'old', 'new'];

export function runRepoChecks({ repoRoot, trackedFiles, failures, warnings }) {
  checkGeneratedArtifacts(trackedFiles, failures);
  checkFileLineLimits(repoRoot, trackedFiles, failures, warnings);
  checkDirectoryFlatness(trackedFiles, failures);
  checkForbiddenPatterns(repoRoot, trackedFiles, failures);
  checkContextDocs(repoRoot, failures);
  checkNamingPatterns(trackedFiles, warnings);
  checkCommentHeuristics(repoRoot, trackedFiles, warnings);
  checkArchitectureBoundaries(repoRoot, trackedFiles, failures);
}

function checkGeneratedArtifacts(trackedFiles, failures) {
  for (const file of trackedFiles) {
    if (isGeneratedPath(file)) failures.push(`tracked generated artifact detected: ${file}`);
  }
}

function checkFileLineLimits(repoRoot, trackedFiles, failures, warnings) {
  for (const file of trackedFiles) {
    const fullPath = path.join(repoRoot, file);
    if (!fs.existsSync(fullPath) || !fs.statSync(fullPath).isFile() || !isSourceFile(file)) continue;
    const limit = sourceFileLineLimit(file);
    if (!limit) continue;
    const lines = readLines(fullPath).length;
    if (lines > limit) failures.push(`${file} is ${lines} lines, exceeds limit ${limit}`);
    else if (warnsOnLargeCoreFile(file) && lines > 240) warnings.push(`${file} is already large at ${lines} lines`);
  }
}

function warnsOnLargeCoreFile(file) {
  return (file.startsWith('core/src/') && file.endsWith('.rs') && !file.startsWith('core/src/rule/')) || file.startsWith('packages/');
}

function checkDirectoryFlatness(trackedFiles, failures) {
  const counts = new Map();
  for (const file of trackedFiles) {
    if (isGeneratedPath(file)) continue;
    const dir = file.includes('/') ? file.slice(0, file.lastIndexOf('/')) : '';
    counts.set(dir, (counts.get(dir) ?? 0) + 1);
  }
  for (const [dir, count] of counts) {
    if (dir && count > 12) failures.push(`directory ${dir} has ${count} tracked files; limit is 12`);
  }
}

function checkForbiddenPatterns(repoRoot, trackedFiles, failures) {
  for (const file of trackedFiles) {
    if (!(file.startsWith('core/src/') && file.endsWith('.rs') && file !== 'core/src/main.rs')) continue;
    const content = fs.readFileSync(path.join(repoRoot, file), 'utf8');
    for (const [needle, message] of FORBIDDEN_LIBRARY_PATTERNS) {
      if (content.includes(needle)) failures.push(`${file}: ${message}`);
    }
  }
}

function checkContextDocs(repoRoot, failures) {
  const required = ['agent/product.md', 'agent/quality.md', 'agent/presets.md', 'agent/scope.md', 'agent/architecture.md', 'agent/status.md', 'agent/rules.md'];
  const context = fs.readFileSync(path.join(repoRoot, 'agent/context.md'), 'utf8');
  for (const doc of required) {
    if (!fs.existsSync(path.join(repoRoot, doc))) failures.push(`required doc missing: ${doc}`);
    else if (!context.includes(doc)) failures.push(`agent/context.md is missing entry for ${doc}`);
  }
}

function checkNamingPatterns(trackedFiles, warnings) {
  const grouped = new Map();
  for (const file of trackedFiles) {
    const dir = file.includes('/') ? file.slice(0, file.lastIndexOf('/')) : '';
    const files = grouped.get(dir) ?? [];
    files.push(file);
    grouped.set(dir, files);
  }

  for (const [dir, files] of grouped) {
    const prefixes = new Map();
    const suffixes = new Map();
    for (const file of files) {
      const stem = file.split('/').at(-1).split('.')[0];
      const parts = stem.split(/[_-]/).filter(Boolean);
      if (parts.length < 2) continue;
      prefixes.set(parts[0], (prefixes.get(parts[0]) ?? 0) + 1);
      suffixes.set(parts.at(-1), (suffixes.get(parts.at(-1)) ?? 0) + 1);
    }
    for (const token of SUSPICIOUS_NAMES) {
      if ((prefixes.get(token) ?? 0) >= 4) warnings.push(`directory ${dir} has many files starting with suspicious token \`${token}\``);
      if ((suffixes.get(token) ?? 0) >= 4) warnings.push(`directory ${dir} has many files ending with suspicious token \`${token}\``);
    }
  }
}

function checkCommentHeuristics(repoRoot, trackedFiles, warnings) {
  for (const file of trackedFiles) {
    const fullPath = path.join(repoRoot, file);
    if (!fs.existsSync(fullPath) || !fs.statSync(fullPath).isFile() || !isSourceFile(file)) continue;
    const lines = readLines(fullPath);
    if (lines.length < 25) continue;
    const commentLines = lines.filter((line) => isCommentLine(line)).length;
    const ratio = commentLines / lines.length;
    if (ratio > 0.28 && lines.length < 140) warnings.push(`${file} has unusually high comment density (${(ratio * 100).toFixed(1)}%)`);
    const branchSignals = lines.filter((line) => ['if', 'match', 'for', 'while', 'switch'].some((token) => line.includes(token))).length;
    if (lines.length >= 80 && branchSignals >= 8 && commentLines === 0) warnings.push(`${file} is complex and has no comments explaining constraints or tradeoffs`);
  }
}

function checkArchitectureBoundaries(repoRoot, trackedFiles, failures) {
  for (const file of trackedFiles) {
    if (!(file.startsWith('core/src/') && file.endsWith('.rs'))) continue;
    const content = fs.readFileSync(path.join(repoRoot, file), 'utf8');
    for (const [prefix, forbidden] of ARCHITECTURE_BOUNDARIES) {
      if (!file.startsWith(prefix)) continue;
      for (const [needle, message] of forbidden) {
        if (content.includes(needle)) failures.push(`${file}: ${message}`);
      }
    }
  }
}

function isCommentLine(line) {
  const trimmed = line.trimStart();
  return trimmed.startsWith('//') || trimmed.startsWith('/*') || trimmed.startsWith('*') || trimmed.startsWith('<!--') || trimmed.startsWith('#');
}

function readLines(fullPath) {
  return normalizePath(fs.readFileSync(fullPath, 'utf8')).split('\n');
}
