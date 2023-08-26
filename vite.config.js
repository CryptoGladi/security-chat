import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import adapter from '@sveltejs/adapter-static';
import { ViteImageOptimizer } from 'vite-plugin-image-optimizer';

export default defineConfig({
	plugins: [sveltekit(), ViteImageOptimizer({})],

	kit: {
		adapter: adapter()
	},

	build: {
		rollupOptions: {
			external: ['lodash']
		}
	}
});
