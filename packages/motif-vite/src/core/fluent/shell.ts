import type { Declaration, ParsedClass } from '../shared.js';
import {
  actionLayout,
  decl,
  fluent,
  fluentTypography,
  tokenDecl,
  withTransition,
} from '../shared.js';

export function resolveFluentShell(parsed: ParsedClass): Declaration[] | null {
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
    case 'drawer:':
      return withTransition(
        [
          decl('display', 'flex'),
          decl('flex-direction', 'column'),
          decl('min-width', '16rem'),
          tokenDecl('color', fluent('color', 'text')),
          tokenDecl('background-color', fluent('color', 'surface')),
          tokenDecl('background-image', fluent('effect', 'surface-tint')),
          tokenDecl('border', fluent('border', 'panel')),
          tokenDecl('border-radius', fluent('radius', 'lg')),
          tokenDecl('padding', fluent('space', 'panel-pad')),
          tokenDecl('box-shadow', fluent('shadow', 'panel')),
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
    case 'segmented:button':
      return withTransition(
        [
          ...fluentTypography('label-size', 'label-weight'),
          decl('display', 'inline-flex'),
          decl('align-items', 'center'),
          decl('justify-content', 'center'),
          decl('min-height', '2.5rem'),
          tokenDecl('color', fluent('color', 'text')),
          tokenDecl('background-color', fluent('color', 'surface-alt')),
          tokenDecl('border', fluent('border', 'action-subtle')),
          tokenDecl('border-radius', fluent('radius', 'md')),
          tokenDecl('padding', fluent('space', 'action-pad')),
        ],
        fluent('effect', 'interactive-transition'),
        fluent('motion', 'duration'),
        fluent('motion', 'easing'),
      );
    case 'search:field':
      return withTransition(
        [
          ...fluentTypography('body-size', 'body-weight'),
          decl('min-height', '2.75rem'),
          decl('padding-left', '2.5rem'),
          tokenDecl('color', fluent('color', 'text')),
          tokenDecl('caret-color', fluent('color', 'primary')),
          tokenDecl('background-color', fluent('color', 'field')),
          tokenDecl('background-image', fluent('effect', 'field-tint')),
          tokenDecl('border', fluent('border', 'field')),
          tokenDecl('border-radius', fluent('radius', 'sm')),
          tokenDecl('padding', fluent('space', 'field-pad')),
          tokenDecl('box-shadow', fluent('shadow', 'field')),
        ],
        fluent('effect', 'interactive-transition'),
        fluent('motion', 'duration'),
        fluent('motion', 'easing'),
      );
    case 'breadcrumb:':
      return withTransition(
        [
          decl('display', 'inline-flex'),
          decl('align-items', 'center'),
          decl('flex-wrap', 'wrap'),
          decl('gap', '0.5rem'),
          tokenDecl('color', fluent('color', 'muted')),
        ],
        fluent('effect', 'transition'),
        fluent('motion', 'duration'),
        fluent('motion', 'easing'),
      );
    case 'breadcrumb:item':
      return [
        ...fluentTypography('label-size', 'label-weight'),
        decl('display', 'inline-flex'),
        decl('align-items', 'center'),
        tokenDecl('color', fluent('color', 'muted')),
      ];
    case 'avatar:':
      return withTransition(
        [
          decl('display', 'inline-grid'),
          decl('place-items', 'center'),
          decl('inline-size', '2.5rem'),
          decl('block-size', '2.5rem'),
          tokenDecl('color', fluent('color', 'text')),
          tokenDecl('background-color', fluent('color', 'surface-alt')),
          decl('border-radius', '999px'),
          tokenDecl('border', fluent('border', 'action-subtle')),
        ],
        fluent('effect', 'interactive-transition'),
        fluent('motion', 'duration'),
        fluent('motion', 'easing'),
      );
    case 'persona:':
      return withTransition(
        [
          decl('display', 'flex'),
          decl('align-items', 'center'),
          decl('gap', '0.75rem'),
          tokenDecl('color', fluent('color', 'text')),
          tokenDecl('background-color', fluent('color', 'surface-alt')),
          tokenDecl('border', fluent('border', 'action-subtle')),
          tokenDecl('border-radius', fluent('radius', 'md')),
          tokenDecl('padding', fluent('space', 'surface-pad-sm')),
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
    case 'sheet:':
      return withTransition(
        [
          tokenDecl('background-color', fluent('color', 'panel')),
          tokenDecl('background-image', fluent('effect', 'panel-tint')),
          decl('background-blend-mode', 'screen'),
          tokenDecl('color', fluent('color', 'text')),
          tokenDecl('border', fluent('border', 'panel')),
          tokenDecl('border-radius', fluent('radius', 'lg')),
          tokenDecl('padding', fluent('space', 'panel-pad')),
          tokenDecl('box-shadow', fluent('shadow', 'panel')),
          decl('backdrop-filter', 'blur(24px) saturate(1.2)'),
          decl('max-width', '28rem'),
          decl('width', '100%'),
          decl('margin-inline', 'auto'),
        ],
        fluent('effect', 'interactive-transition'),
        fluent('motion', 'duration'),
        fluent('motion', 'easing'),
      );
    case 'sheet:side':
      return withTransition(
        [
          tokenDecl('background-color', fluent('color', 'panel')),
          tokenDecl('background-image', fluent('effect', 'panel-tint')),
          decl('background-blend-mode', 'screen'),
          tokenDecl('color', fluent('color', 'text')),
          tokenDecl('border', fluent('border', 'panel')),
          tokenDecl('border-radius', fluent('radius', 'lg')),
          tokenDecl('padding', fluent('space', 'panel-pad')),
          tokenDecl('box-shadow', fluent('shadow', 'panel')),
          decl('backdrop-filter', 'blur(24px) saturate(1.2)'),
          decl('max-width', '28rem'),
          decl('width', '100%'),
          decl('margin-inline', 'auto'),
          decl('min-height', '100%'),
          decl('margin-right', '0'),
          decl('border-top-right-radius', '0'),
          decl('border-bottom-right-radius', '0'),
        ],
        fluent('effect', 'interactive-transition'),
        fluent('motion', 'duration'),
        fluent('motion', 'easing'),
      );
    case 'sheet:bottom':
      return withTransition(
        [
          tokenDecl('background-color', fluent('color', 'panel')),
          tokenDecl('background-image', fluent('effect', 'panel-tint')),
          decl('background-blend-mode', 'screen'),
          tokenDecl('color', fluent('color', 'text')),
          tokenDecl('border', fluent('border', 'panel')),
          tokenDecl('border-radius', fluent('radius', 'lg')),
          tokenDecl('padding', fluent('space', 'panel-pad')),
          tokenDecl('box-shadow', fluent('shadow', 'panel')),
          decl('backdrop-filter', 'blur(24px) saturate(1.2)'),
          decl('max-width', '36rem'),
          decl('width', '100%'),
          decl('margin-inline', 'auto'),
          decl('margin-bottom', '0'),
          decl('border-bottom-left-radius', '0'),
          decl('border-bottom-right-radius', '0'),
        ],
        fluent('effect', 'interactive-transition'),
        fluent('motion', 'duration'),
        fluent('motion', 'easing'),
      );
    default:
      return null;
  }
}