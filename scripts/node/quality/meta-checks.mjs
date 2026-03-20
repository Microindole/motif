import fs from 'node:fs';
import path from 'node:path';
import { readGithubEventBody, readGithubHeadSha } from './github.mjs';
import { isGeneratedPath, normalizePath } from './source-rules.mjs';

const REQUIRED_PR_SECTIONS = ['## Summary', '## Hard checks', '## Structure review', '## AI-specific review', '## Large change override'];
const LARGE_CHANGE_BOX = '- [x] This PR intentionally exceeds the change-size gate and requires explicit human review.';
const LARGE_CHANGE_BOX_ALT = '- [X] This PR intentionally exceeds the change-size gate and requires explicit human review.';

export function runMetaChecks({ repoRoot, trackedFiles, env, git, failures, warnings }) {
  checkDependencyHygiene({ repoRoot, trackedFiles, git, failures, warnings });
  checkChangeSize({ repoRoot, env, git, failures, warnings });
  checkCommitMessage({ repoRoot, env, git, failures, warnings });
  checkPrDescription({ env, failures, warnings });
  checkDuplicateBlocks({ repoRoot, trackedFiles, git, failures, warnings });
}

function checkDependencyHygiene({ repoRoot, trackedFiles, git, failures, warnings }) {
  for (const [manifest, warn, fail] of [['core/Cargo.toml', 4, 8], ['xtask/Cargo.toml', 3, 6]]) {
    const count = countCargoDependencies(fs.readFileSync(path.join(repoRoot, manifest), 'utf8'));
    if (count > fail) failures.push(`${manifest} declares ${count} direct dependencies; hard limit is ${fail}`);
    else if (count > warn) warnings.push(`[warn] ${manifest} declares ${count} direct dependencies; review whether they all pay for themselves`);
  }

  const versionMap = new Map();
  for (const file of trackedFiles.filter((file) => file.endsWith('package.json'))) {
    const content = fs.readFileSync(path.join(repoRoot, file), 'utf8');
    const json = JSON.parse(content);
    const depCount = Object.keys(json.dependencies ?? {}).length + Object.keys(json.devDependencies ?? {}).length;
    if (depCount > 10) failures.push(`${file} declares ${depCount} npm dependencies; hard limit is 10`);
    else if (depCount > 8) warnings.push(`[warn] ${file} declares ${depCount} npm dependencies; review whether the demo is pulling too much`);
    for (const section of ['dependencies', 'devDependencies']) {
      for (const [name, version] of Object.entries(json[section] ?? {})) {
        const set = versionMap.get(name) ?? new Set();
        set.add(version);
        versionMap.set(name, set);
      }
    }
  }
  for (const [name, versions] of versionMap) {
    if (versions.size >= 2) warnings.push(`[warn] npm dependency \`${name}\` uses multiple versions across demos: ${[...versions].join(', ')}`);
  }

  const spec = diffSpec(repoRoot, process.env, git);
  if (!spec) {
    warnings.push('[info] dependency-diff check skipped: no usable diff base available');
    return;
  }
  for (const manifest of ['core/Cargo.toml', 'xtask/Cargo.toml']) {
    const diff = git(['diff', '--unified=20', spec.range, '--', manifest]);
    const names = extractAddedCargoDependencies(diff);
    if (!names.length) continue;
    const message = `${manifest} adds direct dependencies in ${spec.label}: ${names.join(', ')}`;
    if (spec.hardGate) failures.push(message);
    else warnings.push(`[warn] ${message}; local branch signal only`);
  }
}

function checkChangeSize({ repoRoot, env, git, failures, warnings }) {
  const spec = diffSpec(repoRoot, env, git);
  if (!spec) return warnings.push('change-size check skipped: no usable diff base available');
  const output = git(['diff', '--numstat', spec.range], false);
  if (output === null) return warnings.push(`change-size check skipped: failed to diff ${spec.label}`);
  const counts = collectCounts(output);
  for (const [label, value, warn, fail] of [['files', counts.files, 12, 24], ['adds', counts.added, 300, 700], ['deletes', counts.deleted, 200, 500]]) {
    const message = evaluateMetric(label, value, warn, fail, spec, activeLargeChangeOverride(env));
    if (!message) continue;
    const [kind, text] = message;
    (kind === 'failure' ? failures : warnings).push(text);
  }
}

