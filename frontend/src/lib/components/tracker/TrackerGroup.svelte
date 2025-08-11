<script lang="ts">
    import TrackerColumn from './TrackerColumn.svelte';
    import { onMount } from 'svelte';
    
    export let trackerCount: number = 4;
    export let trackerNumbers: number[] = [1, 2, 3, 4]; // Default numbers
    export let width: number = 320;
    export let height: number = 400;
    
    // Constants
    const TOTAL_ROWS = 64;
    const DIVIDER_WIDTH = 6;
    const FRAME_THICKNESS = 6;
    
    // Colors for divider and frame
    const dividerColor = "#586078";
    const dividerColorLight = "#6e7898"; // 20% lighter
    const dividerColorDark = "#353b48"; // 40% darker
    const frameColor = "#586078";
    const frameColorLight = "#6e7898"; // 20% lighter
    const frameColorDark = "#353b48"; // 40% darker
    
    // Shared state for all trackers in the group
    let selectedRow = 0;
    let containerElement: HTMLElement;
    let rowHeight = 20; // Fixed row height
    let visibleRows = 20; // Number of rows visible in the container
    
    // Ensure trackerNumbers array has the right length
    $: {
        if (trackerNumbers.length !== trackerCount) {
            trackerNumbers = Array.from({length: trackerCount}, (_, i) => i + 1);
        }
    }
    
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
        
        // Add keyboard event listeners for the group
        window.addEventListener('keydown', handleKeyDown);
        
        return () => {
            window.removeEventListener('keydown', handleKeyDown);
        };
    });
    
    // Handle keyboard navigation for the entire group
    function handleKeyDown(e: KeyboardEvent) {
        if (e.key === 'ArrowUp') {
            e.preventDefault();
            selectedRow = selectedRow > 0 ? selectedRow - 1 : TOTAL_ROWS - 1;
        } else if (e.key === 'ArrowDown') {
            e.preventDefault();
            selectedRow = selectedRow < TOTAL_ROWS - 1 ? selectedRow + 1 : 0;
        }
    }
    
    // Handle row selection from any tracker in the group
    function handleRowSelect(index: number) {
        selectedRow = ((index % TOTAL_ROWS) + TOTAL_ROWS) % TOTAL_ROWS;
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
            selectedRow = rowIndex;
        }
    }
    
    // Calculate scrollbar thumb position based on selected row
    $: thumbPosition = containerElement ? 
        (selectedRow / (TOTAL_ROWS - 1)) * (containerElement.clientHeight - 30) : 0;
    
    // Calculate total width for the group (row numbers + trackers + dividers + frame + scrollbar)
    $: totalWidth = 30 + (width * trackerCount) + (DIVIDER_WIDTH * (trackerCount - 1)) + (FRAME_THICKNESS * 2) + 16;
</script>

<div class="tracker-group-container" style="--total-width: {totalWidth}px; --height: {height}px; --frame-thickness: {FRAME_THICKNESS}px;">
    <!-- Tracker frame -->
    <div class="tracker-frame">        
        <!-- Tracker content area -->
        <div class="tracker-content-wrapper" bind:this={containerElement}>
            <!-- Shared row numbers column -->
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
            
            <div class="tracker-columns">
                {#each trackerNumbers as trackNumber, i}
                    {#if i > 0}
                        <!-- Divider between trackers -->
                        <div class="tracker-divider" style="--divider-width: {DIVIDER_WIDTH}px;"></div>
                    {/if}
                    
                    <TrackerColumn 
                        {trackNumber}
                        {visibleRowData}
                        onrowselect={handleRowSelect}
                    />
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
    .tracker-group-container {
        position: relative;
        width: var(--total-width);
        height: var(--height);
        display: flex;
        flex-direction: column;
        align-items: center;
        margin: 20px auto;
        font-family: 'Bytesized', 'Fontstuck', monospace;
    }
    
    .tracker-frame {
        width: 100%;
        height: 100%;
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
        flex-shrink: 0;
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
    
    /* Apply highlight to row numbers as well */
    .row-number.highlighted {
        background-color: #303540;
        border-top: 2px solid #464b59;
        border-bottom: 2px solid #1d2130;
    }
    
    .tracker-columns {
        display: flex;
        align-items: flex-start;
        gap: 0;
        flex: 1;
        margin-right: 16px; /* Account for scrollbar */
    }
    
    .tracker-divider {
        width: var(--divider-width);
        height: 100%;
        background-color: #586078;
        border-left: 2px solid #6e7898; /* 20% lighter */
        border-right: 2px solid #353b48; /* 40% darker */
        flex-shrink: 0;
    }
    
    .scrollbar-container {
        position: absolute;
        right: 0;
        top: 0;
        width: 16px;
        height: 100%;
        background-color: #1A1E28;
        border-left: 2px solid #353b48;
        flex-shrink: 0;
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
        height: 30px;
        background-color: #586078;
        border-top: 2px solid #6e7898;
        border-left: 2px solid #6e7898;
        border-right: 2px solid #353b48;
        border-bottom: 2px solid #353b48;
    }
</style>
