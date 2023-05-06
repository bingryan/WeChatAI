import { createPinia } from 'pinia';
import type { App } from 'vue';

export const store = createPinia();

export function setupStore(app: App) {
	app.use(store);
}

export * from './chat';
export * from './app';
export * from './prompt';
