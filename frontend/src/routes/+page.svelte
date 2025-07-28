<script>
	// Animation effects could be added here if needed in the future
</script>

<div
	class="flex min-h-screen flex-col items-center justify-center bg-gradient-to-br from-[#1a2030] to-[#2d3748] p-6 text-white"
>
	<div class="w-full max-w-3xl text-center">
		<!-- Logo/Header Section -->
		<div class="mb-8">
			<h1
				class="font-fontstuck mb-0 bg-gradient-to-r from-[#d5efed] to-[#9fe1e0] bg-clip-text pt-5 pb-0 text-5xl leading-relaxed font-bold tracking-tighter text-transparent md:text-7xl"
			>
				TYPHEUS
			</h1>
			<p class="font-bytesized mt-0 mb-6 text-2xl font-bold tracking-wide text-blue-200">
				A Place For Stuff
			</p>

			<!-- Browser Preview Image -->
			<div class="relative mx-auto w-full max-w-md">
				<img
					src="/typheus-browser.png"
					alt="Typheus Browser Preview"
					class="mx-auto w-3/4 rounded-lg"
				/>
			</div>
		</div>

		<!-- Construction Notice -->
		<div class="neo-brutalist-container border-2 border-[#9fe1e0] bg-[#2a3341]/60 p-8 backdrop-blur-sm relative">
			<div class="mb-6 flex justify-center">
				<div class="relative">
					<!-- Spinning circle with pixelation effect -->
					<svg
						class="animate-spin-slow pixelated h-32 w-32 text-[#9fe1e0]"
						xmlns="http://www.w3.org/2000/svg"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<!-- Using a simplified shape with straight lines for more visible pixelation -->
						<path
							d="M12 2 L14 2 L16 3 L18 4 L20 6 L21 8 L22 10 L22 14 L21 16 L20 18 L18 20 L16 21 L14 22 L10 22 L8 21 L6 20 L4 18 L3 16 L2 14 L2 10 L3 8 L4 6 L6 4 L8 3 L10 2 Z"
							class="opacity-25"
						></path>
						<path class="opacity-75" d="M12 2 L14 2 L16 3 L18 4 L20 6 M12 2 L10 2 L8 3 L6 4 L4 6"
						></path>
					</svg>

					<!-- Typheus logo in center -->
					<div class="absolute top-0 right-0 bottom-0 left-0 flex items-center justify-center">
						<img
							src="/typheus.png"
							alt="Typheus Logo"
							class="h-16 w-16 rounded-full object-contain p-1"
						/>
					</div>
				</div>
			</div>

			<h2 class="text-l font-fontstuck mb-4 font-bold text-blue-100 md:text-xl">
				A Large, Ominous Countdown Looms Over This Page. . .
			</h2>
			<p class="font-bytesized mb-6 text-lg text-blue-200">Stay tuned.</p>
		</div>

		<!-- Footer -->
		<div class="font-bytesized mt-12 text-sm text-blue-300">
			&copy; 2025 CCappsDevelopment &bull; Font “FONTSTUCK” by
			<a href="https://fontstruct.com/fontstructions/show/675134" target="_blank" rel="noopener"
				>KevinHilt</a
			>
		</div>
	</div>
</div>

<style>
	:global(body) {
		margin: 0;
		font-family: ui-sans-serif, system-ui, sans-serif;
	}

	/* Make the Fontstuck font available in Tailwind */
	.font-fontstuck {
		font-family: 'Fontstuck', ui-sans-serif, system-ui, sans-serif;
		line-height: 1.2;
		padding: 0.25em 0 0;
		margin-bottom: 0;
		display: inline-block;
	}

	/* Bytesized font class for the subheader */
	.font-bytesized {
		font-family: 'Bytesized', 'Courier New', monospace !important;
		letter-spacing: 0.05em;
	}

	/* Custom animation class */
	:global(.animate-spin-slow) {
		animation: spin 8s linear infinite;
	}

	/* Pixelation effect */
	.pixelated {
		image-rendering: pixelated;
		image-rendering: -moz-crisp-edges;
		image-rendering: crisp-edges;
		filter: contrast(1.4);
		stroke-width: 2px;
		shape-rendering: crispEdges;
		vector-effect: non-scaling-stroke;
	}
	
	/* Neo-brutalist container hover effect */
	.neo-brutalist-container {
		box-shadow: 8px 8px 0px 0px #000000;
		transform: translate(-4px, -4px);
		transition: transform 0.2s, box-shadow 0.2s;
	}
	
	.neo-brutalist-container:hover {
		transform: translate(-2px, -2px);
		box-shadow: 6px 6px 0px 0px #000000;
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}
</style>
