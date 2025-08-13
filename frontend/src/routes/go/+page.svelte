<script lang="ts">
  import { onMount } from 'svelte';
  import { loadGoWasm } from '$lib/go';

  const size = 19;
  const cells = Array.from({ length: size * size }, (_, i) => i);
  let currentPlayer: 'black' | 'white' = 'black';
  let board: Array<'black' | 'white' | null> = Array(size * size).fill(null);
  let engine: any = null; // WASM Game instance
  let hoveredCell: number | null = null;

  // Debug/engine scaffolding state
  type PlaceMove = { kind: 'place'; index: number; color: 'black' | 'white' };
  type PassMove = { kind: 'pass'; color: 'black' | 'white' };
  type ResignMove = { kind: 'resign'; color: 'black' | 'white' };
  type Move = PlaceMove | PassMove | ResignMove;
  let pastMoves: Move[] = [];     // committed moves history (for Undo)
  let futureMoves: Move[] = [];   // undone moves (for Redo)
  let lastMove: string = '-';
  let blackScore = 0; // placeholder until engine computes score
  let whiteScore = 0; // placeholder until engine computes score
  let blackCaptures = 0; // placeholder until engine computes captures
  let whiteCaptures = 0; // placeholder until engine computes captures
  let gameOver = false;
  let showGameOverModal = false;
  let winnerText: string = '';

  // Ko placeholder values (to be provided by the engine later)
  let koPoint: number | null = null;
  let koBlockedColor: 'black' | 'white' | null = null;

  // Board annotation toggles
  let showCoords = true;
  let showHoverLiberties = true;
  // Debug panel visibility
  let debugOpen = false;

  // Derived turn number (next move number)
  $: turnNumber = pastMoves.length + 1;
  // Reactive full history for the Moves panel (past + reversed future)
  $: fullHistory = [...pastMoves, ...futureMoves.slice().reverse()];

  // Star points (handicap points) on a 19x19 board
  const starPoints = new Set([
    3 * size + 3,   // (3,3)
    3 * size + 9,   // (3,9)
    3 * size + 15,  // (3,15)
    9 * size + 3,   // (9,3)
    9 * size + 9,   // (9,9)
    9 * size + 15,  // (9,15)
    15 * size + 3,  // (15,3)
    15 * size + 9,  // (15,9)
    15 * size + 15  // (15,15)
  ]);

  async function handleCellClick(i: number) {
  if (!engine || gameOver) return;
  const mover = currentPlayer; // capture player before engine flips turn
    const { row, col } = getGridPosition(i);
    const res = engine.play_move(row, col);
    if (!res.ok) {
      console.warn('Illegal move:', res.reason);
      return;
    }
    applyEngineResult(res);
  pastMoves = [...pastMoves, { kind: 'place', index: i, color: mover }];
    futureMoves = [];
  lastMove = `${capitalize(mover)} ${indexToGCoord(i)}`;
    hoveredCell = null;
  }

  function handleMouseEnter(i: number) {
  if (gameOver) return; hoveredCell = i;
  }

  function handleMouseLeave() {
  hoveredCell = null;
  }

  function getGridPosition(index: number) {
    const row = Math.floor(index / size);
    const col = index % size;
    return { row, col };
  }

  // Convert linear index -> Go coordinate (columns A–T skipping I, rows 19 (top) to 1 (bottom))
  function indexToGCoord(index: number): string {
    const { row, col } = getGridPosition(index);
    // Skip 'I' in column letters
    const baseCode = 'A'.charCodeAt(0);
    const offset = col >= 8 ? 1 : 0; // insert skip after H
    const letter = String.fromCharCode(baseCode + col + offset);
    const number = size - row; // top row is 19
    return `${letter}${number}`;
  }

  function capitalize(s: 'black' | 'white'): 'Black' | 'White' {
    return s === 'black' ? 'Black' : 'White';
  }

  // Debug functions
  function newGame() {
    if (!engine) return;
    engine = new wasm.Game(size);
    syncFromEngine();
    pastMoves = [];
    futureMoves = [];
    lastMove = '-';
    hoveredCell = null;
  gameOver = false;
  showGameOverModal = false;
  winnerText = '';
  }

  function undo() {
  if (!engine || gameOver) return;
    const res = engine.undo();
    applyEngineResult(res);
    if (pastMoves.length > 0) {
  const move = pastMoves[pastMoves.length - 1]!;
  pastMoves = pastMoves.slice(0, -1);
  futureMoves = [...futureMoves, move];
      lastMove = pastMoves.length > 0 ? formatMove(pastMoves[pastMoves.length - 1]) : '-';
    }
    hoveredCell = null;
  }

  function redo() {
  if (!engine || gameOver) return;
    const res = engine.redo();
    applyEngineResult(res);
    if (futureMoves.length > 0) {
  const move = futureMoves[futureMoves.length - 1]!;
  futureMoves = futureMoves.slice(0, -1);
  pastMoves = [...pastMoves, move];
      lastMove = formatMove(move);
    }
    hoveredCell = null;
  }

  function passTurn() {
  if (!engine || gameOver) return;
  const mover = currentPlayer; // player who is passing
    const res = engine.pass();
    applyEngineResult(res);
  const move: PassMove = { kind: 'pass', color: mover };
  pastMoves = [...pastMoves, move];
  futureMoves = [];
    lastMove = formatMove(move);
  }

  function resign() {
  if (!engine || gameOver) return;
    const mover = currentPlayer; // player who is resigning
    const res = engine.resign();
    applyEngineResult(res);
    // Record resign in history for visibility/navigation
    const move: ResignMove = { kind: 'resign', color: mover };
    pastMoves = [...pastMoves, move];
    futureMoves = [];
    lastMove = formatMove(move);
  }

  function formatMove(move: Move): string {
    if (move.kind === 'pass') {
      return `${capitalize(move.color)} Pass`;
    }
    if (move.kind === 'resign') {
      return `${capitalize(move.color)} Resign`;
    }
    return `${capitalize(move.color)} ${indexToGCoord(move.index)}`;
  }

  // fullHistory is reactive (see declaration above)

  function jumpToMove(n: number) {
    if (!engine) return;
    // n is the desired count of applied moves (0..total)
    const total = fullHistory.length;
    if (n < 0 || n > total) return;
    const current = pastMoves.length;
    if (n === current) return;
    if (n < current) {
      const steps = current - n;
      for (let i = 0; i < steps; i++) {
        // Use existing undo logic to keep UI stacks in sync
        const res = engine.undo();
        applyEngineResult(res);
        if (pastMoves.length > 0) {
          const move = pastMoves[pastMoves.length - 1]!;
          pastMoves = pastMoves.slice(0, -1);
          futureMoves = [...futureMoves, move];
        }
      }
      lastMove = n > 0 ? formatMove(pastMoves[pastMoves.length - 1]) : '-';
    } else {
      const steps = n - current;
      for (let i = 0; i < steps; i++) {
        const res = engine.redo();
        applyEngineResult(res);
        if (futureMoves.length > 0) {
          const move = futureMoves[futureMoves.length - 1]!;
          futureMoves = futureMoves.slice(0, -1);
          pastMoves = [...pastMoves, move];
        }
      }
      lastMove = formatMove(pastMoves[pastMoves.length - 1]);
    }
    hoveredCell = null;
  }

  // Group + liberties utilities for hover diagnostics
  function neighbors(index: number): number[] {
    const { row, col } = getGridPosition(index);
    const out: number[] = [];
    if (row > 0) out.push((row - 1) * size + col);
    if (row < size - 1) out.push((row + 1) * size + col);
    if (col > 0) out.push(row * size + (col - 1));
    if (col < size - 1) out.push(row * size + (col + 1));
    return out;
  }

  function getGroupAndLiberties(start: number): { color: 'black' | 'white'; stones: Set<number>; liberties: Set<number> } | null {
  const color = board[start];
  if (color === null || !engine) return null;
  const { row, col } = getGridPosition(start);
  const res = engine.get_group_and_liberties(row, col);
  if (!res) return null;
  const wasmColor = res[0] as number; // 0 black, 1 white
  const stones = new Set<number>((res[1] as [number, number][]).map(([r, c]) => r * size + c));
  const liberties = new Set<number>((res[2] as [number, number][]).map(([r, c]) => r * size + c));
  return { color: wasmColor === 0 ? 'black' : 'white', stones, liberties };
  }

  $: hoveredGroupInfo = hoveredCell !== null && board[hoveredCell] !== null
    ? getGroupAndLiberties(hoveredCell)
    : null;

  // Helpers
  function applyEngineResult(res: any) {
    const arr: number[] = res.board;
    board = arr.map((v) => (v === 0 ? 'black' : v === 1 ? 'white' : null));
    currentPlayer = fromWasmPlayer(res.turn);
    blackCaptures = res.captures?.[0] ?? 0;
    whiteCaptures = res.captures?.[1] ?? 0;
    const [b, w] = engine.get_scores();
    blackScore = b; whiteScore = w;
    // Check game over from engine
    gameOver = !!res.is_over;
    if (gameOver) {
      const outcome = engine.get_outcome();
      winnerText = formatOutcome(outcome);
      showGameOverModal = true;
    }
  }
  function formatOutcome(outcome: any): string {
    if (!outcome) return 'Game Over';
    // Try various serde encodings
    try {
      if (typeof outcome === 'string') {
        const s = outcome.toLowerCase();
        if (s.includes('black')) return 'Black Wins!';
        if (s.includes('white')) return 'White Wins!';
        if (s.includes('draw')) return 'Draw!';
      }
      if (outcome && typeof outcome === 'object') {
        if ('WinnerByResign' in outcome) {
          const p = outcome.WinnerByResign;
          return (p && ('Black' in p || p === 0 || p === 'Black')) ? 'Black Wins!' : 'White Wins!';
        }
        if ('WinnerByPoints' in outcome) {
          const arr = outcome.WinnerByPoints;
          const p = Array.isArray(arr) ? arr[0] : arr;
          if (typeof p === 'number') return p === 0 ? 'Black Wins!' : 'White Wins!';
          if (typeof p === 'string') return p.toLowerCase().includes('black') ? 'Black Wins!' : 'White Wins!';
          if (p && typeof p === 'object') {
            if ('Black' in p || 'black' in p) return 'Black Wins!';
            if ('White' in p || 'white' in p) return 'White Wins!';
          }
          return 'Winner by points';
        }
        if ('Draw' in outcome) return 'Draw!';
      }
    } catch {}
    return 'Game Over';
  }
  function fromWasmPlayer(p: any): 'black' | 'white' {
    // Supports serde unit variants as strings ("Black"/"White"),
    // numeric encodings (0/1), or object forms ({ Black: null })
    if (typeof p === 'number') return p === 0 ? 'black' : 'white';
    if (typeof p === 'string') {
      const s = p.toLowerCase();
      if (s.includes('black')) return 'black';
      if (s.includes('white')) return 'white';
    }
    if (p && typeof p === 'object') {
      if ('Black' in p || 'black' in p) return 'black';
      if ('White' in p || 'white' in p) return 'white';
    }
    return 'black';
  }
  function syncFromEngine() {
    const arr: number[] = engine.get_board();
    board = arr.map((v) => (v === 0 ? 'black' : v === 1 ? 'white' : null));
    currentPlayer = fromWasmPlayer(engine.get_turn());
    const [b, w] = engine.get_scores(); blackScore = b; whiteScore = w;
    const [cb, cw] = engine.get_capture_counts(); blackCaptures = cb; whiteCaptures = cw;
  }

  let wasm: any;
  onMount(async () => {
    wasm = await loadGoWasm();
    engine = new wasm.Game(size);
    syncFromEngine();
  });
