type StorageType = 'localStorage' | 'sessionStorage';

export class WindowJsonStorage {
	private storageType: StorageType;

	constructor(storageType: StorageType) {
		this.storageType = storageType;
	}

	public set<T = any>(key: string, value: T) {
		const strValue = JSON.stringify(value);
		window[this.storageType].setItem(key, strValue);
	}

	public get(key: string) {
		const value = window[this.storageType].getItem(key);
		if (value) {
			try {
				return JSON.parse(value);
			} catch {
				return null;
			}
		}

		return null;
	}

	public remove(key: string) {
		window[this.storageType].removeItem(key);
	}

	public clear() {
		window[this.storageType].clear();
	}
}

export const wls = new WindowJsonStorage('localStorage');
export const wss = new WindowJsonStorage('sessionStorage');
