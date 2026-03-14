import type { Declaration, ParsedClass } from '../shared.js';
import {
  actionLayout,
  decl,
  material,
  materialTypography,
  tokenDecl,
  withTransition,
} from '../shared.js';

export function resolveMaterialShell(parsed: ParsedClass): Declaration[] | null {
  switch (`${parsed.utility}:${parsed.value ?? ''}`) {
    case 'badge:':
      return [
        ...materialTypography('label-size', 'label-weight'),
        decl('display', 'inline-flex'),
        decl('align-items', 'center'),
        decl('min-height', '1.75rem'),
        tokenDecl('color', material('color', 'on-primary-container')),
        tokenDecl('background-color', material('color', 'primary-container')),
        decl('border', '0'),
        tokenDecl('border-radius', material('radius', 'pill')),
        decl('padding', '0.35rem 0.75rem'),
      ];
    case 'chip:':
      return withTransition(
        [
          ...materialTypography('label-size', 'label-weight'),
          decl('display', 'inline-flex'),
          decl('align-items', 'center'),
          decl('gap', '0.5rem'),
          decl('min-height', '2rem'),
          tokenDecl('color', material('color', 'primary')),
          tokenDecl('background-color', material('color', 'surface-container')),
          tokenDecl('border', material('border', 'outlined-action')),
          tokenDecl('border-radius', material('radius', 'pill')),
          decl('padding', '0.55rem 0.85rem'),
        ],
        material('effect', 'state-transition'),
        material('motion', 'duration'),
        material('motion', 'easing'),
      );
    case 'tag:':
      return withTransition(
        [
          ...materialTypography('label-size', 'label-weight'),
          decl('display', 'inline-flex'),
          decl('align-items', 'center'),
          decl('min-height', '1.5rem'),
          tokenDecl('color', material('color', 'primary')),
          decl('background-color', 'transparent'),
          tokenDecl('border', material('border', 'outlined-action')),
          tokenDecl('border-radius', material('radius', 'md')),
          decl('padding', '0.2rem 0.55rem'),
        ],
        material('effect', 'state-transition'),
        material('motion', 'duration'),
        material('motion', 'easing'),
      );
    case 'tooltip:':
      return [
        ...materialTypography('label-size', 'label-weight'),
        tokenDecl('color', material('color', 'on-surface')),
        tokenDecl('background-color', material('color', 'surface-container')),
        tokenDecl('border', material('border', 'surface-container')),
        tokenDecl('border-radius', material('radius', 'lg')),
        decl('padding', '0.5rem 0.75rem'),
        tokenDecl('box-shadow', material('shadow', 'container')),
      ];
    case 'banner:':
      return withTransition(
        [
          decl('display', 'flex'),
          decl('align-items', 'center'),
          decl('justify-content', 'space-between'),
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
    case 'toast:':
      return withTransition(
        [
          decl('display', 'flex'),
          decl('align-items', 'center'),
          decl('justify-content', 'space-between'),
          tokenDecl('color', material('color', 'on-surface')),
          tokenDecl('background-color', material('color', 'surface-container')),
          tokenDecl('border', material('border', 'surface-container')),
          tokenDecl('border-radius', material('radius', 'lg')),
          decl('padding', '0.85rem 1rem'),
          tokenDecl('box-shadow', material('shadow', 'container')),
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
    case 'progress:':
      return withTransition(
        [
          decl('display', 'block'),
          decl('inline-size', '100%'),
          decl('block-size', '0.5rem'),
          tokenDecl('background-color', material('color', 'surface-container')),
          tokenDecl('border-radius', material('radius', 'pill')),
          tokenDecl('background-image', material('effect', 'container-tint')),
          tokenDecl('border', material('border', 'surface-container')),
        ],
        material('effect', 'state-transition'),
        material('motion', 'duration'),
        material('motion', 'easing'),
      );
    case 'spinner:':
      return withTransition(
        [
          decl('display', 'inline-block'),
          decl('inline-size', '1.25rem'),
          decl('block-size', '1.25rem'),
          decl('border-width', '2px'),
          decl('border-style', 'solid'),
          tokenDecl('border-color', material('color', 'surface-variant')),
          tokenDecl('border-top-color', material('color', 'primary')),
          tokenDecl('border-radius', material('radius', 'pill')),
          decl('animation', 'motif-spin 0.9s linear infinite'),
        ],
        material('effect', 'state-transition'),
        material('motion', 'duration'),
        material('motion', 'easing'),
      );
    case 'skeleton:':
      return withTransition(
        [
          decl('display', 'block'),
          decl('min-height', '1rem'),
          tokenDecl('background-color', material('color', 'surface-container')),
          decl(
            'background-image',
            'linear-gradient(90deg, rgba(255, 255, 255, 0.12), rgba(255, 255, 255, 0.56), rgba(211, 227, 253, 0.2))',
          ),
          decl('background-size', '200% 100%'),
          tokenDecl('border-radius', material('radius', 'lg')),
          decl('animation', 'motif-shimmer 1.4s ease-in-out infinite'),
        ],
        material('effect', 'transition'),
        material('motion', 'duration'),
        material('motion', 'easing'),
      );
    case 'empty:state':
      return withTransition(
        [
          decl('display', 'grid'),
          decl('justify-items', 'start'),
          decl('gap', '0.75rem'),
          tokenDecl('color', material('color', 'muted')),
          tokenDecl('background-color', material('color', 'surface-container')),
          tokenDecl('border', material('border', 'surface-container')),
          tokenDecl('border-radius', material('radius', 'lg')),
          decl('padding', '1rem'),
          tokenDecl('box-shadow', material('shadow', 'container')),
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