import { localStorage } from '@/utils/storage';

export default function useStateStorage(storageKey: string, defaultState: any) {
	function getState() {
		const localState: any | undefined = localStorage.get(storageKey);
		return { ...defaultState, ...localState };
	}

	function setState(state: any) {
		localStorage.set(storageKey, state);
	}

	return { setState, getState };
}
