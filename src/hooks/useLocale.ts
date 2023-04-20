import { computed } from 'vue';
import enUS from '@arco-design/web-vue/es/locale/lang/en-us';
import zhCN from '@arco-design/web-vue/es/locale/lang/zh-cn';
import { useAppStore } from '@/store';
import { setLocale } from '@/locales';

export function useLocale() {
	const appStore = useAppStore();

	const locale = computed(() => {
		switch (appStore.language) {
			case 'en-US':
				setLocale('en-US');
				return enUS;
			case 'zh-CN':
				setLocale('zh-CN');
				return zhCN;
			default:
				setLocale('zh-CN');
				return zhCN;
		}
	});

	return { locale };
}
