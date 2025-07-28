import tailwindcss from '@tailwindcss/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import wasm from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await';

export default defineConfig({
    plugins: [tailwindcss(), sveltekit(), wasm(), topLevelAwait()],
    preview: {
        port: 4174, // make sure this matches your NGINX proxy_pass port
        allowedHosts: ['typhe.us']
        // If you want to allow www.typhe.us or subdomains, add: allowedHosts: ['typhe.us', '.typhe.us']
    }
});
