import { useChatStore } from '@/store';

type ThenFunction = () => Promise<void>;

export function useChat(thenFunc?: ThenFunction) {
	const chatStore = useChatStore();

	const addCache = (uuid: number, data: any, dataIndex?: number | null) => {
		chatStore.addCache(uuid, data, dataIndex);
		if (thenFunc) thenFunc();
	};

	return {
		addCache,
	};
}
