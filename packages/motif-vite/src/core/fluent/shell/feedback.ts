import type { Declaration, ParsedClass } from '../../shared.js';
import { decl, fluent, fluentTypography, tokenDecl, withTransition } from '../../shared.js';

export function resolveFluentShellFeedback(parsed: ParsedClass): Declaration[] | null {
  switch (`${parsed.utility}:${parsed.value ?? ''}`) {
    case 'badge:':
      return [
        ...fluentTypography('label-size', 'label-weight'),
        decl('display', 'inline-flex'),
        decl('align-items', 'center'),
        decl('min-height', '1.75rem'),
        tokenDecl('color', fluent('color', 'primary')),
        tokenDecl('background-color', fluent('color', 'surface-alt')),
        tokenDecl('border', fluent('border', 'action-subtle')),
        decl('border-radius', '999px'),
        tokenDecl('padding', fluent('space', 'surface-pad-sm')),
      ];
    case 'chip:':
      return withTransition(
        [
          ...fluentTypography('label-size', 'label-weight'),
          decl('display', 'inline-flex'),
          decl('align-items', 'center'),
          decl('gap', '0.5rem'),
          decl('min-height', '2rem'),
          tokenDecl('color', fluent('color', 'text')),
          tokenDecl('background-color', fluent('color', 'surface-alt')),
          tokenDecl('border', fluent('border', 'action-subtle')),
          decl('border-radius', '999px'),
          tokenDecl('padding', fluent('space', 'surface-pad-sm')),
        ],
        fluent('effect', 'interactive-transition'),
        fluent('motion', 'duration'),
        fluent('motion', 'easing'),
      );
    case 'tag:':
      return withTransition(
        [
          ...fluentTypography('label-size', 'label-weight'),
          decl('display', 'inline-flex'),
          decl('align-items', 'center'),
          decl('min-height', '1.5rem'),
          tokenDecl('color', fluent('color', 'primary')),
          decl('background-color', 'transparent'),
          tokenDecl('border', fluent('border', 'action-subtle')),
          tokenDecl('border-radius', fluent('radius', 'sm')),
          decl('padding', '0.2rem 0.55rem'),
        ],
        fluent('effect', 'interactive-transition'),
        fluent('motion', 'duration'),
        fluent('motion', 'easing'),
      );
    case 'tooltip:':
      return [
        ...fluentTypography('label-size', 'label-weight'),
        tokenDecl('color', fluent('color', 'text')),
        tokenDecl('background-color', fluent('color', 'surface-alt')),
        tokenDecl('border', fluent('border', 'action-subtle')),
        tokenDecl('border-radius', fluent('radius', 'sm')),
        tokenDecl('padding', fluent('space', 'surface-pad-sm')),
        tokenDecl('box-shadow', fluent('shadow', 'surface-alt')),
      ];
    case 'banner:':
      return withTransition(
        [
          decl('display', 'flex'),
          decl('align-items', 'center'),
          decl('justify-content', 'space-between'),
          tokenDecl('color', fluent('color', 'text')),
          tokenDecl('background-color', fluent('color', 'surface-alt')),
          tokenDecl('border', fluent('border', 'panel')),
          tokenDecl('border-radius', fluent('radius', 'md')),
          tokenDecl('padding', fluent('space', 'surface-pad-sm')),
        ],
        fluent('effect', 'interactive-transition'),
        fluent('motion', 'duration'),
        fluent('motion', 'easing'),
      );
    case 'toast:':
      return withTransition(
        [
          decl('display', 'flex'),
          decl('align-items', 'center'),
          decl('justify-content', 'space-between'),
          tokenDecl('color', fluent('color', 'text')),
          tokenDecl('background-color', fluent('color', 'surface-alt')),
          tokenDecl('border', fluent('border', 'action-subtle')),
          tokenDecl('border-radius', fluent('radius', 'md')),
          tokenDecl('padding', fluent('space', 'surface-pad-sm')),
          tokenDecl('box-shadow', fluent('shadow', 'surface-alt')),
        ],
        fluent('effect', 'interactive-transition'),
        fluent('motion', 'duration'),
        fluent('motion', 'easing'),
      );
    case 'progress:':
      return withTransition(
        [
          decl('display', 'block'),
          decl('inline-size', '100%'),
          decl('block-size', '0.5rem'),
          tokenDecl('background-color', fluent('color', 'surface-alt')),
          decl('border-radius', '999px'),
          tokenDecl('background-image', fluent('effect', 'surface-alt-tint')),
          tokenDecl('border', fluent('border', 'action-subtle')),
        ],
        fluent('effect', 'interactive-transition'),
        fluent('motion', 'duration'),
        fluent('motion', 'easing'),
      );
    case 'spinner:':
      return withTransition(
        [
          decl('display', 'inline-block'),
          decl('inline-size', '1.25rem'),
          decl('block-size', '1.25rem'),
          decl('border-width', '2px'),
          decl('border-style', 'solid'),
          tokenDecl('border-color', fluent('color', 'border')),
          tokenDecl('border-top-color', fluent('color', 'primary')),
          decl('border-radius', '999px'),
          decl('animation', 'motif-spin 0.9s linear infinite'),
        ],
        fluent('effect', 'transition'),
        fluent('motion', 'duration'),
        fluent('motion', 'easing'),
      );
    case 'skeleton:':
      return withTransition(
        [
          decl('display', 'block'),
          decl('min-height', '1rem'),
          tokenDecl('background-color', fluent('color', 'surface-alt')),
          decl(
            'background-image',
            'linear-gradient(90deg, rgba(255, 255, 255, 0.06), rgba(255, 255, 255, 0.4), rgba(255, 255, 255, 0.08))',
          ),
          decl('background-size', '200% 100%'),
          tokenDecl('border-radius', fluent('radius', 'sm')),
          decl('animation', 'motif-shimmer 1.4s ease-in-out infinite'),
        ],
        fluent('effect', 'transition'),
        fluent('motion', 'duration'),
        fluent('motion', 'easing'),
      );
    case 'empty:state':
      return withTransition(
        [
          decl('display', 'grid'),
          decl('justify-items', 'start'),
          decl('gap', '0.75rem'),
          tokenDecl('color', fluent('color', 'muted')),
          tokenDecl('background-color', fluent('color', 'surface-alt')),
          tokenDecl('border', fluent('border', 'action-subtle')),
          tokenDecl('border-radius', fluent('radius', 'lg')),
          tokenDecl('padding', fluent('space', 'panel-pad')),
          tokenDecl('box-shadow', fluent('shadow', 'surface-alt')),
        ],
        fluent('effect', 'interactive-transition'),
        fluent('motion', 'duration'),
        fluent('motion', 'easing'),
      );
    default:
      return null;
  }
}
