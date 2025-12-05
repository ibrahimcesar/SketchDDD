// TypeScript types for SketchDDD WASM module

export interface ParseResult {
  success: boolean;
  data?: ParsedModel;
  error?: string;
}

export interface ParsedModel {
  contexts: ContextInfo[];
  context_maps: ContextMapInfo[];
  warnings: WarningInfo[];
}

export interface ContextInfo {
  name: string;
  entities: EntityInfo[];
  value_objects: ValueObjectInfo[];
  aggregates: AggregateInfo[];
  enums: EnumInfo[];
  morphisms: MorphismInfo[];
  objects: string[];
}

export interface EntityInfo {
  name: string;
  fields: FieldInfo[];
}

export interface ValueObjectInfo {
  name: string;
  fields: FieldInfo[];
}

export interface FieldInfo {
  name: string;
  type_name: string;
  optional: boolean;
}

export interface AggregateInfo {
  name: string;
  root?: string;
  contains: string[];
}

export interface EnumInfo {
  name: string;
  variants: VariantInfo[];
}

export interface VariantInfo {
  name: string;
  has_payload: boolean;
}

export interface MorphismInfo {
  name: string;
  source: string;
  target: string;
}

export interface ContextMapInfo {
  name: string;
  source_context: string;
  target_context: string;
  pattern?: string;
  mappings: MappingInfo[];
}

export interface MappingInfo {
  source: string;
  target: string;
}

export interface WarningInfo {
  message: string;
  line?: number;
  column?: number;
}

export interface ValidationResult {
  valid: boolean;
  error_count: number;
  warning_count: number;
  issues: ValidationIssue[];
}

export interface ValidationIssue {
  severity: 'error' | 'warning' | 'hint';
  code: string;
  message: string;
  context?: string;
  line?: number;
  column?: number;
  suggestion?: string;
}

export interface CodegenResult {
  success: boolean;
  code?: string;
  error?: string;
}

export interface VizResult {
  success: boolean;
  output?: string;
  error?: string;
}

export type CodegenTarget =
  | 'rust'
  | 'typescript'
  | 'kotlin'
  | 'python'
  | 'java'
  | 'clojure'
  | 'haskell';

export type VizFormat = 'mermaid' | 'graphviz' | 'dot';
