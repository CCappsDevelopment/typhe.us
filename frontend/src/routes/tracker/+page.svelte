<script lang="ts">
	import RetroButton from '$lib/components/tracker/RetroButton.svelte';
	import RetroButtonGroup from '$lib/components/tracker/RetroButtonGroup.svelte';
	import TrackerWindow from '$lib/components/tracker/TrackerWindow.svelte';
</script>

<div
	class="flex min-h-screen flex-col items-center justify-center bg-[#23293A] py-8 font-sans text-[#e0e6f0]"
>
	<div class="container mx-auto px-4">
		<h1 class="mb-6 text-center text-3xl font-bold">Music Tracker</h1>

		<div class="tracker-interface">
            <!-- Tracker Window Proof of Concept -->
            <div class="section mb-8">
                <h2 class="mb-4 text-xl font-semibold">Tracker Window</h2>
                <div class="bg-[#1A1E2A] p-6 rounded flex justify-center">
                    <TrackerWindow trackNumber={1} width={320} height={400} />
                </div>
            </div>
        
            <!-- Transport Controls -->
            <div class="section mb-8">
                <h2 class="mb-4 text-xl font-semibold">Transport Controls</h2>
                <div class="bg-[#1A1E2A] p-6 rounded">
                    <RetroButtonGroup>
                        <RetroButton width="120" height="64" color="#282C2C">PLAY</RetroButton>
                        <RetroButton width="120" height="64" color="#2C3C7C">STOP</RetroButton>
                        <RetroButton width="120" height="64" color="#7C2C2C">SAVE</RetroButton>
                        <RetroButton width="120" height="64" color="#555555">EXIT</RetroButton>
                    </RetroButtonGroup>
                </div>
            </div>
            
            <!-- File operation buttons -->
            <div class="section mb-8">
                <h2 class="mb-4 text-xl font-semibold">File Operations</h2>
                <div class="bg-[#1A1E2A] p-6 rounded">
                    <RetroButtonGroup>
                        <RetroButton width="120" height="40" color="#282C2C">LOAD</RetroButton>
                        <RetroButton width="120" height="40" color="#2C3C7C">SAVE</RetroButton>
                        <RetroButton width="120" height="40" color="#7C2C2C">NEW</RetroButton>
                    </RetroButtonGroup>
                </div>
            </div>
            
            <!-- Different sized buttons -->
            <div class="section mb-8">
                <h2 class="mb-4 text-xl font-semibold">Variable Size Buttons</h2>
                <div class="bg-[#1A1E2A] p-6 rounded">
                    <RetroButtonGroup>
                        <RetroButton width="80" height="80" color="#282C2C">S</RetroButton>
                        <RetroButton width="120" height="80" color="#2C3C7C">MED</RetroButton>
                        <RetroButton width="160" height="80" color="#7C2C2C">LARGE</RetroButton>
                    </RetroButtonGroup>
                </div>
            </div>
        </div>
	</div>
</div>

<style>
    .tracker-interface {
        width: 100%;
        max-width: 800px;
        margin: 0 auto;
    }
    
    .section {
        width: 100%;
    }
</style>
