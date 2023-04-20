import { Store } from 'tauri-plugin-store-api';

export class LocalJsonStorage {
	private store: Store;

	constructor(path: string) {
		this.store = new Store(path);
	}

	public async set<T = any>(key: string, value: T) {
		await this.store.set(key, value);
	}

	public async get(key: string) {
		const value = await this.store.get(key);
		return value || undefined;
	}

	public async remove(key: string) {
		await this.store.delete(key);
	}

	public async clear() {
		this.store.clear();
	}
}

export const localCache = new LocalJsonStorage('./cahce.json');
