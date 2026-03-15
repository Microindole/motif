import type { Declaration, ParsedClass } from '../../shared.js';
import { decl, material, materialTypography, tokenDecl, withTransition } from '../../shared.js';

export function resolveMaterialShellFeedback(parsed: ParsedClass): Declaration[] | null {
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
    default:
      return null;
  }
}
