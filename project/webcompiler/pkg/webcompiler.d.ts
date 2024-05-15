/* tslint:disable */
/* eslint-disable */
/**
* @param {string} code
* @returns {CompileResult}
*/
export function compile(code: string): CompileResult;
/**
* @param {string} code
* @returns {string}
*/
export function lexer(code: string): string;
/**
*/
export class CompileResult {
  free(): void;
/**
*/
  assembly: string;
/**
*/
  ast: string;
/**
*/
  optimized_tac: string;
/**
*/
  tac: string;
/**
*/
  tokens: string;
/**
*/
  turing_program: string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_compileresult_free: (a: number) => void;
  readonly __wbg_get_compileresult_tokens: (a: number, b: number) => void;
  readonly __wbg_set_compileresult_tokens: (a: number, b: number, c: number) => void;
  readonly __wbg_get_compileresult_ast: (a: number, b: number) => void;
  readonly __wbg_set_compileresult_ast: (a: number, b: number, c: number) => void;
  readonly __wbg_get_compileresult_tac: (a: number, b: number) => void;
  readonly __wbg_set_compileresult_tac: (a: number, b: number, c: number) => void;
  readonly __wbg_get_compileresult_optimized_tac: (a: number, b: number) => void;
  readonly __wbg_set_compileresult_optimized_tac: (a: number, b: number, c: number) => void;
  readonly __wbg_get_compileresult_assembly: (a: number, b: number) => void;
  readonly __wbg_set_compileresult_assembly: (a: number, b: number, c: number) => void;
  readonly __wbg_get_compileresult_turing_program: (a: number, b: number) => void;
  readonly __wbg_set_compileresult_turing_program: (a: number, b: number, c: number) => void;
  readonly compile: (a: number, b: number) => number;
  readonly lexer: (a: number, b: number, c: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
