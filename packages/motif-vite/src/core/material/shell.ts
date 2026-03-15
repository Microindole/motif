import type { Declaration, ParsedClass } from '../shared.js';
import { resolveMaterialShellFeedback } from './shell/feedback.js';
import { resolveMaterialShellLayout } from './shell/layout.js';

export function resolveMaterialShell(parsed: ParsedClass): Declaration[] | null {
  return resolveMaterialShellFeedback(parsed) ?? resolveMaterialShellLayout(parsed);
}
