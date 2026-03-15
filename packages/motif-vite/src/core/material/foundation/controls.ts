import type { Declaration, ParsedClass } from '../../shared.js';
import { decl, material, materialTypography, tokenDecl, withTransition } from '../../shared.js';

export function resolveMaterialFoundationControls(parsed: ParsedClass): Declaration[] | null {
  switch (`${parsed.utility}:${parsed.value ?? ''}`) {
    case 'field:':
      return withTransition(
        [
          ...materialTypography('body-size', 'body-weight'),
          decl('min-height', '2.75rem'),
          tokenDecl('color', material('color', 'on-surface')),
          tokenDecl('caret-color', material('color', 'primary')),
          tokenDecl('background-color', material('color', 'field')),
          tokenDecl('background-image', material('effect', 'field-tint')),
          tokenDecl('border', material('border', 'field')),
          tokenDecl('border-radius', material('radius', 'lg')),
          tokenDecl('padding', material('space', 'field-pad')),
          tokenDecl('box-shadow', material('shadow', 'field')),
        ],
        material('effect', 'state-transition'),
        material('motion', 'duration'),
        material('motion', 'easing'),
      );
    case 'checkbox:':
      return withTransition(
        [
          decl('appearance', 'none'),
          decl('display', 'inline-grid'),
          decl('place-items', 'center'),
          decl('inline-size', '1.125rem'),
          decl('block-size', '1.125rem'),
          tokenDecl('background-color', material('color', 'surface')),
          tokenDecl('border', material('border', 'outlined-action')),
          tokenDecl('border-radius', material('radius', 'md')),
          tokenDecl('box-shadow', material('shadow', 'outlined-action')),
        ],
        material('effect', 'state-transition'),
        material('motion', 'duration'),
        material('motion', 'easing'),
      );
    case 'radio:':
      return withTransition(
        [
          decl('appearance', 'none'),
          decl('display', 'inline-grid'),
          decl('place-items', 'center'),
          decl('inline-size', '1.125rem'),
          decl('block-size', '1.125rem'),
          tokenDecl('background-color', material('color', 'surface')),
          tokenDecl('border', material('border', 'outlined-action')),
          tokenDecl('border-radius', material('radius', 'pill')),
          tokenDecl('box-shadow', material('shadow', 'outlined-action')),
        ],
        material('effect', 'state-transition'),
        material('motion', 'duration'),
        material('motion', 'easing'),
      );
    case 'switch:':
      return withTransition(
        [
          decl('appearance', 'none'),
          decl('display', 'inline-flex'),
          decl('align-items', 'center'),
          decl('inline-size', '2.5rem'),
          decl('block-size', '1.5rem'),
          tokenDecl('background-color', material('color', 'surface-container')),
          tokenDecl('border', material('border', 'outlined-action')),
          tokenDecl('border-radius', material('radius', 'pill')),
          tokenDecl('box-shadow', material('shadow', 'outlined-action')),
        ],
        material('effect', 'state-transition'),
        material('motion', 'duration'),
        material('motion', 'easing'),
      );
    case 'textarea:':
      return withTransition(
        [
          ...materialTypography('body-size', 'body-weight'),
          decl('min-height', '6.5rem'),
          decl('resize', 'vertical'),
          tokenDecl('color', material('color', 'on-surface')),
          tokenDecl('caret-color', material('color', 'primary')),
          tokenDecl('background-color', material('color', 'field')),
          tokenDecl('background-image', material('effect', 'field-tint')),
          tokenDecl('border', material('border', 'field')),
          tokenDecl('border-radius', material('radius', 'lg')),
          tokenDecl('padding', material('space', 'field-pad')),
          tokenDecl('box-shadow', material('shadow', 'field')),
        ],
        material('effect', 'state-transition'),
        material('motion', 'duration'),
        material('motion', 'easing'),
      );
    case 'select:':
      return withTransition(
        [
          ...materialTypography('body-size', 'body-weight'),
          decl('min-height', '2.75rem'),
          decl('appearance', 'none'),
          decl('background-repeat', 'no-repeat'),
          decl('background-position', 'right 0.85rem center'),
          tokenDecl('color', material('color', 'on-surface')),
          tokenDecl('background-color', material('color', 'field')),
          tokenDecl('background-image', material('effect', 'field-tint')),
          tokenDecl('border', material('border', 'field')),
          tokenDecl('border-radius', material('radius', 'lg')),
          tokenDecl('padding', material('space', 'field-pad')),
          tokenDecl('box-shadow', material('shadow', 'field')),
        ],
        material('effect', 'state-transition'),
        material('motion', 'duration'),
        material('motion', 'easing'),
      );
    case 'tab:':
      return withTransition(
        [
          ...materialTypography('label-size', 'label-weight'),
          decl('display', 'inline-flex'),
          decl('align-items', 'center'),
          decl('justify-content', 'center'),
          decl('min-height', '2.5rem'),
          tokenDecl('color', material('color', 'primary')),
          decl('background-color', 'transparent'),
          tokenDecl('border-radius', material('radius', 'pill')),
          tokenDecl('padding', material('space', 'action-pad')),
          tokenDecl('border', material('border', 'outlined-action')),
        ],
        material('effect', 'state-transition'),
        material('motion', 'duration'),
        material('motion', 'easing'),
      );
    case 'dialog:':
      return withTransition(
        [
          tokenDecl('background-color', material('color', 'surface-container')),
          tokenDecl('background-image', material('effect', 'container-tint')),
          tokenDecl('color', material('color', 'on-surface')),
          tokenDecl('border', material('border', 'surface-container')),
          tokenDecl('border-radius', material('radius', 'lg')),
          decl('padding', '1rem'),
          tokenDecl('box-shadow', material('shadow', 'container-high')),
          decl('max-width', '32rem'),
          decl('width', '100%'),
        ],
        material('effect', 'state-transition'),
        material('motion', 'duration'),
        material('motion', 'easing'),
      );
    case 'list:item':
      return withTransition(
        [
          decl('display', 'flex'),
          decl('align-items', 'center'),
          decl('justify-content', 'space-between'),
          decl('min-height', '3rem'),
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
    case 'menu:item':
      return withTransition(
        [
          decl('display', 'flex'),
          decl('align-items', 'center'),
          decl('justify-content', 'space-between'),
          decl('min-height', '2.5rem'),
          decl('cursor', 'pointer'),
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
    case 'icon:button':
      return withTransition(
        [
          decl('display', 'inline-grid'),
          decl('place-items', 'center'),
          decl('inline-size', '2.5rem'),
          decl('block-size', '2.5rem'),
          tokenDecl('color', material('color', 'primary')),
          tokenDecl('background-color', material('color', 'surface-container')),
          tokenDecl('border', material('border', 'outlined-action')),
          tokenDecl('border-radius', material('radius', 'pill')),
          tokenDecl('box-shadow', material('shadow', 'outlined-action')),
        ],
        material('effect', 'state-transition'),
        material('motion', 'duration'),
        material('motion', 'easing'),
      );
    case 'nav:item':
      return withTransition(
        [
          ...materialTypography('label-size', 'label-weight'),
          decl('display', 'inline-flex'),
          decl('align-items', 'center'),
          decl('min-height', '2.5rem'),
          tokenDecl('color', material('color', 'primary')),
          decl('background-color', 'transparent'),
          tokenDecl('border-radius', material('radius', 'pill')),
          tokenDecl('padding', material('space', 'action-pad')),
        ],
        material('effect', 'state-transition'),
        material('motion', 'duration'),
        material('motion', 'easing'),
      );
    default:
      return null;
  }
}
