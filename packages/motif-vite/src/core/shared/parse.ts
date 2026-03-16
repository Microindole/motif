import { control, decl, text } from './declarations.js';
import type { Declaration, Family, ParsedClass, Variant } from './types.js';

const CLASS_BOUNDARY = /[\s"'`<>=\{\}\(\)\[\],;]/;
const CLASS_CHAR = /^[A-Za-z0-9:/_.-]+$/;

export function extractClassNames(content: string): Set<string> {
  const result = new Set<string>();
  for (const candidate of content.split(CLASS_BOUNDARY)) {
    const normalized = normalizeCandidate(candidate);
    if (normalized) {
      result.add(normalized);
    }
  }
  return result;
}

function normalizeCandidate(candidate: string): string | null {
  let start = 0;
  let end = candidate.length;

  while (start < end && !isClassChar(candidate[start]!)) {
    start += 1;
  }
  while (end > start && !isClassChar(candidate[end - 1]!)) {
    end -= 1;
  }

  const trimmed = candidate.slice(start, end);
  if (!trimmed || !CLASS_CHAR.test(trimmed)) {
    return null;
  }

  const utility = trimmed.split(':').at(-1) ?? trimmed;
  if (utility.startsWith('f-') || utility.startsWith('m-') || utility.startsWith('ui-')) {
    return trimmed;
  }
  return null;
}

function isClassChar(value: string): boolean {
  return /[A-Za-z0-9:/_.-]/.test(value);
}

export function parseClassName(input: string): ParsedClass | null {
  if (!input) {
    return null;
  }

  const segments = input.split(':');
  const base = segments.pop();
  if (!base) {
    return null;
  }

  const variants = segments.every(isVariant) ? (segments as Variant[]) : null;
  if (!variants) {
    return null;
  }

  const family: Family | null = base.startsWith('f-')
    ? 'f'
    : base.startsWith('m-')
      ? 'm'
      : base.startsWith('ui-')
        ? 'ui'
        : null;
  if (!family) {
    return null;
  }

  const body = family === 'ui' ? base.slice(3) : base.slice(2);
  if (!body) {
    return null;
  }

  const [utility, value] = splitUtility(body);
  if (!utility) {
    return null;
  }

  return { raw: input, variants, family, utility, value };
}

function isVariant(value: string): value is Variant {
  return value === 'hover' || value === 'focus' || value === 'active' || value === 'dark';
}

function splitUtility(body: string): [string, string | null] {
  const dash = body.indexOf('-');
  if (dash === -1) {
    return [body, null];
  }
  return [body.slice(0, dash), body.slice(dash + 1)];
}

export function resolveUniversal(parsed: ParsedClass): Declaration[] | null {
  switch (`${parsed.utility}:${parsed.value ?? ''}`) {
    case 'control:sm':
      return control('2.25rem', '0.625rem 0.85rem', '0.875rem');
    case 'control:md':
      return control('2.75rem', '0.78rem 0.95rem', '0.95rem');
    case 'control:lg':
      return control('3.25rem', '0.95rem 1.2rem', '1rem');
    case 'pad:sm':
      return [decl('padding', '0.75rem')];
    case 'pad:md':
      return [decl('padding', '1rem')];
    case 'pad:lg':
      return [decl('padding', '1.25rem')];
    case 'radius:sm':
      return [decl('border-radius', '6px')];
    case 'radius:md':
      return [decl('border-radius', '10px')];
    case 'radius:lg':
      return [decl('border-radius', '14px')];
    case 'radius:pill':
      return [decl('border-radius', '999px')];
    case 'text:sm':
      return text('0.875rem', '1.4');
    case 'text:md':
      return text('1rem', '1.5');
    case 'text:lg':
      return text('1.125rem', '1.55');
    default:
      return null;
  }
}
