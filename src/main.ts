import App from '@/App.vue';
import { createApp } from 'vue';
import { setupStore } from '@/store';
import { setupRouter } from '@/router';
import { setupStyle } from '@/plugins/style';
import { setupI18n } from '@/locales';

async function bootstrap() {
	const app = createApp(App);
	setupStyle(app);
	setupStore(app);
	setupI18n(app);
	await setupRouter(app);
	app.mount('#app');
}

bootstrap();
