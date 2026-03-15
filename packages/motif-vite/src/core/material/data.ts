import type { Declaration, ParsedClass } from '../shared.js';
import {
  actionLayout,
  decl,
  material,
  materialTypography,
  tokenDecl,
  withTransition,
} from '../shared.js';

export function resolveMaterialData(parsed: ParsedClass): Declaration[] | null {
  switch (`${parsed.utility}:${parsed.value ?? ''}`) {
    case 'table:':
      return withTransition(
        [
          decl('display', 'grid'),
          decl('gap', '0.5rem'),
          tokenDecl('color', material('color', 'on-surface')),
          tokenDecl('background-color', material('color', 'surface-container')),
          tokenDecl('border', material('border', 'surface-container')),
          tokenDecl('border-radius', material('radius', 'lg')),
          decl('padding', '0.85rem 1rem'),
        ],
        material('effect', 'state-transition'),
        material('motion', 'duration'),
        material('motion', 'easing'),
      );
    case 'table:header':
      return [
        ...materialTypography('label-size', 'label-weight'),
        decl('display', 'grid'),
        decl('grid-template-columns', 'minmax(0, 2fr) minmax(0, 1fr) auto'),
        decl('align-items', 'center'),
        decl('gap', '0.75rem'),
        tokenDecl('color', material('color', 'muted')),
        decl('padding', '0.85rem 1rem'),
      ];
    case 'table:cell':
      return materialTypography('body-size', 'body-weight', material('color', 'on-surface'));
    case 'accordion:header':
      return [
        ...materialTypography('label-size', 'label-weight'),
        decl('display', 'flex'),
        decl('align-items', 'center'),
        decl('justify-content', 'space-between'),
        tokenDecl('color', material('color', 'on-surface')),
      ];
    case 'accordion:content':
      return [
        ...materialTypography('body-size', 'body-weight'),
        tokenDecl('color', material('color', 'muted')),
        decl('display', 'grid'),
        decl('gap', '0.5rem'),
      ];
    case 'accordion:item':
      return withTransition(
        [
          decl('display', 'grid'),
          decl('gap', '0.75rem'),
          tokenDecl('color', material('color', 'on-surface')),
          tokenDecl('background-color', material('color', 'surface-container')),
          tokenDecl('border', material('border', 'surface-container')),
          tokenDecl('border-radius', material('radius', 'lg')),
          decl('padding', '1rem'),
        ],
        material('effect', 'state-transition'),
        material('motion', 'duration'),
        material('motion', 'easing'),
      );
    case 'accordion:item-open':
      return withTransition(
        [
          decl('display', 'grid'),
          decl('gap', '0.75rem'),
          tokenDecl('color', material('color', 'on-surface')),
          tokenDecl('background-color', material('color', 'surface-container')),
          tokenDecl('background-image', material('effect', 'container-tint')),
          tokenDecl('border', material('border', 'surface-container')),
          tokenDecl('border-radius', material('radius', 'lg')),
          decl('padding', '1rem'),
          tokenDecl('box-shadow', material('shadow', 'container')),
        ],
        material('effect', 'state-transition'),
        material('motion', 'duration'),
        material('motion', 'easing'),
      );
    case 'table:row':
      return withTransition(
        [
          decl('display', 'grid'),
          decl('grid-template-columns', 'minmax(0, 2fr) minmax(0, 1fr) auto'),
          decl('align-items', 'center'),
          decl('gap', '0.75rem'),
          decl('min-height', '3rem'),
          tokenDecl('color', material('color', 'on-surface')),
          tokenDecl('background-color', material('color', 'surface-container')),
          tokenDecl('border', material('border', 'surface-container')),
          tokenDecl('border-radius', material('radius', 'md')),
          decl('padding', '0.85rem 1rem'),
        ],
        material('effect', 'state-transition'),
        material('motion', 'duration'),
        material('motion', 'easing'),
      );
    case 'table:row-selected':
      return withTransition(
        [
          decl('display', 'grid'),
          decl('grid-template-columns', 'minmax(0, 2fr) minmax(0, 1fr) auto'),
          decl('align-items', 'center'),
          decl('gap', '0.75rem'),
          decl('min-height', '3rem'),
          tokenDecl('color', material('color', 'on-surface')),
          tokenDecl('background-color', material('color', 'surface-container')),
          tokenDecl('background-image', material('effect', 'container-tint')),
          tokenDecl('border', material('border', 'surface-container')),
          tokenDecl('border-color', material('color', 'primary')),
          tokenDecl('border-radius', material('radius', 'md')),
          decl('padding', '0.85rem 1rem'),
        ],
        material('effect', 'state-transition'),
        material('motion', 'duration'),
        material('motion', 'easing'),
      );
    case 'action:primary':
      return withTransition(
        [
          ...actionLayout(),
          ...materialTypography('label-size', 'label-weight'),
          tokenDecl('color', material('color', 'on-primary')),
          tokenDecl('background-color', material('color', 'primary')),
          decl('border', '0'),
          tokenDecl('border-radius', material('radius', 'pill')),
          tokenDecl('padding', material('space', 'action-pad')),
          tokenDecl('box-shadow', material('shadow', 'action')),
        ],
        material('effect', 'state-transition'),
        material('motion', 'duration'),
        material('motion', 'easing'),
      );
    case 'action:tonal':
      return withTransition(
        [
          ...actionLayout(),
          ...materialTypography('label-size', 'label-weight'),
          tokenDecl('color', material('color', 'on-primary-container')),
          tokenDecl('background-color', material('color', 'primary-container')),
          decl('border', '0'),
          tokenDecl('border-radius', material('radius', 'lg')),
          tokenDecl('padding', material('space', 'action-pad')),
          tokenDecl('box-shadow', material('shadow', 'tonal-action')),
        ],
        material('effect', 'state-transition'),
        material('motion', 'duration'),
        material('motion', 'easing'),
      );
    case 'action:outlined':
      return withTransition(
        [
          ...actionLayout(),
          ...materialTypography('label-size', 'label-weight'),
          tokenDecl('color', material('color', 'primary')),
          tokenDecl('background-color', material('color', 'surface')),
          tokenDecl('border', material('border', 'outlined-action')),
          tokenDecl('border-radius', material('radius', 'pill')),
          tokenDecl('padding', material('space', 'action-pad')),
          tokenDecl('box-shadow', material('shadow', 'outlined-action')),
        ],
        material('effect', 'state-transition'),
        material('motion', 'duration'),
        material('motion', 'easing'),
      );
    case 'elevation:1':
      return [tokenDecl('box-shadow', material('shadow', '1'))];
    case 'shadow:2':
      return [tokenDecl('box-shadow', material('shadow', '2'))];
    case 'shadow:press':
      return [tokenDecl('box-shadow', material('shadow', 'press'))];
    default:
      return null;
  }
}