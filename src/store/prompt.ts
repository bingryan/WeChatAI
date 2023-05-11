import { defineStore } from 'pinia';
import useLocalStorage from '@/hooks/useLocalStorage';
import defaultState from '@/config/prompt.json';

const ID = 'prompt-store';

const { setState, getState } = useLocalStorage(ID, defaultState);

export const usePromptStore = defineStore(ID, {
	state: (): App.Prompt => getState(),
	getters: {
		getTemplate(state: App.Prompt) {
			return state.template;
		},
		getTemplateByKey(state: App.Prompt) {
			return (key: string) => {
				return state.template.find((item) => item.key === key);
			};
		},
		getTemplateByName(state: App.Prompt) {
			return (name: string) => {
				return state.template.find((item) => item.name === name);
			};
		},
	},
	actions: {
		updatePrompt(partial: Partial<App.Prompt>) {
			// @ts-ignore-next-line
			this.$patch(partial);
			setState(this.$state);
		},

		addTemplate(template: App.PromptTemplate) {
			// upset
			const index = this.template.findIndex(
				(item) => item.name === template.name
			);
			if (index >= 0) {
				this.template[index] = template;
			} else {
				this.template.unshift(template);
			}
			setState(this.$state);
		},

		updateTemplate(template: App.PromptTemplate) {
			this.template = this.template.map((item) => {
				if (item.key === template.key) {
					return template;
				}
				return item;
			});
			setState(this.$state);
		},
		removeTemplate(key: string) {
			this.template = this.template.filter((item) => item.key !== key);
			setState(this.$state);
		},
	},
});

export default usePromptStore;
