const GENERATED_PATH_MARKERS = [
  '/target/',
  '/node_modules/',
  '/dist/',
  '/coverage/',
  '/.vite/',
];

export const FORBIDDEN_LIBRARY_PATTERNS = [
  ['unsafe', 'unsafe is forbidden outside explicit review'],
  ['transmute', 'transmute is forbidden'],
  ['MaybeUninit', 'MaybeUninit is forbidden'],
  ['todo!(', 'todo! is forbidden'],
  ['unimplemented!(', 'unimplemented! is forbidden'],
  ['dbg!(', 'dbg! is forbidden'],
  ['.unwrap(', 'unwrap() is forbidden in library code'],
  ['.expect(', 'expect() is forbidden in library code'],
  ['println!(', 'println! is forbidden in library code'],
  ['eprintln!(', 'eprintln! is forbidden in library code'],
];

export const ARCHITECTURE_BOUNDARIES = [
  ['core/src/scan/', [['crate::gen', 'scan layer must not depend on gen'], ['crate::rule', 'scan layer must not depend on rule'], ['crate::token', 'scan layer must not depend on token'], ['crate::write', 'scan layer must not depend on write']]],
  ['core/src/parse/', [['crate::scan', 'parse layer must not depend on scan'], ['crate::rule', 'parse layer must not depend on rule'], ['crate::token', 'parse layer must not depend on token'], ['crate::gen', 'parse layer must not depend on gen'], ['crate::write', 'parse layer must not depend on write']]],
  ['core/src/token/', [['std::fs', 'token layer must not do file IO'], ['std::io', 'token layer must not do file IO'], ['crate::scan', 'token layer must not depend on scan'], ['crate::gen', 'token layer must not depend on gen'], ['crate::write', 'token layer must not depend on write']]],
  ['core/src/rule/', [['std::fs', 'rule layer must not do file IO'], ['std::io', 'rule layer must not do file IO'], ['crate::scan', 'rule layer must not depend on scan'], ['crate::gen', 'rule layer must not depend on gen'], ['crate::write', 'rule layer must not depend on write']]],
  ['core/src/gen/', [['std::fs', 'gen layer must not do file IO'], ['std::io', 'gen layer must not do file IO'], ['crate::scan', 'gen layer must not depend on scan'], ['crate::write', 'gen layer must not depend on write']]],
  ['core/src/write/', [['crate::scan', 'write layer must not depend on scan'], ['crate::parse', 'write layer must not depend on parse'], ['crate::rule', 'write layer must not depend on rule'], ['crate::token', 'write layer must not depend on token'], ['crate::gen', 'write layer must not depend on gen']]],
];

export function normalizePath(value) {
  return value.replaceAll('\\', '/');
}

export function isGeneratedPath(file) {
  return GENERATED_PATH_MARKERS.some((marker) => file.includes(marker))
    || file.endsWith('/motif.css')
    || file.endsWith('.tsbuildinfo');
}

export function isCodeExtension(file) {
  return ['.rs', '.ts', '.tsx', '.js', '.jsx', '.vue', '.svelte', '.ps1', '.sh']
    .some((ext) => file.endsWith(ext));
}

export function isSourceFile(file) {
  return !isGeneratedPath(file) && isCodeExtension(file);
}

export function sourceFileLineLimit(file) {
  if (file.startsWith('core/src/rule/') && file.endsWith('.rs') && slashCount(file) >= 4) return 360;
  if (file.startsWith('core/src/rule/') && file.endsWith('.rs')) return 240;
  if (file.startsWith('core/src/') && file.endsWith('.rs')) return 320;
  if (file.startsWith('core/tests/') && file.endsWith('.rs')) return 420;
  if (file.startsWith('xtask/src/') && file.endsWith('.rs')) return 320;
  if (file.startsWith('xtask/tests/') && file.endsWith('.rs')) return 320;
  if (file.startsWith('scripts/') && (file.endsWith('.ps1') || file.endsWith('.sh'))) return 320;
  if (file.startsWith('packages/motif-vite/src/core/') && isCodeExtension(file) && slashCount(file) >= 6) return 280;
  if (file.startsWith('packages/motif-vite/src/') && isCodeExtension(file) && slashCount(file) >= 5) return 220;
  if (file.startsWith('packages/') && isCodeExtension(file)) return 360;
  if ((file.startsWith('demo/') || file.startsWith('cases/')) && isSourceFile(file)) return 260;
  if (isSourceFile(file)) return 320;
  return null;
}

function slashCount(file) {
  return [...file].filter((ch) => ch === '/').length;
}
