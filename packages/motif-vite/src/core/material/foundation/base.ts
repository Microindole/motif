import type { Declaration, ParsedClass } from '../../shared.js';
import { decl, material, materialTypography, tokenDecl, withTransition } from '../../shared.js';

export function resolveMaterialFoundationBase(parsed: ParsedClass): Declaration[] | null {
  switch (`${parsed.utility}:${parsed.value ?? ''}`) {
    case 'surface:':
      return withTransition(
        [
          tokenDecl('background-color', material('color', 'surface')),
          tokenDecl('background-image', material('effect', 'surface-tint')),
          tokenDecl('color', material('color', 'on-surface')),
          tokenDecl('border-radius', material('radius', 'lg')),
          decl('padding', '1.25rem'),
          tokenDecl('box-shadow', material('shadow', 'surface')),
        ],
        material('effect', 'transition'),
        material('motion', 'duration'),
        material('motion', 'easing'),
      );
    case 'surface:container':
      return withTransition(
        [
          tokenDecl('background-color', material('color', 'surface-container')),
          tokenDecl('background-image', material('effect', 'container-tint')),
          tokenDecl('color', material('color', 'on-surface')),
          tokenDecl('border', material('border', 'surface-container')),
          tokenDecl('border-radius', material('radius', 'lg')),
          decl('padding', '1rem'),
          tokenDecl('box-shadow', material('shadow', 'container-high')),
        ],
        material('effect', 'state-transition'),
        material('motion', 'duration'),
        material('motion', 'easing'),
      );
    case 'surface:variant':
      return withTransition(
        [
          tokenDecl('background-color', material('color', 'surface-variant')),
          tokenDecl('background-image', material('effect', 'variant-tint')),
          tokenDecl('color', material('color', 'on-surface')),
          tokenDecl('border-radius', material('radius', 'lg')),
          decl('padding', '1rem'),
          tokenDecl('box-shadow', material('shadow', 'container')),
        ],
        material('effect', 'transition'),
        material('motion', 'duration'),
        material('motion', 'easing'),
      );
    case 'bg:primary':
      return withTransition(
        [
          tokenDecl('background-color', material('color', 'primary')),
          tokenDecl('color', material('color', 'on-primary')),
          tokenDecl('border-radius', material('radius', 'pill')),
        ],
        material('effect', 'state-transition'),
        material('motion', 'duration'),
        material('motion', 'easing'),
      );
    case 'bg:hover-primary':
      return [tokenDecl('background-color', material('color', 'hover-primary'))];
    case 'bg:hover-container':
      return [tokenDecl('background-color', material('color', 'hover-container'))];
    case 'bg:hover-surface':
      return [tokenDecl('background-color', material('color', 'hover-surface'))];
    case 'bg:primary-container':
      return withTransition(
        [
          tokenDecl('background-color', material('color', 'primary-container')),
          tokenDecl('color', material('color', 'on-primary-container')),
          tokenDecl('border-radius', material('radius', 'lg')),
          tokenDecl('box-shadow', material('shadow', 'container')),
        ],
        material('effect', 'state-transition'),
        material('motion', 'duration'),
        material('motion', 'easing'),
      );
    case 'text:primary':
      return [tokenDecl('color', material('color', 'primary'))];
    case 'text:on-primary':
      return [tokenDecl('color', material('color', 'on-primary'))];
    case 'text:muted':
      return [tokenDecl('color', material('color', 'muted'))];
    case 'ring:':
      return withTransition(
        [
          decl('outline-width', '2px'),
          decl('outline-style', 'solid'),
          tokenDecl('outline-color', material('color', 'primary')),
          decl('outline-offset', '2px'),
          decl('box-shadow', '0 0 0 4px rgba(26, 115, 232, 0.16)'),
        ],
        material('effect', 'state-transition'),
        material('motion', 'duration'),
        material('motion', 'easing'),
      );
    case 'title:':
      return materialTypography('title-size', 'title-weight', material('color', 'on-surface'));
    case 'body:':
      return materialTypography('body-size', 'body-weight', material('color', 'on-surface'));
    case 'label:':
      return materialTypography('label-size', 'label-weight', material('color', 'muted'));
    case 'divider:':
      return [decl('display', 'block'), tokenDecl('border-bottom', material('border', 'divider'))];
    case 'border:focus':
      return [tokenDecl('border', material('border', 'focus'))];
    default:
      return null;
  }
}
