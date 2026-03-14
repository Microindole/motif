import type { Declaration, ParsedClass } from '../shared.js';
import { resolveFluentFoundation } from './foundation.js';
import { resolveFluentShell } from './shell.js';
import { resolveFluentData } from './data.js';

export function resolveFluent(parsed: ParsedClass): Declaration[] | null {
  return resolveFluentFoundation(parsed) ?? resolveFluentShell(parsed) ?? resolveFluentData(parsed);
}