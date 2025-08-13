// Thin wrapper to load the WASM module and expose a friendly API
// At build time we'll compile /go-engine to a pkg and copy into /src/lib/go-wasm

export type MoveResult = {
  ok: boolean;
  reason?: string | null;
  board: number[];
  turn: unknown;
  captures?: [number, number];
  ko?: [number, number] | null;
  is_over: boolean;
};

export interface WasmGame {
  size(): number;
  get_board(): number[];
  get_turn(): unknown;
  play_move(row: number, col: number): MoveResult;
  pass(): MoveResult;
  resign(): MoveResult;
  undo(): MoveResult;
  redo(): MoveResult;
  get_scores(): [number, number];
  get_group_and_liberties(row: number, col: number): unknown;
  get_capture_counts(): [number, number];
}

type WasmModule = {
  default: (input?: unknown) => Promise<unknown>;
  Game: new (size: number) => WasmGame;
};

let wasmInit: (() => Promise<WasmModule>) | null = null;
let wasmMod: WasmModule | null = null;

export async function loadGoWasm(): Promise<WasmModule> {
  if (wasmMod) return wasmMod;
  if (!wasmInit) {
    // dynamic import of generated pkg; path after build/copy
    wasmInit = async () => {
      // During dev before build, this path may not exist; run `npm run go:wasm` first
      const mod = (await import('./go-wasm/go_engine.js')) as unknown as WasmModule;
      // wasm-bindgen's init default accepts various inputs; we call with none
      await mod.default();
      return mod;
    };
  }
  wasmMod = await wasmInit();
  return wasmMod;
}
