<script lang="ts">
    export let trackNumber: number = 1;
    export let visibleRowData: any[] = [];
    export let onrowselect: ((row: number) => void) | undefined = undefined; // Callback prop
    
    // Dot colors for each group
    const dotColors = [
        "#FFFFFF", // white
        "#8A9CFF", // bluish
        "#6DDBD8", // tealish
        "#E0E6F0", // off-white
        "#888888"  // grey
    ];
    
    function handleRowDoubleClick(index: number) {
        onrowselect?.(index);
    }
</script>

<div class="tracker-column">
    <!-- Track number heading -->
    <div class="track-number">{trackNumber}</div>
    
    <!-- Content area -->
    <div class="tracker-content-area">
        <!-- Tracker content -->
        <div class="tracker-content">
            {#each visibleRowData as row}
                <div 
                    class="tracker-row" 
                    class:highlighted={row.isSelected}
                    on:dblclick={() => handleRowDoubleClick(row.index)}
                    role="button"
                    tabindex="0"
                    aria-label="Row {row.hex}"
                    style="position: absolute; top: {row.yPosition - 45}px;"
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
    </div>
</div>

<style>
    .tracker-column {
        position: relative;
        width: 256px;
        height: 100%;
        display: flex;
        flex-direction: column;
        align-items: center;
        font-family: 'Bytesized', 'Fontstuck', monospace;
    }
    
    .track-number {
        font-size: 32px;
        font-family: 'Bytesized', 'Fontstuck', monospace;
        color: #FFFF00;
        text-align: center;
        margin-bottom: 5px;
        height: 40px;
        display: flex;
        align-items: center;
        justify-content: center;
    }
    
    .tracker-content-area {
        position: relative;
        flex: 1;
        display: flex;
        width: 100%;
        overflow: hidden;
        background-color: black;
    }
    
    .tracker-content {
        position: relative;
        flex: 1;
        height: 100%;
        overflow: hidden;
    }
    
    .tracker-row {
        position: absolute;
        height: 20px;
        width: 100%;
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
    
    .tracker-row:focus {
        outline: none;
    }
    
    .dot-group {
        display: flex;
        gap: 16px;
        margin: 0 4px;
    }
    
    .dot {
        color: var(--dot-color);
        font-size: 12px;
    }
</style>
