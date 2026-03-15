import type { MotifCompileAdapter, MotifCompileResult, MotifSourceFile } from './types.js';
import {
  extractClassNames,
  parseClassName,
  renderStylesheet,
  resolveUniversal,
  type ParsedClass,
  type RuleMatch,
} from './core/shared.js';
import { resolveFluent } from './core/fluent/index.js';
import { resolveMaterial } from './core/material/index.js';

export function createTsCoreAdapter(): MotifCompileAdapter {
  return {
    compileSources(sources) {
      return compileSources(sources);
    },
  };
}

export function compileSources(sources: MotifSourceFile[]): MotifCompileResult {
  const classNames = new Set<string>();
  for (const source of sources) {
    for (const className of extractClassNames(source.content)) {
      classNames.add(className);
    }
  }

  const rules = Array.from(classNames)
    .sort((left, right) => left.localeCompare(right))
    .map(parseClassName)
    .filter((parsed): parsed is ParsedClass => parsed !== null)
    .map(resolveRule)
    .filter((rule): rule is RuleMatch => rule !== null);

  return {
    stylesheet: renderStylesheet(rules),
  };
}

function resolveRule(parsed: ParsedClass): RuleMatch | null {
  const declarations =
    parsed.family === 'f'
      ? resolveFluent(parsed)
      : parsed.family === 'm'
        ? resolveMaterial(parsed)
        : resolveUniversal(parsed);
  if (!declarations) {
    return null;
  }
  return {
    rawClassName: parsed.raw,
    variants: parsed.variants,
    declarations,
  };
}