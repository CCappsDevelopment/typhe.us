/**
 * Adjusts a hex color's brightness by a given factor
 * @param hexColor - Hex color string (with or without #)
 * @param factor - Multiplication factor (>1 brightens, <1 darkens)
 * @returns Adjusted hex color with #
 */
export function adjustColorBrightness(hexColor: string, factor: number): string {
    // Remove # if present
    hexColor = hexColor.replace('#', '');
    
    // Parse the hex color
    const r = parseInt(hexColor.substring(0, 2), 16);
    const g = parseInt(hexColor.substring(2, 4), 16);
    const b = parseInt(hexColor.substring(4, 6), 16);
    
    // Adjust brightness
    const newR = Math.min(255, Math.round(r * factor));
    const newG = Math.min(255, Math.round(g * factor));
    const newB = Math.min(255, Math.round(b * factor));
    
    // Convert back to hex
    return `#${newR.toString(16).padStart(2, '0')}${newG.toString(16).padStart(2, '0')}${newB.toString(16).padStart(2, '0')}`;
}

/**
 * Calculates a lighter version of the given color
 * @param hexColor - Hex color string
 * @returns 20% lighter color
 */
export function calculateLighterColor(hexColor: string): string {
    return adjustColorBrightness(hexColor, 1.2); // 20% lighter
}

/**
 * Calculates a darker version of the given color
 * @param hexColor - Hex color string
 * @returns 40% darker color
 */
export function calculateDarkerColor(hexColor: string): string {
    return adjustColorBrightness(hexColor, 0.6); // 40% darker
}
