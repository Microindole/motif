import type { Declaration, ParsedClass } from '../../shared.js';
import { decl, fluent, fluentTypography, tokenDecl, withTransition } from '../../shared.js';

export function resolveFluentShellLayout(parsed: ParsedClass): Declaration[] | null {
  switch (`${parsed.utility}:${parsed.value ?? ''}`) {
    case 'drawer:':
      return withTransition(
        [
          decl('display', 'flex'),
          decl('flex-direction', 'column'),
          decl('gap', '1rem'),
          decl('min-width', '16rem'),
          decl('width', '100%'),
          decl('align-self', 'stretch'),
          decl('overflow', 'hidden'),
          tokenDecl('color', fluent('color', 'text')),
          tokenDecl('background-color', fluent('color', 'panel')),
          tokenDecl('background-image', fluent('effect', 'panel-tint')),
          decl('background-blend-mode', 'screen'),
          tokenDecl('border', fluent('border', 'panel')),
          tokenDecl('border-radius', fluent('radius', 'lg')),
          tokenDecl('padding', fluent('space', 'panel-pad')),
          tokenDecl('box-shadow', fluent('shadow', 'panel')),
          decl('backdrop-filter', 'blur(24px) saturate(1.18)'),
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
          decl('align-self', 'stretch'),
          decl('margin-inline', 'auto'),
          decl('overflow', 'hidden'),
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
