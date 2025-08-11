<script lang="ts">
    import { onMount } from 'svelte';
    import * as trackerDSP from '$lib/tracker-wasm/tracker_dsp.js';

    let notes = [
        { name: 'C', semitone: 0, key: 'z', isSharp: false },
        { name: 'C#', semitone: 1, key: 's', isSharp: true },
        { name: 'D', semitone: 2, key: 'x', isSharp: false },
        { name: 'D#', semitone: 3, key: 'd', isSharp: true },
        { name: 'E', semitone: 4, key: 'c', isSharp: false },
        { name: 'F', semitone: 5, key: 'v', isSharp: false },
        { name: 'F#', semitone: 6, key: 'g', isSharp: true },
        { name: 'G', semitone: 7, key: 'b', isSharp: false },
        { name: 'G#', semitone: 8, key: 'h', isSharp: true },
        { name: 'A', semitone: 9, key: 'n', isSharp: false },
        { name: 'A#', semitone: 10, key: 'j', isSharp: true },
        { name: 'B', semitone: 11, key: 'm', isSharp: false },
        { name: 'C5', semitone: 12, key: ',', isSharp: false }
    ];
    
    let octaves = [2, 3, 4, 5, 6];
    let selectedOctave = 4; // Default: A4=440Hz

    // Waveform types
    let waveformTypes = [
        { name: 'Sine', value: 'sine' },
        { name: 'Square', value: 'square' },
        { name: 'Sawtooth', value: 'sawtooth' },
        { name: 'Triangle', value: 'triangle' }
    ];
    let selectedWaveform = 'sine'; // Default waveform

    let wasmReady = false;
    let audioContext: AudioContext | null = null;
    let activeNotes: Map<string, { source: AudioBufferSourceNode, ctx: AudioContext }> = new Map();
    // Track currently active keys for visual feedback
    let activeKeys = new Set<string>();

    // Store references to WASM functions after init.
    let generateSineWave: ((freq: number, duration: number, sr: number) => Float32Array) | undefined;
    let generateSquareWave: ((freq: number, duration: number, sr: number) => Float32Array) | undefined;
    let generateSawtoothWave: ((freq: number, duration: number, sr: number) => Float32Array) | undefined;
    let generateTriangleWave: ((freq: number, duration: number, sr: number) => Float32Array) | undefined;

    // Helper: Convert note/octave to frequency (A4=440Hz)
    function noteToFreq(semitone: number, octave: number) {
        const a4 = 440;
        const n = semitone + 12 * (octave - 4); // Distance from A4 in semitones
        return a4 * Math.pow(2, (n - 9) / 12);
    }

    onMount(() => {
        // Async initialization
        (async () => {
            await trackerDSP.default();         // Initialize wasm-bindgen module
            
            // Initialize all waveform generator functions
            generateSineWave = trackerDSP.generate_sine_wave;
            generateSquareWave = trackerDSP.generate_square_wave;
            generateSawtoothWave = trackerDSP.generate_sawtooth_wave;
            generateTriangleWave = trackerDSP.generate_triangle_wave;
            
            wasmReady = true;
        })();

        // Add keyboard event listeners
        window.addEventListener('keydown', handleKeyDown);
        window.addEventListener('keyup', handleKeyUp);

        // Return cleanup function
        return () => {
            window.removeEventListener('keydown', handleKeyDown);
            window.removeEventListener('keyup', handleKeyUp);
            // Clean up any active notes
            stopAllNotes();
        };
    });
    
    function handleKeyDown(e: KeyboardEvent) {
        if (!wasmReady || e.repeat) return;
        
        // Find the note that corresponds to this key
        const note = notes.find(n => n.key === e.key.toLowerCase());
        if (note) {
            const key = e.key.toLowerCase();
            // Add key to active keys for immediate visual feedback
            activeKeys = new Set(activeKeys).add(key);
            playNote(note.semitone, key);
        }
    }
    
    function handleKeyUp(e: KeyboardEvent) {
        const key = e.key.toLowerCase();
        if (activeNotes.has(key)) {
            // Update activeKeys first for immediate visual feedback
            activeKeys = new Set([...activeKeys].filter(k => k !== key));
            stopNote(key);
        }
    }
    
    function stopAllNotes() {
        activeNotes.forEach((noteData, key) => {
            stopNote(key);
        });
    }
    
    function stopNote(key: string) {
        if (activeNotes.has(key)) {
            const { source, ctx } = activeNotes.get(key)!;
            source.stop();
            setTimeout(() => ctx.close(), 100); // Give a small delay before closing context
            activeNotes.delete(key);
            activeKeys.delete(key); // Also remove from active keys
        }
    }

    async function playNote(semitone: number, key: string) {
        if (!wasmReady) return;
        
        // If this note is already playing, stop it first
        if (activeNotes.has(key)) {
            stopNote(key);
        }
        
        const freq = noteToFreq(semitone, selectedOctave);
        const duration = 10.0; // Long duration, we'll stop it manually
        const sampleRate = 44100;

        // Generate samples based on the selected waveform type
        let samples: Float32Array;
        switch (selectedWaveform) {
            case 'square':
                if (!generateSquareWave) return;
                samples = generateSquareWave(freq, duration, sampleRate);
                break;
            case 'sawtooth':
                if (!generateSawtoothWave) return;
                samples = generateSawtoothWave(freq, duration, sampleRate);
                break;
            case 'triangle':
                if (!generateTriangleWave) return;
                samples = generateTriangleWave(freq, duration, sampleRate);
                break;
            case 'sine':
            default:
                if (!generateSineWave) return;
                samples = generateSineWave(freq, duration, sampleRate);
        }

        // Play using Web Audio API
        const ctx = new AudioContext();
        const buffer = ctx.createBuffer(1, samples.length, sampleRate);
        buffer.copyToChannel(Float32Array.from(samples), 0);
        const source = ctx.createBufferSource();
        source.buffer = buffer;
        source.connect(ctx.destination);
        source.start();
        
        // Store reference to the playing note
        activeNotes.set(key, { source, ctx });
    }

    // For mouse click support on the virtual keyboard
    function handlePianoKeyClick(semitone: number, noteName: string) {
        if (!wasmReady) return;
        const key = notes.find(n => n.semitone === semitone)?.key || '';
        playNote(semitone, key);
        
        // Simulate key release after 300ms
        setTimeout(() => {
            if (key) stopNote(key);
        }, 300);
    }
