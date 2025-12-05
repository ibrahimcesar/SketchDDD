/* tslint:disable */
/* eslint-disable */

/**
 * Create a new bounded context.
 */
export function create_context(name: string): any;

/**
 * Format source code (pretty print).
 */
export function format_source(source: string): any;

/**
 * Generate code from a SketchDDD source.
 *
 * Supported targets: rust, typescript, kotlin, python, java, clojure, haskell
 */
export function generate_code(source: string, target: string): any;

/**
 * Generate visualization from a SketchDDD source.
 *
 * Supported formats: mermaid, graphviz (or dot)
 */
export function generate_viz(source: string, format: string): any;

/**
 * Initialize the WASM module.
 */
export function init(): void;

/**
 * Parse a SketchDDD source file and return structured data.
 */
export function parse(source: string): any;

/**
 * Get list of supported code generation targets.
 */
export function supported_targets(): any;

/**
 * Get list of supported visualization formats.
 */
export function supported_viz_formats(): any;

/**
 * Validate a parsed model and return validation issues.
 */
export function validate(model_json: string): any;

/**
 * Validate source directly without pre-parsing.
 */
export function validate_source(source: string): any;

/**
 * Get the version of the WASM module.
 */
export function version(): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly create_context: (a: number, b: number) => any;
  readonly format_source: (a: number, b: number) => any;
  readonly generate_code: (a: number, b: number, c: number, d: number) => any;
  readonly generate_viz: (a: number, b: number, c: number, d: number) => any;
  readonly parse: (a: number, b: number) => any;
  readonly supported_targets: () => any;
  readonly supported_viz_formats: () => any;
  readonly validate: (a: number, b: number) => any;
  readonly validate_source: (a: number, b: number) => any;
  readonly version: () => [number, number];
  readonly init: () => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_externrefs: WebAssembly.Table;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
