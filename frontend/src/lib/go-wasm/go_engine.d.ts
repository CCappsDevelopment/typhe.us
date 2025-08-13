/* tslint:disable */
/* eslint-disable */
export class Game {
  free(): void;
  constructor(size: number);
  size(): number;
  get_board(): any;
  get_turn(): any;
  turn_num(): number;
  play_move(row: number, col: number): any;
  pass(): any;
  resign(): any;
  undo(): any;
  redo(): any;
  get_capture_counts(): any;
  get_group_and_liberties(row: number, col: number): any;
  get_scores(): any;
  is_over(): boolean;
  get_outcome(): any;
  export_state(): any;
  import_state(v: any): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_game_free: (a: number, b: number) => void;
  readonly game_new: (a: number) => number;
  readonly game_size: (a: number) => number;
  readonly game_get_board: (a: number) => number;
  readonly game_get_turn: (a: number) => number;
  readonly game_turn_num: (a: number) => number;
  readonly game_play_move: (a: number, b: number, c: number) => number;
  readonly game_pass: (a: number) => number;
  readonly game_resign: (a: number) => number;
  readonly game_undo: (a: number) => number;
  readonly game_redo: (a: number) => number;
  readonly game_get_capture_counts: (a: number) => number;
  readonly game_get_group_and_liberties: (a: number, b: number, c: number) => number;
  readonly game_get_scores: (a: number) => number;
  readonly game_is_over: (a: number) => number;
  readonly game_get_outcome: (a: number) => number;
  readonly game_export_state: (a: number) => number;
  readonly game_import_state: (a: number, b: number, c: number) => void;
  readonly __wbindgen_export_0: (a: number, b: number) => number;
  readonly __wbindgen_export_1: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_2: (a: number) => void;
  readonly __wbindgen_export_3: (a: number, b: number, c: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
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
