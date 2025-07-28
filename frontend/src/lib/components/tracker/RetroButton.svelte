<script lang="ts">
    export let width = 'auto';
    export let height = 'auto';
    export let onClick = () => {};
    export let disabled = false;
    export let color = '#282C2C'; // Default button color

    // Calculate lighter and darker stroke colors based on the base color
    $: lightStrokeColor = calculateLighterColor(color);
    $: darkStrokeColor = calculateDarkerColor(color);

    // Function to lighten a color (for top/left edges)
    function calculateLighterColor(hexColor: string): string {
        return adjustColorBrightness(hexColor, 1.2); // 20% lighter
    }

    // Function to darken a color (for bottom/right edges)
    function calculateDarkerColor(hexColor: string): string {
        return adjustColorBrightness(hexColor, 0.6); // 40% darker
    }

    // Helper function to adjust color brightness
    function adjustColorBrightness(hexColor: string, factor: number): string {
        // Remove # if present
        hexColor = hexColor.replace('#', '');
        
        // Parse the hex color
        const r = parseInt(hexColor.substring(0, 2), 16);
        const g = parseInt(hexColor.substring(2, 4), 16);
        const b = parseInt(hexColor.substring(4, 6), 16);
        
        // Adjust brightness
        let newR = Math.min(255, Math.round(r * factor));
        let newG = Math.min(255, Math.round(g * factor));
        let newB = Math.min(255, Math.round(b * factor));
        
        // Convert back to hex
        return `#${newR.toString(16).padStart(2, '0')}${newG.toString(16).padStart(2, '0')}${newB.toString(16).padStart(2, '0')}`;
    }
</script>

<button 
    class="retro-button"
    style="
        width: {width}px; 
        height: {height}px; 
        --button-width: {width}px; 
        --button-height: {height}px;
        --button-color: {color};
        --light-stroke-color: {lightStrokeColor};
        --dark-stroke-color: {darkStrokeColor};
    "
    on:click={onClick}
    {disabled}
>
    <div class="button-content">
        <slot>Button</slot>
    </div>
</button>

<style>
    .retro-button {
        position: relative;
        background-color: var(--button-color, #282C2C);
        border: 2px solid #000000;
        padding: 0;
        display: flex;
        justify-content: center;
        align-items: center;
        font-family: 'Bytesized', monospace;
        color: #CCCCCC;
        cursor: pointer;
        text-align: center;
        box-sizing: border-box;
        user-select: none;
    }

    .retro-button::before {
        content: '';
        position: absolute;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        border-top: 2px solid var(--light-stroke-color, #353A3A);
        border-left: 2px solid var(--light-stroke-color, #353A3A);
        border-right: 2px solid var(--dark-stroke-color, #0C0D0D);
        border-bottom: 2px solid var(--dark-stroke-color, #0C0D0D);
        pointer-events: none;
    }

    .button-content {
        display: flex;
        justify-content: center;
        align-items: center;
        width: 100%;
        height: 100%;
        text-align: center;
        padding: 4px;
        font-size: calc(min(var(--button-width, 100px), var(--button-height, 40px)) * 0.44); /* Increased by 25% from 0.35 to 0.44 */
        line-height: 1;
    }

    .retro-button:active:not([disabled]) {
        background-color: var(--dark-stroke-color, #1E2020);
    }

    .retro-button:active:not([disabled])::before {
        border-top: 2px solid var(--dark-stroke-color, #0C0D0D);
        border-left: 2px solid var(--dark-stroke-color, #0C0D0D);
        border-right: 2px solid var(--light-stroke-color, #353A3A);
        border-bottom: 2px solid var(--light-stroke-color, #353A3A);
    }

    .retro-button[disabled] {
        opacity: 0.6;
        cursor: not-allowed;
    }
</style>
