<script lang="ts">
    import { onMount } from 'svelte';
    
    export let trackNumber: number = 1;
    export let width: number = 320;
    export let height: number = 400;
    
    // Constants
    const TOTAL_ROWS = 64; // 0x00 to 0x3F (64 rows)
    const FRAME_THICKNESS = 6; // 6px frame thickness
    
    // State
    let selectedRow = 0; // Start with row 00 selected
    let containerElement: HTMLElement;
    let rowHeight = 20; // Fixed row height
    let visibleRows = 20; // Number of rows visible in the container
    
    // Colors
    const frameColor = "#586078";
    const frameColorLight = "#6e7898"; // 20% lighter
    const frameColorDark = "#353b48"; // 40% darker
    const selectionColor = "#303540";
    const selectionColorLight = "#464b59"; // 20% lighter
    const selectionColorDark = "#1d2130"; // 40% darker
    
    // Dot colors for each group
    const dotColors = [
        "#FFFFFF", // white
        "#8A9CFF", // bluish
        "#6DDBD8", // tealish
        "#E0E6F0", // off-white
        "#888888"  // grey
    ];
    
    // Calculate visible rows and their positions
    $: {
        if (containerElement) {
            visibleRows = Math.ceil(containerElement.clientHeight / rowHeight) + 2; // Add buffer
        }
    }
    
    // Calculate which rows should be visible and their positions
    $: visibleRowData = (() => {
        const halfVisible = Math.floor(visibleRows / 2);
        const rows = [];
        
        for (let i = 0; i < visibleRows; i++) {
            const offsetFromCenter = i - halfVisible;
            const rowIndex = selectedRow + offsetFromCenter;
            
            // Simple rule: only show rows that are within 0-63 range, no wrap-around
            if (rowIndex >= 0 && rowIndex < TOTAL_ROWS) {
                // Shift all rows up by one row height to align with selection window
                const yPosition = (i * rowHeight) - rowHeight;
                
                rows.push({
                    index: rowIndex,
                    hex: rowIndex.toString(16).padStart(2, '0').toUpperCase(),
                    isSpecialNumber: rowIndex % 4 === 0,
                    isSelected: rowIndex === selectedRow,
                    yPosition: yPosition
                });
            }
            // If rowIndex is outside 0-63 range, we simply don't show anything (black space)
        }
        
        return rows;
    })();
    
    onMount(() => {
        // Calculate container height and visible rows
        if (containerElement) {
            visibleRows = Math.ceil(containerElement.clientHeight / rowHeight) + 2;
        }
        
        // Add keyboard event listeners
        window.addEventListener('keydown', handleKeyDown);
        
        // Return cleanup function
        return () => {
            window.removeEventListener('keydown', handleKeyDown);
        };
    });
    
    // Handle keyboard navigation
    function handleKeyDown(e: KeyboardEvent) {
        if (e.key === 'ArrowUp') {
            e.preventDefault();
            selectRow(selectedRow > 0 ? selectedRow - 1 : TOTAL_ROWS - 1);
        } else if (e.key === 'ArrowDown') {
            e.preventDefault();
            selectRow(selectedRow < TOTAL_ROWS - 1 ? selectedRow + 1 : 0);
        }
    }
    
    // Handle row selection
    function selectRow(index: number) {
        selectedRow = ((index % TOTAL_ROWS) + TOTAL_ROWS) % TOTAL_ROWS; // Ensure valid range with wrap-around
    }
    
    function handleRowDoubleClick(index: number) {
        selectRow(index);
    }
    
    // Handle clicking on the scrollbar track
    function handleTrackClick(e: MouseEvent) {
        // Calculate relative position
        const trackRect = (e.currentTarget as HTMLElement).getBoundingClientRect();
        const clickPosition = e.clientY - trackRect.top;
        const trackHeight = trackRect.height;
        
        // Calculate which row this corresponds to
        const rowIndex = Math.floor((clickPosition / trackHeight) * TOTAL_ROWS);
        if (rowIndex >= 0 && rowIndex < TOTAL_ROWS) {
            selectRow(rowIndex);
        }
    }
    
    // Calculate scrollbar thumb position based on selected row
    $: thumbPosition = containerElement ? 
        (selectedRow / (TOTAL_ROWS - 1)) * (containerElement.clientHeight - 30) : 0;
</script>