function checkCommitMessage({ repoRoot, env, git, failures, warnings }) {
  const revision = readGithubHeadSha(env) ?? 'HEAD';
  const message = git(['log', '-1', '--pretty=%s%n%b', revision], false);
  if (message === null) return warnings.push(`commit-message check skipped: unable to read commit message for ${revision}`);
  const shortstat = git(['show', '--format=', '--shortstat', '--no-renames', revision], false) ?? '';
  const nonTrivial = parseShortstat(shortstat).files >= 3 || parseShortstat(shortstat).changedLines >= 40;
  const [subject, ...bodyLines] = message.split(/\r?\n/);
  const body = bodyLines.join('\n').trim();
  if (!subject?.trim()) return failures.push('HEAD commit message subject is empty');
  if (subject.length > 72) failures.push(`HEAD commit subject is ${subject.length} chars; limit is 72`);
  if (subject.endsWith('.')) failures.push('HEAD commit subject must not end with a period');
  const match = subject.match(/^([a-z]+)(\([^)]+\))?: (.+)$/);
  if (!match) failures.push('HEAD commit subject must match `type: description` or `type(scope): description`');
  else {
    if (!['feat', 'fix', 'refactor', 'docs', 'test', 'build', 'ci', 'chore', 'perf', 'style'].includes(match[1])) failures.push(`HEAD commit subject uses unsupported type \`${match[1]}\``);
    if (!match[3].trim()) failures.push('HEAD commit subject must include a description after `: `');
    else if (/^[A-Z]/.test(match[3].trim())) warnings.push('HEAD commit subject description starts with uppercase; prefer sentence fragment style');
  }
  if (!body && nonTrivial) warnings.push('HEAD commit has no body; add one when the change is non-trivial');
}

function checkPrDescription({ env, failures, warnings }) {
  const body = readGithubEventBody(env);
  if (body === null) return;
  if (!body.trim()) return warnings.push('PR description is empty; add one when the change is non-trivial');
  for (const section of REQUIRED_PR_SECTIONS) if (!body.includes(section)) failures.push(`PR description is missing required section \`${section}\``);
  const summary = extractSection(body, '## Summary', '## Hard checks');
  if (summary && summary.split(/\r?\n/).map((line) => line.trim()).filter(Boolean).every((line) => line.startsWith('- ['))) failures.push('PR Summary must include at least one non-checkbox line explaining the change');
  if ((body.includes(LARGE_CHANGE_BOX) || body.includes(LARGE_CHANGE_BOX_ALT)) && !largeChangeOverride(body)) failures.push('Large change override is checked but missing rationale under `## Large change override`');
  const unchecked = body.split(/\r?\n/).filter((line) => line.trim().startsWith('- [ ]') && !insideLargeChangeSection(body, line)).length;
  if (unchecked > 0) failures.push(`PR description still contains ${unchecked} unchecked template item(s); complete them or delete them`);
}

function checkDuplicateBlocks({ repoRoot, trackedFiles, git, failures, warnings }) {
  const windows = new Map();
  const changed = new Set((git(['diff', '--name-only', diffSpec(repoRoot, process.env, git)?.range ?? ''], false) ?? '').split(/\r?\n/).map(normalizePath).filter(isDuplicateScanTarget));
  for (const file of trackedFiles) {
    const fullPath = path.join(repoRoot, file);
    if (!fs.existsSync(fullPath) || !fs.statSync(fullPath).isFile() || !isDuplicateScanTarget(file)) continue;
    const normalized = fs.readFileSync(fullPath, 'utf8').split(/\r?\n/).map((line) => line.trim()).filter((line) => line && !isCommentOnly(line));
    if (normalized.length < 16) continue;
    for (let i = 0; i <= normalized.length - 16; i += 1) {
      const fingerprint = normalized.slice(i, i + 16).join('\n');
      const files = windows.get(fingerprint) ?? new Set();
      files.add(file);
      windows.set(fingerprint, files);
    }
  }
  const reports = [...windows.values()].map((files) => [...files].sort()).filter((files) => files.length >= 2 && !isAllowedMirroredRuleReport(files));
  const deduped = [...new Map(reports.map((files) => [files.join('|'), files])).values()];
  if (deduped.filter((files) => files.some((file) => changed.has(file))).length >= 1) failures.push('change set introduces 1 duplicate block group(s) touching changed files');
  for (const files of deduped.slice(0, 5)) warnings.push(`possible duplicate block across ${files.join(', ')}`);
}

function diffSpec(repoRoot, env, git) {
  if (env.GITHUB_BASE_REF) {
    const remoteRef = `origin/${env.GITHUB_BASE_REF}`;
    const range = mergeBaseRange(remoteRef, git);
    if (range) return { range, label: `merge-base with ${remoteRef}`, hardGate: true };
  }
  if (git(['rev-parse', '--verify', 'HEAD~1'], false) !== null) return { range: 'HEAD~1..HEAD', label: 'HEAD~1..HEAD', hardGate: true };
  const branch = (git(['rev-parse', '--abbrev-ref', 'HEAD'], false) ?? '').trim();
  if (branch !== 'main' && branch !== 'master') for (const remoteRef of ['origin/main', 'origin/master']) {
    const range = mergeBaseRange(remoteRef, git);
    if (range) return { range, label: `merge-base with ${remoteRef}`, hardGate: false };
  }
  return null;
}

