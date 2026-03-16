import fluentTokens from '../../../../../tokens/fluent.json';
import materialTokens from '../../../../../tokens/material.json';
import { tokenDecl } from './declarations.js';
import type { Declaration, TokenGroup, TokenMap } from './types.js';

const registry = {
  fluent: fluentTokens as TokenGroup,
  material: materialTokens as TokenGroup,
};

export function fluentTypography(sizeKey: string, weightKey: string, color?: string): Declaration[] {
  return typography(registry.fluent.typography, sizeKey, weightKey, color);
}

export function materialTypography(sizeKey: string, weightKey: string, color?: string): Declaration[] {
  return typography(registry.material.typography, sizeKey, weightKey, color);
}

function typography(tokens: TokenMap, sizeKey: string, weightKey: string, color?: string): Declaration[] {
  const declarations = [
    tokenDecl('font-family', tokens['font-family']),
    tokenDecl('font-size', tokens[sizeKey]),
    tokenDecl('font-weight', tokens[weightKey]),
    tokenDecl('line-height', tokens['line-height']),
    tokenDecl('letter-spacing', tokens.tracking),
  ];
  if (color) {
    declarations.push(tokenDecl('color', color));
  }
  return declarations;
}

export function fluent(group: string, key: string): string {
  return getToken(registry.fluent, group, key);
}

export function material(group: string, key: string): string {
  return getToken(registry.material, group, key);
}

function getToken(source: TokenGroup, group: string, key: string): string {
  const tokens = source[group];
  if (!tokens || !(key in tokens)) {
    throw new Error(`Missing token ${group}.${key}`);
  }
  return tokens[key]!;
}
