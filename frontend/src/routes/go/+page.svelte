<script lang="ts">
  const size = 19;
  const cells = Array.from({ length: size * size }, (_, i) => i);
  let currentPlayer: 'black' | 'white' = 'black';
  let board: Array<'black' | 'white' | null> = Array(size * size).fill(null);
  let hoveredCell: number | null = null;

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
</script>

<style>
  /* Remove default focus outline from clickable areas */
  circle:focus {
    outline: none;
  }
</style>

<!-- Page background -->
<div class="min-h-screen w-full flex items-center justify-center" style="background-color: #098b5d;">
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
