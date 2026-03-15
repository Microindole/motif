import type { Declaration, ParsedClass } from '../../shared.js';
import { decl, fluent, fluentTypography, tokenDecl, withTransition } from '../../shared.js';

export function resolveFluentFoundationBase(parsed: ParsedClass): Declaration[] | null {
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
    default:
      return null;
  }
}
