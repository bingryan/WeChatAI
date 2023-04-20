import { computed, watch } from 'vue';
import { useAppStore } from '@/store';
import { useOsTheme } from 'vooks';

export function useTheme() {
	const appStore = useAppStore();

	const osTheme = useOsTheme();

	const isDark = computed(() => {
		if (appStore.theme === 'auto') {
			return osTheme.value === 'dark';
		}
		return appStore.theme === 'dark';
	});

	const theme = computed(() => {
		return appStore.theme;
	});

	const setTheme = (theme: 'auto' | 'dark' | 'light') => {
		appStore.setTheme(theme);
	};

	watch(
		() => isDark.value,
		(dark) => {
			if (dark) {
				// markdown theme
				document.documentElement.classList.remove('light');
				document.documentElement.classList.add('dark');
				// arco-theme
				document.body.setAttribute('arco-theme', 'dark');
				// tauri system theme
				// TODO: app titlebar's theme not changed(must reload)
				// feat: https://github.com/tauri-apps/tauri/issues/5279
			} else {
				// markdown theme
				document.documentElement.classList.remove('dark');
				document.documentElement.classList.add('light');
				// arco-theme
				document.body.removeAttribute('arco-theme');
			}
		},
		{ immediate: true }
	);

	return { theme, setTheme };
}
