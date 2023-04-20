import { defineStore } from 'pinia';
import useLocalStorage from '@/hooks/useLocalStorage';
import defaultState from '@/config/app.json';
import { store } from '@/store';

const ID = 'app-store';

const { setState, getState } = useLocalStorage(ID, defaultState);

export const useAppStore = defineStore(ID, {
	state: (): App.Setting => getState(),
	actions: {
		updateSettings(partial: Partial<App.Setting>) {
			// @ts-ignore-next-line
			this.$patch(partial);
			setState(this.$state);
		},
		setTheme(theme: 'auto' | 'dark' | 'light') {
			if (this.theme !== theme) {
				this.theme = theme;
				setState(this.$state);
			}
		},
		setLanguage(language: string) {
			if (this.language !== language) {
				this.language = language;
				setState(this.$state);
			}
		},
	},
});

export function useAppStoreWithOut() {
	return useAppStore(store);
}
