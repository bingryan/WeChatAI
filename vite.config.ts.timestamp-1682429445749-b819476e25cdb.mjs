// vite.config.ts
import path from "path";
import { defineConfig } from "file:///Users/legotime/VScode/tauri-workspace/WeChatAI/node_modules/.pnpm/registry.npmmirror.com+vite@4.3.1_c5ikhegrqpr7fpvja63jstlkma/node_modules/vite/dist/node/index.js";
import vue from "file:///Users/legotime/VScode/tauri-workspace/WeChatAI/node_modules/.pnpm/registry.npmmirror.com+@vitejs+plugin-vue@4.1.0_vite@4.3.1+vue@3.2.47/node_modules/@vitejs/plugin-vue/dist/index.mjs";
import { viteMockServe } from "file:///Users/legotime/VScode/tauri-workspace/WeChatAI/node_modules/.pnpm/registry.npmmirror.com+vite-plugin-mock@2.9.8_mockjs@1.1.0+vite@4.3.1/node_modules/vite-plugin-mock/dist/index.js";
var vite_config_default = defineConfig(async ({ command }) => {
  return {
    resolve: {
      alias: {
        "@": path.resolve(process.cwd(), "src")
      }
    },
    plugins: [
      vue(),
      viteMockServe({
        mockPath: "mock",
        localEnabled: command === "serve"
      })
    ],
    // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
    // prevent vite from obscuring rust errors
    clearScreen: false,
    // tauri expects a fixed port, fail if that port is not available
    server: {
      port: 1420,
      strictPort: true,
      proxy: {
        "/api": {
          target: "http://localhost:19999/",
          changeOrigin: true,
          rewrite: (path2) => path2.replace(/^\/api/, "")
        }
      }
    },
    // to make use of `TAURI_DEBUG` and other env variables
    // https://tauri.studio/v1/api/config#buildconfig.beforedevcommand
    envPrefix: ["VITE_", "TAURI_"],
    build: {
      // Tauri supports es2021
      target: process.env.TAURI_PLATFORM === "windows" ? "chrome105" : "safari13",
      // don't minify for debug builds
      minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
      // produce sourcemaps for debug builds
      sourcemap: !!process.env.TAURI_DEBUG
    }
  };
});
export {
  vite_config_default as default
};
//# sourceMappingURL=data:application/json;base64,ewogICJ2ZXJzaW9uIjogMywKICAic291cmNlcyI6IFsidml0ZS5jb25maWcudHMiXSwKICAic291cmNlc0NvbnRlbnQiOiBbImNvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9kaXJuYW1lID0gXCIvVXNlcnMvbGVnb3RpbWUvVlNjb2RlL3RhdXJpLXdvcmtzcGFjZS9XZUNoYXRBSVwiO2NvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9maWxlbmFtZSA9IFwiL1VzZXJzL2xlZ290aW1lL1ZTY29kZS90YXVyaS13b3Jrc3BhY2UvV2VDaGF0QUkvdml0ZS5jb25maWcudHNcIjtjb25zdCBfX3ZpdGVfaW5qZWN0ZWRfb3JpZ2luYWxfaW1wb3J0X21ldGFfdXJsID0gXCJmaWxlOi8vL1VzZXJzL2xlZ290aW1lL1ZTY29kZS90YXVyaS13b3Jrc3BhY2UvV2VDaGF0QUkvdml0ZS5jb25maWcudHNcIjtpbXBvcnQgcGF0aCBmcm9tICdwYXRoJztcbmltcG9ydCB7IGRlZmluZUNvbmZpZywgQ29uZmlnRW52IH0gZnJvbSAndml0ZSc7XG5pbXBvcnQgdnVlIGZyb20gJ0B2aXRlanMvcGx1Z2luLXZ1ZSc7XG5pbXBvcnQgeyB2aXRlTW9ja1NlcnZlIH0gZnJvbSAndml0ZS1wbHVnaW4tbW9jayc7XG5cbi8vIGh0dHBzOi8vdml0ZWpzLmRldi9jb25maWcvXG5leHBvcnQgZGVmYXVsdCBkZWZpbmVDb25maWcoYXN5bmMgKHsgY29tbWFuZCB9OiBDb25maWdFbnYpID0+IHtcblx0cmV0dXJuIHtcblx0XHRyZXNvbHZlOiB7XG5cdFx0XHRhbGlhczoge1xuXHRcdFx0XHQnQCc6IHBhdGgucmVzb2x2ZShwcm9jZXNzLmN3ZCgpLCAnc3JjJyksXG5cdFx0XHR9LFxuXHRcdH0sXG5cdFx0cGx1Z2luczogW1xuXHRcdFx0dnVlKCksXG5cdFx0XHR2aXRlTW9ja1NlcnZlKHtcblx0XHRcdFx0bW9ja1BhdGg6ICdtb2NrJyxcblx0XHRcdFx0bG9jYWxFbmFibGVkOiBjb21tYW5kID09PSAnc2VydmUnLFxuXHRcdFx0fSksXG5cdFx0XSxcblxuXHRcdC8vIFZpdGUgb3B0aW9ucyB0YWlsb3JlZCBmb3IgVGF1cmkgZGV2ZWxvcG1lbnQgYW5kIG9ubHkgYXBwbGllZCBpbiBgdGF1cmkgZGV2YCBvciBgdGF1cmkgYnVpbGRgXG5cdFx0Ly8gcHJldmVudCB2aXRlIGZyb20gb2JzY3VyaW5nIHJ1c3QgZXJyb3JzXG5cdFx0Y2xlYXJTY3JlZW46IGZhbHNlLFxuXHRcdC8vIHRhdXJpIGV4cGVjdHMgYSBmaXhlZCBwb3J0LCBmYWlsIGlmIHRoYXQgcG9ydCBpcyBub3QgYXZhaWxhYmxlXG5cdFx0c2VydmVyOiB7XG5cdFx0XHRwb3J0OiAxNDIwLFxuXHRcdFx0c3RyaWN0UG9ydDogdHJ1ZSxcblx0XHRcdHByb3h5OiB7XG5cdFx0XHRcdCcvYXBpJzoge1xuXHRcdFx0XHRcdHRhcmdldDogJ2h0dHA6Ly9sb2NhbGhvc3Q6MTk5OTkvJyxcblx0XHRcdFx0XHRjaGFuZ2VPcmlnaW46IHRydWUsXG5cdFx0XHRcdFx0cmV3cml0ZTogKHBhdGgpID0+IHBhdGgucmVwbGFjZSgvXlxcL2FwaS8sICcnKSxcblx0XHRcdFx0fSxcblx0XHRcdH0sXG5cdFx0fSxcblxuXHRcdC8vIHRvIG1ha2UgdXNlIG9mIGBUQVVSSV9ERUJVR2AgYW5kIG90aGVyIGVudiB2YXJpYWJsZXNcblx0XHQvLyBodHRwczovL3RhdXJpLnN0dWRpby92MS9hcGkvY29uZmlnI2J1aWxkY29uZmlnLmJlZm9yZWRldmNvbW1hbmRcblx0XHRlbnZQcmVmaXg6IFsnVklURV8nLCAnVEFVUklfJ10sXG5cdFx0YnVpbGQ6IHtcblx0XHRcdC8vIFRhdXJpIHN1cHBvcnRzIGVzMjAyMVxuXHRcdFx0dGFyZ2V0OlxuXHRcdFx0XHRwcm9jZXNzLmVudi5UQVVSSV9QTEFURk9STSA9PT0gJ3dpbmRvd3MnID8gJ2Nocm9tZTEwNScgOiAnc2FmYXJpMTMnLFxuXHRcdFx0Ly8gZG9uJ3QgbWluaWZ5IGZvciBkZWJ1ZyBidWlsZHNcblx0XHRcdG1pbmlmeTogIXByb2Nlc3MuZW52LlRBVVJJX0RFQlVHID8gJ2VzYnVpbGQnIDogZmFsc2UsXG5cdFx0XHQvLyBwcm9kdWNlIHNvdXJjZW1hcHMgZm9yIGRlYnVnIGJ1aWxkc1xuXHRcdFx0c291cmNlbWFwOiAhIXByb2Nlc3MuZW52LlRBVVJJX0RFQlVHLFxuXHRcdH0sXG5cdH07XG59KTtcbiJdLAogICJtYXBwaW5ncyI6ICI7QUFBK1QsT0FBTyxVQUFVO0FBQ2hWLFNBQVMsb0JBQStCO0FBQ3hDLE9BQU8sU0FBUztBQUNoQixTQUFTLHFCQUFxQjtBQUc5QixJQUFPLHNCQUFRLGFBQWEsT0FBTyxFQUFFLFFBQVEsTUFBaUI7QUFDN0QsU0FBTztBQUFBLElBQ04sU0FBUztBQUFBLE1BQ1IsT0FBTztBQUFBLFFBQ04sS0FBSyxLQUFLLFFBQVEsUUFBUSxJQUFJLEdBQUcsS0FBSztBQUFBLE1BQ3ZDO0FBQUEsSUFDRDtBQUFBLElBQ0EsU0FBUztBQUFBLE1BQ1IsSUFBSTtBQUFBLE1BQ0osY0FBYztBQUFBLFFBQ2IsVUFBVTtBQUFBLFFBQ1YsY0FBYyxZQUFZO0FBQUEsTUFDM0IsQ0FBQztBQUFBLElBQ0Y7QUFBQTtBQUFBO0FBQUEsSUFJQSxhQUFhO0FBQUE7QUFBQSxJQUViLFFBQVE7QUFBQSxNQUNQLE1BQU07QUFBQSxNQUNOLFlBQVk7QUFBQSxNQUNaLE9BQU87QUFBQSxRQUNOLFFBQVE7QUFBQSxVQUNQLFFBQVE7QUFBQSxVQUNSLGNBQWM7QUFBQSxVQUNkLFNBQVMsQ0FBQ0EsVUFBU0EsTUFBSyxRQUFRLFVBQVUsRUFBRTtBQUFBLFFBQzdDO0FBQUEsTUFDRDtBQUFBLElBQ0Q7QUFBQTtBQUFBO0FBQUEsSUFJQSxXQUFXLENBQUMsU0FBUyxRQUFRO0FBQUEsSUFDN0IsT0FBTztBQUFBO0FBQUEsTUFFTixRQUNDLFFBQVEsSUFBSSxtQkFBbUIsWUFBWSxjQUFjO0FBQUE7QUFBQSxNQUUxRCxRQUFRLENBQUMsUUFBUSxJQUFJLGNBQWMsWUFBWTtBQUFBO0FBQUEsTUFFL0MsV0FBVyxDQUFDLENBQUMsUUFBUSxJQUFJO0FBQUEsSUFDMUI7QUFBQSxFQUNEO0FBQ0QsQ0FBQzsiLAogICJuYW1lcyI6IFsicGF0aCJdCn0K
