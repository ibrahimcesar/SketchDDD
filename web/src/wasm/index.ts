// SketchDDD WASM module wrapper
export * from './types';

import type {
  ParseResult,
  ValidationResult,
  CodegenResult,
  VizResult,
  CodegenTarget,
  VizFormat,
} from './types';

// Dynamic import for WASM module
let wasmModule: typeof import('./sketchddd_wasm') | null = null;
let initPromise: Promise<void> | null = null;

/**
 * Initialize the WASM module. Must be called before using any other functions.
 * Safe to call multiple times - will only initialize once.
 */
export async function initWasm(): Promise<void> {
  if (wasmModule) return;

  if (!initPromise) {
    initPromise = (async () => {
      const wasm = await import('./sketchddd_wasm');
      await wasm.default();
      wasmModule = wasm;
    })();
  }

  await initPromise;
}

/**
 * Check if WASM module is initialized
 */
export function isWasmInitialized(): boolean {
  return wasmModule !== null;
}

/**
 * Get the WASM module version
 */
export function getVersion(): string {
  if (!wasmModule) throw new Error('WASM module not initialized. Call initWasm() first.');
  return wasmModule.version();
}

/**
 * Parse a SketchDDD source file
 */
export function parse(source: string): ParseResult {
  if (!wasmModule) throw new Error('WASM module not initialized. Call initWasm() first.');
  return wasmModule.parse(source) as ParseResult;
}

/**
 * Validate a SketchDDD source file
 */
export function validateSource(source: string): ValidationResult {
  if (!wasmModule) throw new Error('WASM module not initialized. Call initWasm() first.');
  return wasmModule.validate_source(source) as ValidationResult;
}

/**
 * Generate code from SketchDDD source
 */
export function generateCode(source: string, target: CodegenTarget): CodegenResult {
  if (!wasmModule) throw new Error('WASM module not initialized. Call initWasm() first.');
  return wasmModule.generate_code(source, target) as CodegenResult;
}

/**
 * Generate visualization from SketchDDD source
 */
export function generateViz(source: string, format: VizFormat): VizResult {
  if (!wasmModule) throw new Error('WASM module not initialized. Call initWasm() first.');
  return wasmModule.generate_viz(source, format) as VizResult;
}

/**
 * Format/pretty-print SketchDDD source
 */
export function formatSource(source: string): string {
  if (!wasmModule) throw new Error('WASM module not initialized. Call initWasm() first.');
  return wasmModule.format_source(source) as string;
}

/**
 * Get supported code generation targets
 */
export function getSupportedTargets(): string[] {
  if (!wasmModule) throw new Error('WASM module not initialized. Call initWasm() first.');
  return wasmModule.supported_targets() as string[];
}

/**
 * Get supported visualization formats
 */
export function getSupportedVizFormats(): string[] {
  if (!wasmModule) throw new Error('WASM module not initialized. Call initWasm() first.');
  return wasmModule.supported_viz_formats() as string[];
}
