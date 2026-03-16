export type Variant = 'hover' | 'focus' | 'active' | 'dark';
export type Family = 'f' | 'm' | 'ui';
export type TokenMap = Record<string, string>;
export type TokenGroup = Record<string, TokenMap>;

export type ParsedClass = {
  raw: string;
  variants: Variant[];
  family: Family;
  utility: string;
  value: string | null;
};

export type Declaration = {
  property: string;
  value: string;
};

export type RuleMatch = {
  rawClassName: string;
  variants: Variant[];
  declarations: Declaration[];
};
