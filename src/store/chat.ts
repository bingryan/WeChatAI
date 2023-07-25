import { defineStore } from 'pinia';
import { router } from '@/router';
import useLocalStorage from '@/hooks/useLocalStorage';
import defaultState from '@/config/chat.json';

const ID = 'chat-store';

const { setState, getState } = useLocalStorage(ID, defaultState);

export const useChatStore = defineStore(ID, {
	state: (): App.Chat => getState(),

	getters: {
		getChatSetting(state: App.Chat) {
			return state.chatSetting;
		},
		getChatSettingByCurrent(state: App.Chat) {
			const index = state.chatSetting.findIndex(
				(item) => item.id === state.current
			);
			if (index !== -1) return state.chatSetting[index];
			return null;
		},

		getChatSettingById(state: App.Chat) {
			return (id?: number) => {
				if (id) return state.chatSetting.find((item) => item.id === id);
				return state.chatSetting.find((item) => item.id === state.current);
			};
		},

		getCacheById(state: App.Chat) {
			return (id?: number) => {
				if (id)
					return state.chatCache.find((item) => item.id === id)?.data ?? [];
				return (
					state.chatCache.find((item) => item.id === state.current)?.data ?? []
				);
			};
		},
		getChatSettingIndex(state: App.Chat) {
			return (id: number) => {
				return state.chatSetting.findIndex((item) => item.id === id);
			};
		},
		getCacheIndex(state: App.Chat) {
			return (id: number) => {
				return state.chatCache.findIndex((item) => item.id === id);
			};
		},
	},

	actions: {
		/**
		 * switch to input id
		 * @param id
		 * @returns
		 */
		async setCurrent(id: number) {
			this.current = id;
			return this.reloadRoute(id);
		},

		/**
		 * set chat page's setting
		 * @param chatSetting
		 */
		setChatSetting(chatSetting: App.ChatSetting) {
			this.chatSetting.unshift(chatSetting);
			this.current = chatSetting.id;
			this.reloadRoute(chatSetting.id);
		},

		updateChatSettingById(chatSetting: App.ChatSetting) {
			const index = this.getChatSettingIndex(chatSetting.id);
			this.chatSetting[index] = chatSetting;
			this.current = chatSetting.id;
			this.reloadRoute(chatSetting.id);
		},

		/**
		 * update all chat setting(for draggable)
		 * @param chatSetting
		 */
		async updateChatSetting(chatSetting: App.ChatSetting[]) {
			this.chatSetting = chatSetting;
			await setState(this.$state);
		},
		/**
		 * remove chat setting,then it's cache will also be removed
		 * @param chatSetting
		 * @returns
		 */
		async removeChatSetting(id: number) {
			const chatSettingIndex = this.getChatSettingIndex(id);
			const cacheIndex = this.getCacheIndex(id);

			this.chatSetting.splice(chatSettingIndex, 1);
			this.chatCache.splice(cacheIndex, 1);

			// Jump to the  first
			const first = this.chatSetting[0];
			if (first) {
				this.current = first.id;
			} else {
				this.current = null;
			}

			this.reloadRoute(this.current);
		},
		/**
		 * add/update cache
		 * @param id
		 * @param data
		 * @param dataIndex: if not null, it means updating the data
		 */
		async addCache(id: number, data: any, dataIndex?: number | null) {
			const index = this.getCacheIndex(id);

			const uptime = new Date().getTime();
			// first chat data
			if (index === -1) {
				this.chatCache.push({ id, uptime, data: [data] });
				this.reloadRoute(id);
				return;
			}

			// update cache data
			if (dataIndex) {
				// first chat data
				this.chatCache[index].data[dataIndex] = {
					...this.chatCache[index].data[dataIndex],
					...data,
				};
				this.chatCache[index].uptime = uptime;
				this.reloadRoute(id);
				return;
			}
			// add new chat data
			this.chatCache[index].data.push(data);
			this.chatCache[index].uptime = uptime;
			this.reloadRoute(id);
		},
		/**
		 * get user's chat context by id and index
		 * @param id chat id
		 * @param dataIndex data index
		 */
		selectContextQA(id: number, dataIndex: number) {
			const cacheIndex = this.getCacheIndex(id);
			const contextData = [];
			if (cacheIndex !== -1) {
				const data = this.chatCache[cacheIndex].data[dataIndex];
				contextData.push(data);
				if (data.role === 'user') {
					contextData.push(this.chatCache[cacheIndex].data[dataIndex + 1]);
				} else {
					contextData.unshift(this.chatCache[cacheIndex].data[dataIndex - 1]);
				}
			}
			return contextData;
		},

		/**
		 * remove single record at cache by id
		 * @param id
		 */
		async removeCache(id: number, dataIndex: number) {
			const cacheIndex = this.getCacheIndex(id);
			if (cacheIndex !== -1) {
				this.chatCache[cacheIndex].data.splice(dataIndex, 1);
			}
			this.reloadRoute(id);
		},
		/**
		 * clear all chat cache by chat id
		 * @param id
		 */
		async clearCache(id: number) {
			const cacheIndex = this.getCacheIndex(id);
			if (cacheIndex !== -1) {
				this.chatCache.splice(cacheIndex, 1);
			}
			this.reloadRoute(id);
		},

		async copyChatSetting(id: number) {
			const index = this.getChatSettingIndex(id);
			const chatSetting = this.chatSetting[index];
			const newChatSetting = { ...chatSetting, id: new Date().getTime() };
			this.chatSetting.unshift(newChatSetting);
			this.current = newChatSetting.id;
			this.reloadRoute(newChatSetting.id);
		},

		async reloadRoute(id?: number | null) {
			setState(this.$state);
			await router.push({ name: 'chat', params: { id } });
		},

		// reset() {
		// 	try {
		// 		setState(this.$state);
		// 	} catch (error) {
		// 		console.error(error);
		// 	} finally {
		// 		this.$reset();
		// 	}
		// },
		// reload(){
		// 	setState(this.$state);
		// }
	},
});

export default useChatStore;
