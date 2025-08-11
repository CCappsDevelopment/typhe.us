<script lang="ts">
    import { onMount } from 'svelte';
    
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
    
    // Number of dots in each group
    const dotGroupSizes = [3, 2, 2, 1, 2];
    
    // Input state: Map of "rowIndex-groupIndex" -> hex value
    let inputValues: Map<string, string> = new Map();
    
    // Currently focused input: "rowIndex-groupIndex" or null
    let focusedInput: string | null = null;
    
    function handleRowDoubleClick(index: number) {
        onrowselect?.(index);
    }
    
    function handleDotGroupClick(rowIndex: number, groupIndex: number, event: MouseEvent) {
        event.stopPropagation(); // Prevent row double-click
        const inputKey = `${rowIndex}-${groupIndex}`;
        focusedInput = inputKey;
        
        // Initialize with empty string if not exists
        if (!inputValues.has(inputKey)) {
            inputValues.set(inputKey, '');
            inputValues = inputValues; // Trigger reactivity
        }
    }
    
    function handleKeyDown(event: KeyboardEvent, rowIndex: number, groupIndex: number) {
        const inputKey = `${rowIndex}-${groupIndex}`;
        const maxLength = dotGroupSizes[groupIndex];
        const currentValue = inputValues.get(inputKey) || '';
        
        if (event.key === 'Escape') {
            focusedInput = null;
            return;
        }
        
        if (event.key === 'Enter') {
            focusedInput = null;
            return;
        }
        
        if (event.key === 'Backspace') {
            const newValue = currentValue.slice(0, -1);
            inputValues.set(inputKey, newValue);
            inputValues = inputValues; // Trigger reactivity
            return;
        }
        
        // Check if it's a valid hex character
        const hexChar = event.key.toUpperCase();
        if (/^[0-9A-F]$/.test(hexChar) && currentValue.length < maxLength) {
            const newValue = currentValue + hexChar;
            inputValues.set(inputKey, newValue);
            inputValues = inputValues; // Trigger reactivity
        }
        
        event.preventDefault(); // Prevent default behavior for all keys
    }
    
    function getDisplayValue(rowIndex: number, groupIndex: number): string {
        const inputKey = `${rowIndex}-${groupIndex}`;
        const inputValue = inputValues.get(inputKey) || '';
        const maxLength = dotGroupSizes[groupIndex];
        
        // Create array of characters/dots with proper spacing
        const chars = [];
        for (let i = 0; i < maxLength; i++) {
            if (i < inputValue.length) {
                chars.push(inputValue[i]);
            } else {
                chars.push('_'); // Use bullet as placeholder for empty dots
            }
        }
        return chars.join('');
    }
    
    function isInputFocused(rowIndex: number, groupIndex: number): boolean {
        return focusedInput === `${rowIndex}-${groupIndex}`;
    }
    
    // Handle clicking outside to unfocus
    function handleGlobalClick(event: MouseEvent) {
        if (focusedInput && !(event.target as Element)?.closest('.dot-group')) {
            focusedInput = null;
        }
    }
    
    onMount(() => {
        document.addEventListener('click', handleGlobalClick);
        
        return () => {
            document.removeEventListener('click', handleGlobalClick);
        };
    });
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
                    <span 
                        class="dot-group" 
                        class:focused={isInputFocused(row.index, 0)}
                        style="--dot-color: {dotColors[0]}"
                        on:click={(e) => handleDotGroupClick(row.index, 0, e)}
                        on:keydown={(e) => handleKeyDown(e, row.index, 0)}
                        role="button"
                        tabindex="0"
                    >
                        {#each getDisplayValue(row.index, 0) as char}
                            {#if char === '•'}
                                <span class="dot">&#x25AA;</span>
                            {:else}
                                <span class="dot">{char}</span>
                            {/if}
                        {/each}
                    </span>
                    
                    <!-- Second group (2 dots) -->
                    <span 
                        class="dot-group" 
                        class:focused={isInputFocused(row.index, 1)}
                        style="--dot-color: {dotColors[1]}"
                        on:click={(e) => handleDotGroupClick(row.index, 1, e)}
                        on:keydown={(e) => handleKeyDown(e, row.index, 1)}
                        role="button"
                        tabindex="0"
                    >
                        {#each getDisplayValue(row.index, 1) as char}
                            {#if char === '•'}
                                <span class="dot">&#x25AA;</span>
                            {:else}
                                <span class="dot">{char}</span>
                            {/if}
                        {/each}
                    </span>
                    
                    <!-- Third group (2 dots) -->
                    <span 
                        class="dot-group" 
                        class:focused={isInputFocused(row.index, 2)}
                        style="--dot-color: {dotColors[2]}"
                        on:click={(e) => handleDotGroupClick(row.index, 2, e)}
                        on:keydown={(e) => handleKeyDown(e, row.index, 2)}
                        role="button"
                        tabindex="0"
                    >
                        {#each getDisplayValue(row.index, 2) as char}
                            {#if char === '•'}
                                <span class="dot">&#x25AA;</span>
                            {:else}
                                <span class="dot">{char}</span>
                            {/if}
                        {/each}
                    </span>
                    
                    <!-- Fourth group (1 dot) -->
                    <span 
                        class="dot-group" 
                        class:focused={isInputFocused(row.index, 3)}
                        style="--dot-color: {dotColors[3]}"
                        on:click={(e) => handleDotGroupClick(row.index, 3, e)}
                        on:keydown={(e) => handleKeyDown(e, row.index, 3)}
                        role="button"
                        tabindex="0"
                    >
                        {#each getDisplayValue(row.index, 3) as char}
                            {#if char === '•'}
                                <span class="dot">&#x25AA;</span>
                            {:else}
                                <span class="dot">{char}</span>
                            {/if}
                        {/each}
                    </span>
                    
                    <!-- Fifth group (2 dots) -->
                    <span 
                        class="dot-group" 
                        class:focused={isInputFocused(row.index, 4)}
                        style="--dot-color: {dotColors[4]}"
                        on:click={(e) => handleDotGroupClick(row.index, 4, e)}
                        on:keydown={(e) => handleKeyDown(e, row.index, 4)}
                        role="button"
                        tabindex="0"
                    >
                        {#each getDisplayValue(row.index, 4) as char}
                            {#if char === '•'}
                                <span class="dot">&#x25AA;</span>
                            {:else}
                                <span class="dot">{char}</span>
                            {/if}
                        {/each}
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
        gap: 6px;
        margin: 0 4px;
        cursor: pointer;
        padding: 2px 4px;
        border-radius: 2px;
        transition: background-color 0.1s ease;
    }
    
    .dot {
        color: var(--dot-color);
        font-size: 12px;
        font-family: 'Bytesized', 'Fontstuck', monospace;
    }
    
    .dot-group:hover {
        background-color: rgba(255, 255, 255, 0.1);
    }
    
    .dot-group.focused {
        background-color: rgba(255, 255, 255, 0.2);
        outline: 1px solid var(--dot-color);
    }
    
    .dot-group:focus {
        outline: 1px solid var(--dot-color);
        background-color: rgba(255, 255, 255, 0.2);
    }
</style>