function mergeBaseRange(target, git) {
  if (git(['rev-parse', '--verify', target], false) === null) return null;
  const base = git(['merge-base', 'HEAD', target], false)?.trim();
  return base ? `${base}..HEAD` : null;
}

function collectCounts(output) {
  const counts = { files: 0, added: 0, deleted: 0 };
  for (const line of output.split(/\r?\n/)) {
    const [add, remove, file] = line.trim().split(/\s+/);
    if (!file || isGeneratedPath(file)) continue;
    counts.files += 1;
    counts.added += Number(add) || 0;
    counts.deleted += Number(remove) || 0;
  }
  return counts;
}

function evaluateMetric(label, value, warn, fail, spec, override) {
  const verb = label === 'files' ? 'touches' : label === 'adds' ? 'adds' : label === 'deletes' ? 'deletes' : 'changes';
  if (value > fail) {
    const message = `change set ${verb} ${value} ${label} in ${spec.label}; hard limit is ${fail}`;
    if (override) return ['warning', `${message}; large-change override is active and still requires explicit review: ${override}`];
    return [spec.hardGate ? 'failure' : 'warning', spec.hardGate ? message : `${message}; local branch signal only`];
  }
  if (value > warn) return ['warning', `change set ${verb} ${value} ${label} in ${spec.label}; consider splitting the work`];
  return null;
}

function activeLargeChangeOverride(env) {
  const body = readGithubEventBody(env);
  return body ? largeChangeOverride(body) : null;
}

function largeChangeOverride(body) {
  if (!(body.includes(LARGE_CHANGE_BOX) || body.includes(LARGE_CHANGE_BOX_ALT))) return null;
  const section = body.slice(body.indexOf('## Large change override') + '## Large change override'.length);
  const rationale = section.split(/\r?\n/).map((line) => line.trim()).filter((line) => line && !line.startsWith('- [')).join(' ');
  return rationale || null;
}

function extractSection(body, start, end) {
  const startIndex = body.indexOf(start);
  const endIndex = body.indexOf(end);
  if (startIndex === -1 || endIndex === -1 || endIndex <= startIndex) return null;
  return body.slice(startIndex + start.length, endIndex);
}

function insideLargeChangeSection(body, targetLine) {
  let inSection = false;
  for (const line of body.split(/\r?\n/)) {
    const trimmed = line.trim();
    if (trimmed.startsWith('## ')) inSection = trimmed === '## Large change override';
    if (line === targetLine) return inSection;
  }
  return false;
}

function countCargoDependencies(content) {
  let inDeps = false;
  let count = 0;
  for (const line of content.split(/\r?\n/)) {
    const trimmed = line.trim();
    if (trimmed.startsWith('[')) inDeps = trimmed === '[dependencies]';
    else if (inDeps && trimmed && !trimmed.startsWith('#') && trimmed.includes('=')) count += 1;
  }
  return count;
}

function extractAddedCargoDependencies(diff) {
  const names = [];
  let inDeps = false;
  for (const line of diff.split(/\r?\n/)) {
    if (!line || line.startsWith('+++') || line.startsWith('---')) continue;
    const prefix = line[0];
    const body = line.slice(1).trim();
    if (body.startsWith('[')) inDeps = body === '[dependencies]';
    else if (prefix === '+' && inDeps && body && !body.startsWith('#') && body.includes('=')) names.push(body.split('=')[0].trim());
  }
  return [...new Set(names)].sort();
}

function parseShortstat(output) {
  const stats = { files: 0, changedLines: 0 };
  const line = output.split(/\r?\n/).find((value) => value.includes('changed'));
  if (!line) return stats;
  for (const part of line.split(',')) {
    const count = Number(part.trim().split(/\s+/)[0]) || 0;
    if (part.includes('file')) stats.files = count;
    if (part.includes('insertion') || part.includes('deletion')) stats.changedLines += count;
  }
  return stats;
}

function isDuplicateScanTarget(file) {
  return (file.startsWith('core/src/') || file.startsWith('core/tests/') || file.startsWith('xtask/src/') || file.startsWith('scripts/')) && ['.rs', '.ps1'].some((ext) => file.endsWith(ext));
}

function isAllowedMirroredRuleReport(files) {
  if (files.length !== 2) return false;
  const [first, second] = files.map((file) => normalizePath(file)).sort();
  const left = first.replace(/^core\/src\/rule\/fluent\/?/, '');
  const right = second.replace(/^core\/src\/rule\/material\/?/, '');
  return left === right;
}

function isCommentOnly(line) {
  return line.startsWith('//') || line.startsWith('/*') || line.startsWith('*') || line.startsWith('#');
}
