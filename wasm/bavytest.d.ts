/* tslint:disable */
/* eslint-disable */

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly main: (a: number, b: number) => number;
  readonly wasm_bindgen__convert__closures_____invoke__h889f6638ea836334: (a: number, b: number, c: any) => void;
  readonly wasm_bindgen__closure__destroy__h83df70367b01b370: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__h3371302b86106166: (a: number, b: number, c: any) => void;
  readonly wasm_bindgen__closure__destroy__h47afbde277b138e4: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__h2be07bae1ff30c67: (a: number, b: number, c: any, d: any) => void;
  readonly wasm_bindgen__convert__closures_____invoke__h7b45dc6248e9a8b7: (a: number, b: number, c: any) => void;
  readonly wasm_bindgen__closure__destroy__h64760e5fa4a5aa85: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__hf9e1b1c6c66a348a: (a: number, b: number) => void;
  readonly wasm_bindgen__closure__destroy__h05ca4f2c92ad098e: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__h9f0bc569e08a8d32: (a: number, b: number, c: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__h0d759cd6d42c7300: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_externrefs: WebAssembly.Table;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
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
