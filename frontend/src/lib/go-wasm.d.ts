// Minimal TS types for the Rust WASM Go engine (as generated via wasm-bindgen)
export type Player = { Black: object } | { White: object };

export interface MoveResult {
  ok: boolean;
  reason?: string | null;
  board: number[]; // 0=Black,1=White,2=Empty
  turn: Player;
  captures: [number, number];
  ko?: [number, number] | null;
  is_over: boolean;
}

export class Game {
  constructor(size: number);
  size(): number;
  get_board(): number[];
  get_turn(): Player;
  play_move(row: number, col: number): MoveResult;
  pass(): MoveResult;
  resign(): MoveResult;
  undo(): MoveResult;
  redo(): MoveResult;
  get_scores(): [number, number];
  get_capture_counts(): [number, number];
  get_group_and_liberties(row: number, col: number): unknown;
  is_over(): boolean;
  get_outcome(): unknown;
  export_state(): unknown;
  import_state(v: unknown): void;
}

export default function init(input?: RequestInfo | URL | Response | BufferSource | WebAssembly.Module): Promise<unknown>;
