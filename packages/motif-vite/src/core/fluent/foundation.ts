import type { Declaration, ParsedClass } from '../shared.js';
import { resolveFluentFoundationBase } from './foundation/base.js';
import { resolveFluentFoundationControls } from './foundation/controls.js';

export function resolveFluentFoundation(parsed: ParsedClass): Declaration[] | null {
  return resolveFluentFoundationBase(parsed) ?? resolveFluentFoundationControls(parsed);
}