</script>

<style>
  /* Remove default focus outline from clickable areas */
  circle:focus {
    outline: none;
  }
</style>

<!-- Page background and layout -->
<div class="min-h-screen w-full relative" style="background-color: #098b5d;">
  <!-- Board centering area; keep perfectly centered regardless of the debug panel -->
  <div class="min-h-screen w-full flex items-center justify-center">
    <!-- Go board container with padding -->
    <div
      class="rounded-lg relative"
      style="background-color: #f2b06d; width: 75vh; height: 75vh; border: 4px solid #bf8a56;"
    >
      <!-- SVG for the grid lines and stones -->
      <svg class="absolute inset-0 w-full h-full" viewBox="0 0 100 100">
        <!-- Horizontal lines -->
        {#each Array(size) as _, i}
          <line
            x1="5"
            y1={5 + (i * 90) / (size - 1)}
            x2="95"
            y2={5 + (i * 90) / (size - 1)}
            stroke="#000"
            stroke-width="0.2"
          />
        {/each}

        <!-- Vertical lines -->
        {#each Array(size) as _, i}
          <line
            x1={5 + (i * 90) / (size - 1)}
            y1="5"
            x2={5 + (i * 90) / (size - 1)}
            y2="95"
            stroke="#000"
            stroke-width="0.2"
          />
        {/each}

        <!-- Star points (handicap points) -->
        {#each cells as cell}
          {#if starPoints.has(cell)}
            {@const pos = getGridPosition(cell)}
            <circle
              cx={5 + (pos.col * 90) / (size - 1)}
              cy={5 + (pos.row * 90) / (size - 1)}
              r="0.8"
              fill="#000"
            />
          {/if}
        {/each}

        <!-- Coordinate labels (toggleable) -->
        {#if showCoords}
          <g style="pointer-events: none;" fill="#000">
            {#each Array(size) as _, c}
              {@const x = 5 + (c * 90) / (size - 1)}
              {@const topY = 3.5}
              {@const botY = 98}
              <text x={x} y={topY} text-anchor="middle" font-size="2.2">{indexToGCoord(c).replace(/\d+$/, '')}</text>
              <text x={x} y={botY} text-anchor="middle" font-size="2.2">{indexToGCoord(c).replace(/\d+$/, '')}</text>
            {/each}
            {#each Array(size) as _, r}
              {@const y = 5 + (r * 90) / (size - 1)}
              {@const leftX = 2.5}
              {@const rightX = 97.5}
              <text x={leftX} y={y} dominant-baseline="middle" text-anchor="middle" font-size="2.2">{size - r}</text>
              <text x={rightX} y={y} dominant-baseline="middle" text-anchor="middle" font-size="2.2">{size - r}</text>
            {/each}
          </g>
        {/if}

        <!-- Hover effect stone -->
        {#if hoveredCell !== null && board[hoveredCell] === null}
          {@const pos = getGridPosition(hoveredCell)}
          <circle
            cx={5 + (pos.col * 90) / (size - 1)}
            cy={5 + (pos.row * 90) / (size - 1)}
            r="2"
            fill={currentPlayer === 'black' ? 'rgba(0, 0, 0, 0.6)' : 'rgba(255, 255, 255, 0.8)'}
            stroke={currentPlayer === 'white' ? 'rgba(0, 0, 0, 0.6)' : 'none'}
            stroke-width="0.1"
            pointer-events="none"
          />
        {/if}

        <!-- Hover group liberties overlay (toggleable) -->
        {#if showHoverLiberties && hoveredGroupInfo}
          {#each Array.from(hoveredGroupInfo.liberties) as lib}
            {@const pos = getGridPosition(lib)}
            <circle
              cx={5 + (pos.col * 90) / (size - 1)}
              cy={5 + (pos.row * 90) / (size - 1)}
              r="0.9"
              fill="rgba(33, 150, 243, 0.9)"
              stroke="#000"
              stroke-width="0.1"
              pointer-events="none"
            />
          {/each}
        {/if}

        <!-- Placed stones -->
        {#each cells as cell}
          {#if board[cell]}
            {@const pos = getGridPosition(cell)}
            <circle
              cx={5 + (pos.col * 90) / (size - 1)}
              cy={5 + (pos.row * 90) / (size - 1)}
              r="2"
              fill={board[cell] === 'black' ? '#000' : '#fff'}
            />
          {/if}
        {/each}

        <!-- Invisible clickable areas for intersections -->
        {#each cells as cell}
          {@const pos = getGridPosition(cell)}
          <circle
            cx={5 + (pos.col * 90) / (size - 1)}
            cy={5 + (pos.row * 90) / (size - 1)}
            r="2.5"
            fill="transparent"
            class="cursor-pointer"
            role="button"
            tabindex="0"
            aria-label="Go board intersection at row {pos.row + 1}, column {pos.col + 1}"
            on:click={() => handleCellClick(cell)}
            on:keydown={(e) => {
              if (e.key === 'Enter' || e.key === ' ') {
                e.preventDefault();
                handleCellClick(cell);
              }
            }}
            on:mouseenter={() => handleMouseEnter(cell)}
            on:mouseleave={handleMouseLeave}
          />
        {/each}
      </svg>
    </div>
  </div>

  <!-- Hamburger toggle button -->
  {#if !debugOpen}
    <button
      class="fixed top-2 right-2 z-50"
      style="width: 44px; height: 44px; background-color: #f8efdc; border: 2px solid #000; box-shadow: 4px 4px 0 #000; display: flex; align-items: center; justify-content: center;"
      aria-label="Open Debug Panel"
      on:click={() => (debugOpen = true)}
    >
      <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="#000" stroke-width="2" stroke-linecap="round">
        <line x1="3" y1="6" x2="21" y2="6" />
        <line x1="3" y1="12" x2="21" y2="12" />
        <line x1="3" y1="18" x2="21" y2="18" />
      </svg>
    </button>
  {/if}

  <!-- Right-side Debug Panel -->
  {#if debugOpen}
  <aside
    class="fixed top-2 right-2 bottom-2 overflow-auto"
    style="width: 25vw; background-color: #f8efdc; border: 2px solid #000; box-shadow: 4px 4px 0 #000;"
  >
    <div class="p-4 text-black space-y-3">
      <div class="flex items-center justify-between">
        <h2 class="text-xl font-bold">Debug Panel</h2>
        <button
          class="inline-flex items-center justify-center"
          style="width: 44px; height: 44px; border: 2px solid #000; background-color: #f8efdc; box-shadow: 4px 4px 0 #000;"
          aria-label="Close Debug Panel"
          on:click={() => (debugOpen = false)}
        >
          <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="#000" stroke-width="2" stroke-linecap="round">
            <line x1="5" y1="5" x2="19" y2="19" />
            <line x1="19" y1="5" x2="5" y2="19" />
          </svg>
        </button>
      </div>
      <div class="text-sm">
        <div><span class="font-semibold">Turn:</span> {capitalize(currentPlayer)}</div>
        <div><span class="font-semibold">Turn #:</span> {turnNumber}</div>
        <div><span class="font-semibold">Last Move:</span> {lastMove}</div>
      </div>

      <h3 class="text-base font-semibold mt-2">Score</h3>
      <div class="text-sm">
        <div>Black: {blackScore}</div>
        <div>White: {whiteScore}</div>
      </div>

      <h3 class="text-base font-semibold mt-2">Captures</h3>
      <div class="text-sm">
        <div>Black captures: {blackCaptures}</div>
        <div>White captures: {whiteCaptures}</div>
      </div>

      <h3 class="text-base font-semibold mt-2">Moves</h3>
      <div class="text-sm max-h-48 overflow-auto border border-black/20 bg-white/50">
        {#each fullHistory as move, i}
          {@const moveNum = i + 1}
          <button
            class="w-full text-left px-2 py-1 hover:bg-gray-100 border-b border-black/10"
            on:click={() => jumpToMove(moveNum)}
          >
            <span class="inline-block w-6">{moveNum}.</span>
            <span class="font-mono">{formatMove(move)}</span>
            {#if i === pastMoves.length - 1}
              <span class="ml-2 text-xs">(current)</span>
            {/if}
          </button>
        {/each}
        {#if fullHistory.length === 0}
          <div class="px-2 py-2 text-gray-600">No moves yet.</div>
        {/if}
      </div>

      <!-- Checkboxes above Inspector -->
      <div class="mt-2 text-sm flex items-center gap-2">
        <label class="inline-flex items-center gap-2 cursor-pointer select-none">
          <input type="checkbox" bind:checked={showCoords} />
          <span>Show Coordinates</span>
        </label>
        <label class="inline-flex items-center gap-2 cursor-pointer select-none">
          <input type="checkbox" bind:checked={showHoverLiberties} />
          <span>Show Hover Liberties</span>
        </label>
      </div>

      <!-- Optional extra diagnostics to aid engine development -->
      <div class="pt-2 border-t border-black/20 text-xs leading-5">
        <div class="font-semibold">Inspector</div>
        <div>Hovered: {hoveredCell !== null ? `${indexToGCoord(hoveredCell)} (#${hoveredCell})` : '-'}</div>
        <div>Hover group: {hoveredGroupInfo ? `${hoveredGroupInfo.stones.size} stones, ${hoveredGroupInfo.liberties.size} liberties` : '-'}</div>
        <div>History len: {pastMoves.length} | Future len: {futureMoves.length}</div>
        <div>Board size: {size}×{size}</div>
        <div>Ko: {koPoint !== null ? `${indexToGCoord(koPoint)} (blocked: ${koBlockedColor ? capitalize(koBlockedColor) : '?'})` : '-'}</div>
      </div>
    </div>
  </aside>
  {/if}

  <!-- Bottom control bar -->
  <div class="fixed left-0 right-0 bottom-0 z-40 flex items-center justify-center gap-3 p-3 pointer-events-none">
    <div class="flex items-center gap-3 pointer-events-auto" style="background-color: transparent;">
      <button
        class="px-3 py-2 bg-white hover:bg-gray-100 active:translate-y-[1px]"
        style="border: 2px solid #000; box-shadow: 2px 2px 0 #000;"
        on:click={newGame}
      >
        New Game
      </button>
      <button
        class="px-3 py-2 bg-white hover:bg-gray-100 active:translate-y-[1px]"
        style="border: 2px solid #000; box-shadow: 2px 2px 0 #000;"
        on:click={passTurn}
        disabled={gameOver}
      >
        Pass
      </button>
      <button
        class="px-3 py-2 bg-white hover:bg-gray-100 active:translate-y-[1px]"
        style="border: 2px solid #000; box-shadow: 2px 2px 0 #000;"
        on:click={resign}
        disabled={gameOver}
      >
        Resign
      </button>
      <button
        class="px-3 py-2 bg-white hover:bg-gray-100 active:translate-y-[1px] disabled:opacity-50 disabled:cursor-not-allowed"
        style="border: 2px solid #000; box-shadow: 2px 2px 0 #000;"
        on:click={undo}
        disabled={pastMoves.length === 0 || gameOver}
      >
        Undo
      </button>
      <button
        class="px-3 py-2 bg-white hover:bg-gray-100 active:translate-y-[1px] disabled:opacity-50 disabled:cursor-not-allowed"
        style="border: 2px solid #000; box-shadow: 2px 2px 0 #000;"
        on:click={redo}
        disabled={futureMoves.length === 0 || gameOver}
      >
        Redo
      </button>
    </div>
  </div>

  {#if showGameOverModal}
    <div class="fixed inset-0 z-50 flex items-center justify-center" style="background: rgba(0,0,0,0.25);">
      <div style="background-color: #f8efdc; border: 2px solid #000; box-shadow: 6px 6px 0 #000; width: auto; max-width: 650px;" class="px-8 py-2">
        <div class="text-2xl font-bold mb-4 text-center">{winnerText}</div>
        <div class="flex items-center gap-4 mb-4">
          <div class="flex-1 text-center">
            <div class="text-sm font-semibold mb-1">Final Score</div>
            <div class="text-xs">Black: {blackScore}</div>
            <div class="text-xs">White: {whiteScore}</div>
          </div>
          <div class="w-px bg-black/30 self-stretch"></div>
          <div class="flex-1 text-center">
            <div class="text-sm font-semibold mb-1">Captures</div>
            <div class="text-xs">Black: {blackCaptures}</div>
            <div class="text-xs">White: {whiteCaptures}</div>
          </div>
        </div>
        <div class="flex justify-center gap-3">
          <button
            class="px-3 py-2 bg-white hover:bg-gray-100 active:translate-y-[1px]"
            style="border: 2px solid #000; box-shadow: 2px 2px 0 #000;"
            on:click={newGame}
          >
            New Game
          </button>
          <button
            class="px-3 py-2 bg-white hover:bg-gray-100 active:translate-y-[1px]"
            style="border: 2px solid #000; box-shadow: 2px 2px 0 #000;"
            on:click={() => (showGameOverModal = false)}
          >
            View Board
          </button>
        </div>
      </div>
    </div>
  {/if}
</div>
