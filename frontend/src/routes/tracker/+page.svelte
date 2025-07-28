<script lang="ts">
    import RetroButton from '$lib/components/tracker/RetroButton.svelte';
</script>

<div class="min-h-screen bg-[#23293a] text-[#e0e6f0] font-sans flex flex-col items-center justify-center py-8">
    <div class="container mx-auto px-4">
        <h1 class="text-3xl font-bold mb-6 text-center">Music Tracker</h1>
        
        <div class="flex flex-col gap-8 items-center">
            <div>
                <h2 class="text-xl mb-2 text-center">Default Color (Original Size)</h2>
                <RetroButton width="128" height="64">TEST</RetroButton>
            </div>
            
            <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                <div class="flex flex-col items-center">
                    <h2 class="text-lg mb-2">Small (80x40)</h2>
                    <RetroButton width="80" height="40">TEST</RetroButton>
                </div>
                
                <div class="flex flex-col items-center">
                    <h2 class="text-lg mb-2">Medium (150x80)</h2>
                    <RetroButton width="150" height="80">TEST</RetroButton>
                </div>
                
                <div class="flex flex-col items-center">
                    <h2 class="text-lg mb-2">Large (200x120)</h2>
                    <RetroButton width="200" height="120">TEST</RetroButton>
                </div>
            </div>
            
            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                <div class="flex flex-col items-center">
                    <h2 class="text-lg mb-2">Wide (240x60)</h2>
                    <RetroButton width="240" height="60">WIDE TEST</RetroButton>
                </div>
                
                <div class="flex flex-col items-center">
                    <h2 class="text-lg mb-2">Tall (60x180)</h2>
                    <RetroButton width="60" height="180">TALL</RetroButton>
                </div>
            </div>
            
            <h2 class="text-2xl mt-6 mb-4 text-center">Color Variations</h2>
            <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                <div class="flex flex-col items-center">
                    <h2 class="text-lg mb-2">Blue</h2>
                    <RetroButton width="150" height="64" color="#2C3C7C">BLUE</RetroButton>
                </div>
                
                <div class="flex flex-col items-center">
                    <h2 class="text-lg mb-2">Red</h2>
                    <RetroButton width="150" height="64" color="#7C2C2C">RED</RetroButton>
                </div>
                
                <div class="flex flex-col items-center">
                    <h2 class="text-lg mb-2">Green</h2>
                    <RetroButton width="150" height="64" color="#2C7C2C">GREEN</RetroButton>
                </div>
            </div>
            
            <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mt-4">
                <div class="flex flex-col items-center">
                    <h2 class="text-lg mb-2">Purple</h2>
                    <RetroButton width="150" height="64" color="#4C2C7C">PURPLE</RetroButton>
                </div>
                
                <div class="flex flex-col items-center">
                    <h2 class="text-lg mb-2">Teal</h2>
                    <RetroButton width="150" height="64" color="#2C7C7C">TEAL</RetroButton>
                </div>
                
                <div class="flex flex-col items-center">
                    <h2 class="text-lg mb-2">Orange</h2>
                    <RetroButton width="150" height="64" color="#7C4C2C">ORANGE</RetroButton>
                </div>
            </div>
        </div>
    </div>
</div>