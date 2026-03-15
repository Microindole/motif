import type { Declaration, ParsedClass } from '../../shared.js';
import { decl, material, materialTypography, tokenDecl, withTransition } from '../../shared.js';

export function resolveMaterialShellLayout(parsed: ParsedClass): Declaration[] | null {
  switch (`${parsed.utility}:${parsed.value ?? ''}`) {
    case 'drawer:':
      return withTransition(
        [
          decl('display', 'flex'),
          decl('flex-direction', 'column'),
          decl('min-width', '16rem'),
          tokenDecl('color', material('color', 'on-surface')),
          tokenDecl('background-color', material('color', 'surface-container')),
          tokenDecl('background-image', material('effect', 'container-tint')),
          tokenDecl('border', material('border', 'surface-container')),
          tokenDecl('border-radius', material('radius', 'lg')),
          decl('padding', '1rem'),
          tokenDecl('box-shadow', material('shadow', 'container-high')),
        ],
        material('effect', 'state-transition'),
        material('motion', 'duration'),
        material('motion', 'easing'),
      );
    case 'segmented:button':
      return withTransition(
        [
          ...materialTypography('label-size', 'label-weight'),
          decl('display', 'inline-flex'),
          decl('align-items', 'center'),
          decl('justify-content', 'center'),
          decl('min-height', '2.5rem'),
          tokenDecl('color', material('color', 'primary')),
          tokenDecl('background-color', material('color', 'surface-container')),
          tokenDecl('border', material('border', 'outlined-action')),
          tokenDecl('border-radius', material('radius', 'pill')),
          tokenDecl('padding', material('space', 'action-pad')),
        ],
        material('effect', 'state-transition'),
        material('motion', 'duration'),
        material('motion', 'easing'),
      );
    case 'search:field':
      return withTransition(
        [
          ...materialTypography('body-size', 'body-weight'),
          decl('min-height', '2.75rem'),
          decl('padding-left', '2.5rem'),
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
    case 'breadcrumb:':
      return withTransition(
        [
          decl('display', 'inline-flex'),
          decl('align-items', 'center'),
          decl('flex-wrap', 'wrap'),
          decl('gap', '0.5rem'),
          tokenDecl('color', material('color', 'muted')),
        ],
        material('effect', 'transition'),
        material('motion', 'duration'),
        material('motion', 'easing'),
      );
    case 'breadcrumb:item':
      return [
        ...materialTypography('label-size', 'label-weight'),
        decl('display', 'inline-flex'),
        decl('align-items', 'center'),
        tokenDecl('color', material('color', 'muted')),
      ];
    case 'avatar:':
      return withTransition(
        [
          decl('display', 'inline-grid'),
          decl('place-items', 'center'),
          decl('inline-size', '2.5rem'),
          decl('block-size', '2.5rem'),
          tokenDecl('color', material('color', 'on-primary-container')),
          tokenDecl('background-color', material('color', 'primary-container')),
          tokenDecl('border-radius', material('radius', 'pill')),
          decl('border', '0'),
        ],
        material('effect', 'state-transition'),
        material('motion', 'duration'),
        material('motion', 'easing'),
      );
    case 'persona:':
      return withTransition(
        [
          decl('display', 'flex'),
          decl('align-items', 'center'),
          decl('gap', '0.75rem'),
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
    case 'sheet:':
      return withTransition(
        [
          tokenDecl('background-color', material('color', 'surface-container')),
          tokenDecl('background-image', material('effect', 'container-tint')),
          tokenDecl('color', material('color', 'on-surface')),
          tokenDecl('border', material('border', 'surface-container')),
          tokenDecl('border-radius', material('radius', 'lg')),
          decl('padding', '1rem'),
          tokenDecl('box-shadow', material('shadow', 'container-high')),
          decl('max-width', '28rem'),
          decl('width', '100%'),
          decl('margin-inline', 'auto'),
        ],
        material('effect', 'state-transition'),
        material('motion', 'duration'),
        material('motion', 'easing'),
      );
    case 'sheet:side':
      return withTransition(
        [
          tokenDecl('background-color', material('color', 'surface-container')),
          tokenDecl('background-image', material('effect', 'container-tint')),
          tokenDecl('color', material('color', 'on-surface')),
          tokenDecl('border', material('border', 'surface-container')),
          tokenDecl('border-radius', material('radius', 'lg')),
          decl('padding', '1rem'),
          tokenDecl('box-shadow', material('shadow', 'container-high')),
          decl('max-width', '28rem'),
          decl('width', '100%'),
          decl('margin-inline', 'auto'),
          decl('min-height', '100%'),
          decl('margin-right', '0'),
          decl('border-top-right-radius', '0'),
          decl('border-bottom-right-radius', '0'),
        ],
        material('effect', 'state-transition'),
        material('motion', 'duration'),
        material('motion', 'easing'),
      );
    case 'sheet:bottom':
      return withTransition(
        [
          tokenDecl('background-color', material('color', 'surface-container')),
          tokenDecl('background-image', material('effect', 'container-tint')),
          tokenDecl('color', material('color', 'on-surface')),
          tokenDecl('border', material('border', 'surface-container')),
          tokenDecl('border-radius', material('radius', 'lg')),
          decl('padding', '1rem'),
          tokenDecl('box-shadow', material('shadow', 'container-high')),
          decl('max-width', '36rem'),
          decl('width', '100%'),
          decl('margin-inline', 'auto'),
          decl('margin-bottom', '0'),
          decl('border-bottom-left-radius', '0'),
          decl('border-bottom-right-radius', '0'),
        ],
        material('effect', 'state-transition'),
        material('motion', 'duration'),
        material('motion', 'easing'),
      );
    default:
      return null;
  }
}