<div class="tracker-container" style="--frame-thickness: {FRAME_THICKNESS}px; --width: {width}px; --height: {height}px;">
    <!-- Track number heading -->
    <div class="track-number">{trackNumber}</div>
    
    <!-- Tracker frame -->
    <div class="tracker-frame">        
        <!-- Tracker content area -->
        <div class="tracker-content-wrapper" bind:this={containerElement}>
            <!-- Row numbers column -->
            <div class="row-numbers">
                {#each visibleRowData as row}
                    <div 
                        class="row-number" 
                        class:highlighted={row.isSelected}
                        class:special-number={row.isSpecialNumber}
                        style="position: absolute; top: {row.yPosition}px;"
                    >
                        {row.hex}
                    </div>
                {/each}
            </div>
            
            <!-- Scrollable content -->
            <div class="tracker-content">
                {#each visibleRowData as row}
                    <div 
                        class="tracker-row" 
                        class:highlighted={row.isSelected}
                        on:dblclick={() => handleRowDoubleClick(row.index)}
                        role="button"
                        tabindex="0"
                        aria-label="Row {row.hex}"
                        style="position: absolute; top: {row.yPosition}px;"
                    >
                        <!-- First group (3 dots) -->
                        <span class="dot-group" style="--dot-color: {dotColors[0]}">
                            <span class="dot">&#x25AA;</span>
                            <span class="dot">&#x25AA;</span>
                            <span class="dot">&#x25AA;</span>
                        </span>
                        
                        <!-- Second group (2 dots) -->
                        <span class="dot-group" style="--dot-color: {dotColors[1]}">
                            <span class="dot">&#x25AA;</span>
                            <span class="dot">&#x25AA;</span>
                        </span>
                        
                        <!-- Third group (2 dots) -->
                        <span class="dot-group" style="--dot-color: {dotColors[2]}">
                            <span class="dot">&#x25AA;</span>
                            <span class="dot">&#x25AA;</span>
                        </span>
                        
                        <!-- Fourth group (1 dot) -->
                        <span class="dot-group" style="--dot-color: {dotColors[3]}">
                            <span class="dot">&#x25AA;</span>
                        </span>
                        
                        <!-- Fifth group (2 dots) -->
                        <span class="dot-group" style="--dot-color: {dotColors[4]}">
                            <span class="dot">&#x25AA;</span>
                            <span class="dot">&#x25AA;</span>
                        </span>
                    </div>
                {/each}
            </div>
            
            <!-- Scrollbar -->
            <div class="scrollbar-container">
                <div 
                    class="scrollbar-track" 
                    on:mousedown={handleTrackClick} 
                    role="button"
                    tabindex="0"
                    aria-label="Scrollbar track"
                >
                    <div 
                        class="scrollbar-thumb" 
                        style="top: {thumbPosition}px"
                    ></div>
                </div>
            </div>
        </div>
    </div>
</div>

<style>
    .tracker-container {
        position: relative;
        width: var(--width);
        height: var(--height);
        display: flex;
        flex-direction: column;
        align-items: center;
        margin: 20px auto;
        font-family: 'Bytesized', 'Fontstuck', monospace;
    }
    
    .track-number {
        font-size: 32px;
        font-family: 'Bytesized', 'Fontstuck', monospace;
        color: #FFFF00;
        text-align: center;
        margin-bottom: 5px;
    }
    
    .tracker-frame {
        width: 100%;
        height: calc(100% - 45px);
        display: flex;
        background-color: #586078;
        border-top: 2px solid #6e7898;
        border-left: 2px solid #6e7898;
        border-right: 2px solid #353b48;
        border-bottom: 2px solid #353b48;
        padding: var(--frame-thickness);
        box-sizing: border-box;
    }
    
    .tracker-content-wrapper {
        position: relative;
        flex: 1;
        display: flex;
        overflow: hidden;
        background-color: black;
    }
    
    .row-numbers {
        position: relative;
        width: 30px;
        background-color: #161822;
        overflow: hidden;
    }
    
    .row-number {
        position: absolute;
        height: 20px;
        width: 100%;
        display: flex;
        align-items: center;
        justify-content: center;
        color: #FFFFFF;
        font-size: 12px;
    }
    
    .special-number {
        color: #FFFF00; /* Vibrant yellow for every 4th row */
    }
    
    .tracker-content-wrapper {
        position: relative;
        flex: 1;
        display: flex;
        overflow: hidden;
        background-color: black;
    }
    
    .tracker-content {
        position: relative;
        width: calc(100% - 36px); /* Leave space for row numbers and scrollbar */
        height: 100%;
        overflow: hidden;
        margin-left: 30px; /* Account for row numbers */
    }
    
    .tracker-row {
        position: absolute;
        height: 20px;
        width: calc(100% - 20px); /* Account for scrollbar */
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 0 10px;
        box-sizing: border-box;
        cursor: pointer;
    }
    
    .highlighted {
        background-color: #303540;
        border-top: 2px solid #464b59;
        border-bottom: 2px solid #1d2130;
        box-sizing: border-box;
        height: 20px;
    }
    
    /* Apply highlight to row numbers as well */
    .row-number.highlighted {
        background-color: #303540;
        border-top: 2px solid #464b59;
        border-bottom: 2px solid #1d2130;
    }
    
    .tracker-row:focus {
        outline: none;
    }
    
    .dot-group {
        display: flex;
        gap: 6px;
        margin: 0 4px;
    }
    
    .dot {
        color: var(--dot-color);
        font-size: 12px;
    }
    
    .scrollbar-container {
        position: absolute;
        right: 0;
        top: 0;
        width: 16px;
        height: 100%;
        background-color: #1A1E28;
        border-left: 2px solid var(--frame-color-dark, #353b48);
    }
    
    .scrollbar-track {
        position: relative;
        width: 100%;
        height: 100%;
    }
    
    .scrollbar-thumb {
        position: absolute;
        width: 12px;
        right: 0;
        height: 30px; /* Will be calculated based on content */
        background-color: #586078;
        border-top: 2px solid #6e7898;
        border-left: 2px solid #6e7898;
        border-right: 2px solid #353b48;
        border-bottom: 2px solid #353b48;
    }
</style>
