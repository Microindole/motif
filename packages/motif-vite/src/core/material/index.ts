import type { Declaration, ParsedClass } from '../shared.js';
import { resolveMaterialFoundation } from './foundation.js';
import { resolveMaterialShell } from './shell.js';
import { resolveMaterialData } from './data.js';

export function resolveMaterial(parsed: ParsedClass): Declaration[] | null {
  return resolveMaterialFoundation(parsed) ?? resolveMaterialShell(parsed) ?? resolveMaterialData(parsed);
}