import { LocalJsonStorage, localCache } from './local';
import { WindowJsonStorage, wls, wss } from './window';

// VITE_LOCAL_STRAGE_TYPE = 'window.localStorage' | 'window.sessionStorage' | 'local'
const defineStorageType = import.meta.env.VITE_LOCAL_STRAGE_TYPE;

type StorageClass = LocalJsonStorage | WindowJsonStorage;
type StorageTypeList =
	| 'window.localStorage'
	| 'window.sessionStorage'
	| 'local';

function isDefineStorage(tp: string): tp is StorageTypeList {
	return ['window.localStorage', 'window.sessionStorage', 'local'].includes(tp);
}

function createStorage(): StorageClass {
	switch (defineStorageType) {
		case 'window.localStorage':
			return wls;
		case 'window.sessionStorage':
			return wss;
		case 'local':
			return localCache;
		default:
			throw new Error(`not support ${defineStorageType} storage`);
	}
}

export const localStorage = createStorage();
