import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import adapter from '@sveltejs/adapter-static';
import viteImagemin from 'vite-plugin-imagemin';

export default defineConfig({
	plugins: [
		sveltekit(),
		viteImagemin({
			gifsicle: {
				optimizationLevel: 7,
				interlaced: false
			},
			optipng: {
				optimizationLevel: 7
			},
			mozjpeg: {
				quality: 20
			},
			pngquant: {
				quality: [0.8, 0.9],
				speed: 4
			},
			svgo: {
				plugins: [
					{
						name: 'removeViewBox'
					},
					{
						name: 'removeEmptyAttrs',
						active: false
					}
				]
			}
		})
	],

	kit: {
		adapter: adapter()
	},

	build: {
		rollupOptions: {
			external: ['lodash']
		}
	}
});
