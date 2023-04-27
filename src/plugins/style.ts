import '@/assets/css/index.css';
import '@/assets/css/github-markdown.css';
import '@/assets/css/font-awesome.min.css';
import '@arco-design/web-vue/dist/arco.css';
import '@imengyu/vue3-context-menu/lib/vue3-context-menu.css';
import ContextMenu from '@imengyu/vue3-context-menu';
import ArcoVue from '@arco-design/web-vue';
import type { App } from 'vue';

export function setupStyle(app: App) {
	app
		.use(ArcoVue, {
			componentPrefix: 'Arco',
		})
		.use(ContextMenu);
}

export default setupStyle;
