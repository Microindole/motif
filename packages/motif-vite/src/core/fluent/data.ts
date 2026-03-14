import type { Declaration, ParsedClass } from '../shared.js';
import {
  actionLayout,
  decl,
  fluent,
  fluentTypography,
  tokenDecl,
  withTransition,
} from '../shared.js';

export function resolveFluentData(parsed: ParsedClass): Declaration[] | null {
  switch (`${parsed.utility}:${parsed.value ?? ''}`) {
    case 'table:':
      return withTransition(
        [
          decl('display', 'grid'),
          decl('gap', '0.5rem'),
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
    case 'table:header':
      return [
        ...fluentTypography('label-size', 'label-weight'),
        decl('display', 'grid'),
        decl('grid-template-columns', 'minmax(0, 2fr) minmax(0, 1fr) auto'),
        decl('align-items', 'center'),
        decl('gap', '0.75rem'),
        tokenDecl('color', fluent('color', 'muted')),
        tokenDecl('padding', fluent('space', 'surface-pad-sm')),
      ];
    case 'table:cell':
      return fluentTypography('body-size', 'body-weight', fluent('color', 'text'));
    case 'accordion:header':
      return [
        ...fluentTypography('label-size', 'label-weight'),
        decl('display', 'flex'),
        decl('align-items', 'center'),
        decl('justify-content', 'space-between'),
        tokenDecl('color', fluent('color', 'text')),
      ];
    case 'accordion:content':
      return [
        ...fluentTypography('body-size', 'body-weight'),
        tokenDecl('color', fluent('color', 'muted')),
        decl('display', 'grid'),
        decl('gap', '0.5rem'),
      ];
    case 'accordion:item':
      return withTransition(
        [
          decl('display', 'grid'),
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
    case 'accordion:item-open':
      return withTransition(
        [
          decl('display', 'grid'),
          decl('gap', '0.75rem'),
          tokenDecl('color', fluent('color', 'text')),
          tokenDecl('background-color', fluent('color', 'surface-alt')),
          tokenDecl('background-image', fluent('effect', 'surface-alt-tint')),
          tokenDecl('border', fluent('border', 'action-subtle')),
          tokenDecl('border-radius', fluent('radius', 'md')),
          tokenDecl('padding', fluent('space', 'surface-pad-sm')),
          tokenDecl('box-shadow', fluent('shadow', 'surface-alt')),
        ],
        fluent('effect', 'interactive-transition'),
        fluent('motion', 'duration'),
        fluent('motion', 'easing'),
      );
    case 'table:row':
      return withTransition(
        [
          decl('display', 'grid'),
          decl('grid-template-columns', 'minmax(0, 2fr) minmax(0, 1fr) auto'),
          decl('align-items', 'center'),
          decl('gap', '0.75rem'),
          decl('min-height', '3rem'),
          tokenDecl('color', fluent('color', 'text')),
          tokenDecl('background-color', fluent('color', 'surface-alt')),
          tokenDecl('border', fluent('border', 'action-subtle')),
          tokenDecl('border-radius', fluent('radius', 'sm')),
          tokenDecl('padding', fluent('space', 'surface-pad-sm')),
        ],
        fluent('effect', 'interactive-transition'),
        fluent('motion', 'duration'),
        fluent('motion', 'easing'),
      );
    case 'table:row-selected':
      return withTransition(
        [
          decl('display', 'grid'),
          decl('grid-template-columns', 'minmax(0, 2fr) minmax(0, 1fr) auto'),
          decl('align-items', 'center'),
          decl('gap', '0.75rem'),
          decl('min-height', '3rem'),
          tokenDecl('color', fluent('color', 'text')),
          tokenDecl('background-color', fluent('color', 'surface-alt')),
          tokenDecl('background-image', fluent('effect', 'surface-alt-tint')),
          tokenDecl('border', fluent('border', 'action-subtle')),
          tokenDecl('border-color', fluent('color', 'border-strong')),
          tokenDecl('border-radius', fluent('radius', 'sm')),
          tokenDecl('padding', fluent('space', 'surface-pad-sm')),
        ],
        fluent('effect', 'interactive-transition'),
        fluent('motion', 'duration'),
        fluent('motion', 'easing'),
      );
    case 'action:primary':
      return withTransition(
        [
          ...actionLayout(),
          ...fluentTypography('label-size', 'label-weight'),
          tokenDecl('color', fluent('color', 'action-foreground')),
          tokenDecl('background-color', fluent('color', 'primary')),
          tokenDecl('border', fluent('border', 'action-primary')),
          tokenDecl('border-radius', fluent('radius', 'md')),
          tokenDecl('padding', fluent('space', 'action-pad')),
          tokenDecl('box-shadow', fluent('shadow', 'action')),
        ],
        fluent('effect', 'interactive-transition'),
        fluent('motion', 'duration'),
        fluent('motion', 'easing'),
      );
    case 'action:subtle':
      return withTransition(
        [
          ...actionLayout(),
          ...fluentTypography('label-size', 'label-weight'),
          tokenDecl('color', fluent('color', 'primary')),
          tokenDecl('background-color', fluent('color', 'action-subtle')),
          tokenDecl('border', fluent('border', 'action-subtle')),
          tokenDecl('border-radius', fluent('radius', 'md')),
          tokenDecl('padding', fluent('space', 'action-pad')),
          tokenDecl('box-shadow', fluent('shadow', 'action-subtle')),
          decl('backdrop-filter', 'blur(18px) saturate(1.08)'),
        ],
        fluent('effect', 'interactive-transition'),
        fluent('motion', 'duration'),
        fluent('motion', 'easing'),
      );
    default:
      return null;
  }
}