<script lang="ts">
    // Props for styling the button group container
    export let width: string | undefined = undefined;
    export let height: string | undefined = undefined;
</script>

<div class="button-group" style={`
    ${width ? `width: ${width};` : ''}
    ${height ? `height: ${height};` : ''}
`}>
    <slot></slot>
</div>

<style>
    .button-group {
        display: flex;
        flex-direction: row;
        align-items: stretch;
    }
    
    /* Make all buttons except the last one have no right border */
    :global(.button-group .retro-button:not(:last-child)) {
        margin-right: -2px; /* Account for border overlap */
    }
    
    /* Make internal buttons have a solid border for clean edges */
    :global(.button-group .retro-button:not(:first-child):not(:last-child)) {
        border-radius: 0;
    }
</style>
