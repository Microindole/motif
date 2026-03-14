import type { Declaration, ParsedClass } from '../shared.js';
import {
  actionLayout,
  decl,
  fluent,
  fluentTypography,
  tokenDecl,
  withTransition,
} from '../shared.js';

export function resolveFluentFoundation(parsed: ParsedClass): Declaration[] | null {
  switch (`${parsed.utility}:${parsed.value ?? ''}`) {
    case 'stack:':
      return [decl('display', 'flex'), decl('flex-direction', 'column'), tokenDecl('gap', fluent('space', 'md'))];
    case 'ring:':
      return withTransition(
        [
          tokenDecl('outline-width', fluent('outline', 'focus-width')),
          decl('outline-style', 'solid'),
          tokenDecl('outline-color', fluent('color', 'primary')),
          tokenDecl('outline-offset', fluent('space', 'focus-offset')),
        ],
        fluent('effect', 'transition'),
        fluent('motion', 'duration'),
        fluent('motion', 'easing'),
      );
    case 'bg:primary':
      return [
        tokenDecl('background-color', fluent('color', 'primary')),
        tokenDecl('color', fluent('color', 'action-foreground')),
      ];
    case 'bg:hover-primary':
      return [
        tokenDecl('background-color', fluent('color', 'hover-primary')),
        tokenDecl('color', fluent('color', 'action-foreground')),
      ];
    case 'bg:hover-subtle':
      return [tokenDecl('background-color', fluent('color', 'hover-subtle'))];
    case 'text:primary':
      return [tokenDecl('color', fluent('color', 'primary'))];
    case 'text:muted':
      return [tokenDecl('color', fluent('color', 'muted'))];
    case 'surface:':
      return withTransition(
        [
          tokenDecl('background-color', fluent('color', 'surface')),
          tokenDecl('background-image', fluent('effect', 'surface-tint')),
          decl('background-blend-mode', 'screen'),
          tokenDecl('color', fluent('color', 'text')),
          tokenDecl('border-radius', fluent('radius', 'md')),
          tokenDecl('padding', fluent('space', 'surface-pad')),
          tokenDecl('border-width', fluent('border', 'subtle-width')),
          decl('border-style', 'solid'),
          tokenDecl('border-color', fluent('color', 'border')),
          tokenDecl('box-shadow', fluent('shadow', 'surface')),
          decl('backdrop-filter', 'blur(18px) saturate(1.15)'),
        ],
        fluent('effect', 'transition'),
        fluent('motion', 'duration'),
        fluent('motion', 'easing'),
      );
    case 'surface:alt':
      return withTransition(
        [
          tokenDecl('background-color', fluent('color', 'surface-alt')),
          tokenDecl('background-image', fluent('effect', 'surface-alt-tint')),
          decl('background-blend-mode', 'screen'),
          tokenDecl('color', fluent('color', 'text')),
          tokenDecl('border-radius', fluent('radius', 'sm')),
          tokenDecl('padding', fluent('space', 'surface-pad-sm')),
          tokenDecl('border-width', fluent('border', 'subtle-width')),
          decl('border-style', 'solid'),
          tokenDecl('border-color', fluent('color', 'border-strong')),
          tokenDecl('box-shadow', fluent('shadow', 'surface-alt')),
          decl('backdrop-filter', 'blur(28px) saturate(1.25)'),
        ],
        fluent('effect', 'transition'),
        fluent('motion', 'duration'),
        fluent('motion', 'easing'),
      );
    case 'panel:':
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
        ],
        fluent('effect', 'interactive-transition'),
        fluent('motion', 'duration'),
        fluent('motion', 'easing'),
      );
    case 'shadow:press':
      return [tokenDecl('box-shadow', fluent('shadow', 'press'))];
    case 'title:':
      return fluentTypography('title-size', 'title-weight', fluent('color', 'text'));
    case 'body:':
      return fluentTypography('body-size', 'body-weight', fluent('color', 'text'));
    case 'label:':
      return fluentTypography('label-size', 'label-weight', fluent('color', 'muted'));
    case 'border:focus':
      return [tokenDecl('border', fluent('color', 'border-focus'))];
    case 'divider:':
      return [decl('display', 'block'), tokenDecl('border-bottom', fluent('border', 'divider'))];
    case 'field:':
      return withTransition(
        [
          ...fluentTypography('body-size', 'body-weight'),
          decl('min-height', '2.75rem'),
          tokenDecl('color', fluent('color', 'text')),
          tokenDecl('caret-color', fluent('color', 'primary')),
          tokenDecl('background-color', fluent('color', 'field')),
          tokenDecl('background-image', fluent('effect', 'field-tint')),
          tokenDecl('border', fluent('border', 'field')),
          tokenDecl('border-radius', fluent('radius', 'sm')),
          tokenDecl('padding', fluent('space', 'field-pad')),
          tokenDecl('box-shadow', fluent('shadow', 'field')),
          decl('backdrop-filter', 'blur(14px) saturate(1.05)'),
        ],
        fluent('effect', 'interactive-transition'),
        fluent('motion', 'duration'),
        fluent('motion', 'easing'),
      );
    case 'checkbox:':
      return withTransition(
        [
          decl('appearance', 'none'),
          decl('display', 'inline-grid'),
          decl('place-items', 'center'),
          decl('inline-size', '1.125rem'),
          decl('block-size', '1.125rem'),
          tokenDecl('background-color', fluent('color', 'surface-alt')),
          tokenDecl('border', fluent('border', 'field')),
          tokenDecl('border-radius', fluent('radius', 'sm')),
          tokenDecl('box-shadow', fluent('shadow', 'field')),
        ],
        fluent('effect', 'interactive-transition'),
        fluent('motion', 'duration'),
        fluent('motion', 'easing'),
      );
    case 'radio:':
      return withTransition(
        [
          decl('appearance', 'none'),
          decl('display', 'inline-grid'),
          decl('place-items', 'center'),
          decl('inline-size', '1.125rem'),
          decl('block-size', '1.125rem'),
          tokenDecl('background-color', fluent('color', 'surface-alt')),
          tokenDecl('border', fluent('border', 'field')),
          decl('border-radius', '999px'),
          tokenDecl('box-shadow', fluent('shadow', 'field')),
        ],
        fluent('effect', 'interactive-transition'),
        fluent('motion', 'duration'),
        fluent('motion', 'easing'),
      );
    case 'switch:':
      return withTransition(
        [
          decl('appearance', 'none'),
          decl('display', 'inline-flex'),
          decl('align-items', 'center'),
          decl('inline-size', '2.5rem'),
          decl('block-size', '1.5rem'),
          tokenDecl('background-color', fluent('color', 'surface-alt')),
          tokenDecl('border', fluent('border', 'field')),
          decl('border-radius', '999px'),
          tokenDecl('box-shadow', fluent('shadow', 'field')),
        ],
        fluent('effect', 'interactive-transition'),
        fluent('motion', 'duration'),
        fluent('motion', 'easing'),
      );
    case 'textarea:':
      return withTransition(
        [
          ...fluentTypography('body-size', 'body-weight'),
          decl('min-height', '6.5rem'),
          decl('resize', 'vertical'),
          tokenDecl('color', fluent('color', 'text')),
          tokenDecl('caret-color', fluent('color', 'primary')),
          tokenDecl('background-color', fluent('color', 'field')),
          tokenDecl('background-image', fluent('effect', 'field-tint')),
          tokenDecl('border', fluent('border', 'field')),
          tokenDecl('border-radius', fluent('radius', 'sm')),
          tokenDecl('padding', fluent('space', 'field-pad')),
          tokenDecl('box-shadow', fluent('shadow', 'field')),
          decl('backdrop-filter', 'blur(14px) saturate(1.05)'),
        ],
        fluent('effect', 'interactive-transition'),
        fluent('motion', 'duration'),
        fluent('motion', 'easing'),
      );
    case 'select:':
      return withTransition(
        [
          ...fluentTypography('body-size', 'body-weight'),
          decl('min-height', '2.75rem'),
          decl('appearance', 'none'),
          decl('background-repeat', 'no-repeat'),
          decl('background-position', 'right 0.85rem center'),
          tokenDecl('color', fluent('color', 'text')),
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
    case 'tab:':
      return withTransition(
        [
          ...fluentTypography('label-size', 'label-weight'),
          decl('display', 'inline-flex'),
          decl('align-items', 'center'),
          decl('justify-content', 'center'),
          decl('min-height', '2.5rem'),
          tokenDecl('color', fluent('color', 'muted')),
          decl('background-color', 'transparent'),
          tokenDecl('border-radius', fluent('radius', 'md')),
          tokenDecl('padding', fluent('space', 'action-pad')),
          tokenDecl('border', fluent('border', 'action-subtle')),
        ],
        fluent('effect', 'interactive-transition'),
        fluent('motion', 'duration'),
        fluent('motion', 'easing'),
      );
    case 'dialog:':
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
          decl('max-width', '32rem'),
          decl('width', '100%'),
        ],
        fluent('effect', 'interactive-transition'),
        fluent('motion', 'duration'),
        fluent('motion', 'easing'),
      );
    case 'list:item':
      return withTransition(
        [
          decl('display', 'flex'),
          decl('align-items', 'center'),
          decl('justify-content', 'space-between'),
          decl('min-height', '3rem'),
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
    case 'menu:item':
      return withTransition(
        [
          decl('display', 'flex'),
          decl('align-items', 'center'),
          decl('justify-content', 'space-between'),
          decl('min-height', '2.5rem'),
          decl('cursor', 'pointer'),
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
    case 'icon:button':
      return withTransition(
        [
          decl('display', 'inline-grid'),
          decl('place-items', 'center'),
          decl('inline-size', '2.5rem'),
          decl('block-size', '2.5rem'),
          tokenDecl('color', fluent('color', 'primary')),
          tokenDecl('background-color', fluent('color', 'action-subtle')),
          tokenDecl('border', fluent('border', 'action-subtle')),
          tokenDecl('border-radius', fluent('radius', 'md')),
          tokenDecl('box-shadow', fluent('shadow', 'action-subtle')),
        ],
        fluent('effect', 'interactive-transition'),
        fluent('motion', 'duration'),
        fluent('motion', 'easing'),
      );
    case 'nav:item':
      return withTransition(
        [
          ...fluentTypography('label-size', 'label-weight'),
          decl('display', 'inline-flex'),
          decl('align-items', 'center'),
          decl('min-height', '2.5rem'),
          tokenDecl('color', fluent('color', 'muted')),
          decl('background-color', 'transparent'),
          tokenDecl('border-radius', fluent('radius', 'md')),
          tokenDecl('padding', fluent('space', 'action-pad')),
        ],
        fluent('effect', 'interactive-transition'),
        fluent('motion', 'duration'),
        fluent('motion', 'easing'),
      );
    default:
      return null;
  }
}