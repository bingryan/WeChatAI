import path from 'path';
import { defineConfig, ConfigEnv } from 'vite';
import vue from '@vitejs/plugin-vue';
import { viteMockServe } from 'vite-plugin-mock';
import VueDevTools from 'vite-plugin-vue-devtools';
import { createSvgIconsPlugin } from 'vite-plugin-svg-icons';

// https://vitejs.dev/config/
export default defineConfig(async ({ command }: ConfigEnv) => {
	return {
		resolve: {
			alias: {
				'@': path.resolve(process.cwd(), 'src'),
			},
		},
		plugins: [
			vue(),
			VueDevTools(),
			createSvgIconsPlugin({
				iconDirs: [path.resolve(process.cwd(), 'src/assets/icons')],
			}),
			viteMockServe({
				mockPath: 'mock',
				localEnabled: command === 'serve',
			}),
		],

		// Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
		// prevent vite from obscuring rust errors
		clearScreen: false,
		// tauri expects a fixed port, fail if that port is not available
		server: {
			port: 1420,
			strictPort: true,
			proxy: {
				'/api': {
					target: 'http://localhost:19999/',
					changeOrigin: true,
					rewrite: (path) => path.replace(/^\/api/, ''),
				},
			},
		},

		// to make use of `TAURI_DEBUG` and other env variables
		// https://tauri.studio/v1/api/config#buildconfig.beforedevcommand
		envPrefix: ['VITE_', 'TAURI_'],
		build: {
			// Tauri supports es2021
			target:
				process.env.TAURI_PLATFORM === 'windows' ? 'chrome105' : 'safari13',
			// don't minify for debug builds
			minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
			// produce sourcemaps for debug builds
			sourcemap: !!process.env.TAURI_DEBUG,
		},
	};
});
