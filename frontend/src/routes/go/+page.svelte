<script lang="ts">
  const size = 19;
  const cells = Array.from({ length: size * size }, (_, i) => i);
  let currentPlayer: 'black' | 'white' = 'black';
  let board: Array<'black' | 'white' | null> = Array(size * size).fill(null);
  let hoveredCell: number | null = null;

  // Debug/engine scaffolding state
  type Move = { index: number; color: 'black' | 'white' };
  let pastMoves: Move[] = [];     // committed moves history (for Undo)
  let futureMoves: Move[] = [];   // undone moves (for Redo)
  let lastMove: string = '-';
  let blackScore = 0; // placeholder until engine computes score
  let whiteScore = 0; // placeholder until engine computes score

  // Derived turn number (next move number)
  $: turnNumber = pastMoves.length + 1;

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

  function handleCellClick(i: number) {
    // Don't place a stone if there's already one at this intersection
    if (board[i] !== null) {
      return;
    }
    
    // Place a stone of the current player's color
    board[i] = currentPlayer;
    // Record move into history, clear any redo stack
  pastMoves.push({ index: i, color: currentPlayer });
  // trigger reactivity for derived values depending on pastMoves
  pastMoves = pastMoves;
    futureMoves = [];
    lastMove = `${capitalize(currentPlayer)} ${indexToGCoord(i)}`;

    console.log(board);
    // Clear hover state to remove any lingering hover effects
    hoveredCell = null;
    
    // Switch to the other player
    currentPlayer = currentPlayer === 'black' ? 'white' : 'black';
  }

  function handleMouseEnter(i: number) {
    hoveredCell = i;
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
    board = Array(size * size).fill(null);
    currentPlayer = 'black';
    pastMoves = [];
    futureMoves = [];
    lastMove = '-';
    blackScore = 0;
    whiteScore = 0;
    hoveredCell = null; // force re-render
  }

  function undo() {
    if (pastMoves.length === 0) return;
    const move = pastMoves.pop()!;
    board[move.index] = null;
  futureMoves.push(move);
  // trigger reactivity for controls/derived values
  pastMoves = pastMoves;
  futureMoves = futureMoves;
    currentPlayer = move.color; // revert turn back to the player who made the undone move
    lastMove = pastMoves.length > 0
      ? `${capitalize(pastMoves[pastMoves.length - 1].color)} ${indexToGCoord(pastMoves[pastMoves.length - 1].index)}`
      : '-';
    hoveredCell = null; // ensure re-render
  }

  function redo() {
    if (futureMoves.length === 0) return;
    const move = futureMoves.pop()!;
    // Re-apply move
    if (board[move.index] === null) {
      board[move.index] = move.color;
  pastMoves.push(move);
  // trigger reactivity updates
  pastMoves = pastMoves;
  futureMoves = futureMoves;
      lastMove = `${capitalize(move.color)} ${indexToGCoord(move.index)}`;
      currentPlayer = move.color === 'black' ? 'white' : 'black';
      hoveredCell = null; // ensure re-render
    }
  }
</script>

<style>
  /* Remove default focus outline from clickable areas */
  circle:focus {
    outline: none;
  }
</style>

<!-- Page background and layout -->
<div class="min-h-screen w-full relative" style="background-color: #098b5d;">
  <!-- Board centering area; reserve space on the right for the debug panel to avoid overlap -->
  <div class="min-h-screen w-full flex items-center justify-center" style="padding-right: calc(25vw + 16px);">
    <!-- Go board container with padding -->
    <div
      class="rounded-lg relative"
      style="background-color: #f2b06d; width: 75vh; height: 75vh; border: 4px solid #bf8a56;"
    >
      <!-- SVG for the grid lines and stones -->
      <svg 
        class="absolute inset-0 w-full h-full" 
        viewBox="0 0 100 100" 
      >
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

  <!-- Right-side Debug Panel -->
  <aside
    class="fixed top-2 right-2 bottom-2 overflow-auto"
    style="width: 25vw; background-color: #f8efdc; border: 2px solid #000; box-shadow: 4px 4px 0 #000;"
  >
    <div class="p-4 text-black space-y-3">
      <h2 class="text-xl font-bold">Debug Panel</h2>
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

      <h3 class="text-base font-semibold mt-2">Debug Functions</h3>
      <div class="flex flex-col gap-2">
        <button
          class="px-3 py-2 bg-white hover:bg-gray-100 active:translate-y-[1px]"
          style="border: 2px solid #000; box-shadow: 2px 2px 0 #000;"
          on:click={newGame}
        >
          New Game
        </button>
        <button
          class="px-3 py-2 bg-white hover:bg-gray-100 active:translate-y-[1px] disabled:opacity-50 disabled:cursor-not-allowed"
          style="border: 2px solid #000; box-shadow: 2px 2px 0 #000;"
          on:click={undo}
          disabled={pastMoves.length === 0}
        >
          Undo
        </button>
        <button
          class="px-3 py-2 bg-white hover:bg-gray-100 active:translate-y-[1px] disabled:opacity-50 disabled:cursor-not-allowed"
          style="border: 2px solid #000; box-shadow: 2px 2px 0 #000;"
          on:click={redo}
          disabled={futureMoves.length === 0}
        >
          Redo
        </button>
      </div>

      <!-- Optional extra diagnostics to aid engine development -->
      <div class="pt-2 border-t border-black/20 text-xs leading-5">
        <div class="font-semibold">Inspector</div>
  <div>Hovered: {hoveredCell !== null ? `${indexToGCoord(hoveredCell)} (#${hoveredCell})` : '-'}</div>
        <div>History len: {pastMoves.length} | Future len: {futureMoves.length}</div>
        <div>Board size: {size}×{size}</div>
      </div>
    </div>
  </aside>
</div>
