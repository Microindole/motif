import type { Declaration } from './types.js';

export function decl(property: string, value: string): Declaration {
  return { property, value };
}

export function tokenDecl(property: string, value: string): Declaration {
  return { property, value };
}

export function actionLayout(): Declaration[] {
  return [
    decl('display', 'inline-flex'),
    decl('align-items', 'center'),
    decl('justify-content', 'center'),
    decl('min-height', '2.5rem'),
    decl('line-height', '1'),
  ];
}

export function control(minHeight: string, padding: string, fontSize: string): Declaration[] {
  return [
    decl('min-height', minHeight),
    decl('padding', padding),
    decl('font-size', fontSize),
  ];
}

export function text(fontSize: string, lineHeight: string): Declaration[] {
  return [decl('font-size', fontSize), decl('line-height', lineHeight)];
}

export function withTransition(
  declarations: Declaration[],
  property: string,
  duration: string,
  easing: string,
): Declaration[] {
  return [
    ...declarations,
    tokenDecl('transition-property', property),
    tokenDecl('transition-duration', duration),
    tokenDecl('transition-timing-function', easing),
  ];
}
