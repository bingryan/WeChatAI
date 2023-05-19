import '@/assets/css/index.css';
import '@/assets/css/github-markdown.css';
import '@/assets/css/font-awesome.min.css';
import '@arco-design/web-vue/dist/arco.css';
import '@imengyu/vue3-context-menu/lib/vue3-context-menu.css';
import ContextMenu from '@imengyu/vue3-context-menu';
import '@/assets/css/setting-box.less';
import ArcoVue from '@arco-design/web-vue';
import type { App } from 'vue';
// eslint-disable-next-line import/no-unresolved
import 'virtual:svg-icons-register';

export function setupStyle(app: App) {
	app
		.use(ArcoVue, {
			componentPrefix: 'Arco',
		})
		.use(ContextMenu);
}

export default setupStyle;
