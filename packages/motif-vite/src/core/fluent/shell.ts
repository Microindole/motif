import type { Declaration, ParsedClass } from '../shared.js';
import { resolveFluentShellFeedback } from './shell/feedback.js';
import { resolveFluentShellLayout } from './shell/layout.js';

export function resolveFluentShell(parsed: ParsedClass): Declaration[] | null {
  return resolveFluentShellFeedback(parsed) ?? resolveFluentShellLayout(parsed);
}
