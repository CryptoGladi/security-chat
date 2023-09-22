import solid from 'solid-start/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [solid({ ssr: false })],
	build: {
		chunkSizeWarningLimit: Number.MAX_SAFE_INTEGER,
	}
});
