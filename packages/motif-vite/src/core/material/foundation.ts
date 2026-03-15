import type { Declaration, ParsedClass } from '../shared.js';
import { resolveMaterialFoundationBase } from './foundation/base.js';
import { resolveMaterialFoundationControls } from './foundation/controls.js';

export function resolveMaterialFoundation(parsed: ParsedClass): Declaration[] | null {
  return resolveMaterialFoundationBase(parsed) ?? resolveMaterialFoundationControls(parsed);
}
