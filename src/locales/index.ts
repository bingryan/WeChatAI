import type { App } from 'vue';
import { createI18n } from 'vue-i18n';
import { useAppStoreWithOut } from '@/store';
import en from './en-US';
import cn from './zh-CN';

const appStore = useAppStoreWithOut();

const defaultLocale = appStore.language || 'zh-CN';

const i18n = createI18n({
	locale: defaultLocale,
	fallbackLocale: 'en-US',
	allowComposition: true,
	messages: {
		'en-US': en,
		'zh-CN': cn,
	},
});

export function t(key: string) {
	return i18n.global.t(key);
}

export function setLocale(locale: 'zh-CN' | 'en-US') {
	i18n.global.locale = locale;
}

export function setupI18n(app: App) {
	app.use(i18n);
}

export default i18n;