</script>

<!-- TailwindCSS classes for dark blue-grey theme -->
<div class="min-h-screen bg-[#23293a] text-[#e0e6f0] font-sans flex flex-col items-center justify-center py-8">
  <div class="w-full max-w-md">
    <div class="mb-6 text-center">
      <h1 class="text-2xl font-bold mb-2">Virtual Piano</h1>
      <div class="flex justify-center space-x-3 mb-4">
        <select
          bind:value={selectedOctave}
          class="bg-[#2c3446] text-[#e0e6f0] border border-[#3a4256] rounded px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-700"
        >
          {#each octaves as oct}
            <option value={oct}>Octave {oct}</option>
          {/each}
        </select>
        
        <select
          bind:value={selectedWaveform}
          class="bg-[#2c3446] text-[#e0e6f0] border border-[#3a4256] rounded px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-700"
        >
          {#each waveformTypes as waveform}
            <option value={waveform.value}>{waveform.name}</option>
          {/each}
        </select>
      </div>
      <p class="text-sm opacity-70">Use your keyboard or click the keys to play</p>
    </div>
    
    <div class="relative h-48">
      <!-- White keys -->
      <div class="flex h-full">
        {#each notes.filter(n => !n.isSharp) as note}
          <button 
            on:mousedown={() => handlePianoKeyClick(note.semitone, note.name)}
            on:mouseup={() => stopNote(note.key)}
            on:mouseleave={() => stopNote(note.key)}
            class={`flex-1 border border-gray-300 rounded-b flex flex-col justify-end items-center pb-2 
                  text-black relative focus:outline-none active:bg-gray-100
                  ${activeKeys.has(note.key) ? 'bg-gray-200 shadow-inner' : 'bg-white'}`}
          >
            <span class="text-sm font-bold">{note.name}</span>
            <span class="text-xs text-gray-500 mt-1">{note.key}</span>
          </button>
        {/each}
      </div>
      
      <!-- Black keys -->
      <div class="absolute top-0 flex h-28 w-full pointer-events-none">
        {#each notes.filter(n => n.isSharp) as note, i}
          <div class="flex-1 flex justify-center">
            {#if note.isSharp}
              <button 
                on:mousedown={() => handlePianoKeyClick(note.semitone, note.name)}
                on:mouseup={() => stopNote(note.key)}
                on:mouseleave={() => stopNote(note.key)}
                class={`pointer-events-auto w-3/5 h-full rounded-b border border-gray-700 z-10 flex flex-col justify-end
                      items-center pb-2 text-white hover:bg-gray-800 active:bg-gray-900
                      ${activeKeys.has(note.key) ? 'bg-gray-900 shadow-inner transform translate-y-0.5' : 'bg-black'}`}
              >
                <span class="text-sm font-bold">{note.name}</span>
                <span class="text-xs text-gray-400 mt-1">{note.key}</span>
              </button>
            {/if}
          </div>
        {/each}
      </div>
    </div>
    
    <div class="mt-8 px-4">
      <h2 class="font-medium mb-2">Keyboard Mapping:</h2>
      <div class="grid grid-cols-2 gap-2 text-sm">
        <div>
          <h3 class="font-medium">White keys:</h3>
          <ul class="list-disc list-inside">
            {#each notes.filter(n => !n.isSharp) as note}
              <li><span class="font-mono">{note.key}</span> - {note.name}</li>
            {/each}
          </ul>
        </div>
        <div>
          <h3 class="font-medium">Black keys:</h3>
          <ul class="list-disc list-inside">
            {#each notes.filter(n => n.isSharp) as note}
              <li><span class="font-mono">{note.key}</span> - {note.name}</li>
            {/each}
          </ul>
        </div>
      </div>
    </div>
  </div>
  
  {#if !wasmReady}
    <div class="absolute inset-0 bg-[#23293a] bg-opacity-90 flex items-center justify-center">
      <div class="text-center p-6">
        <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-white mx-auto mb-4"></div>
        <p class="text-lg">Loading audio engine...</p>
      </div>
    </div>
  {/if}
</div>
