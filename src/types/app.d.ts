declare namespace App {
	interface Prompt {
		key: string;
		name: string;
		content: string;
	}
	interface Content {
		userLightBackgroundColor: string;
		userDarkBackgroundColor: string;
		assistantLightBackgroundColor: string;
		assistantDarkBackgroundColor: string;
		userContentType: string;
		assistantContentType: string;
	}

	interface chatgptInfo {
		api_key: string;
		temperature: string;
		top_p: string;
		presence_penalty: string;
		frequency_penalty: string;
		reply_count: string;
		engine: string;
		api_url: string;
		context_size: string;
		system_message: string;
	}
	interface Setting {
		theme: string;
		language: string;
		userAvatarUrl: string;
		assistantAvatarUrl: string;
		sidebar: boolean;
		avtivitybar: boolean;
		globalSettings: boolean;
		chatModelVisible: boolean;
		newChatModal: boolean;
		content: Content;
		device: string;
		chatgptInfo: chatgptInfo;
		[key: string]: unknown;
	}

	interface ChatSetting {
		id: number;
		model: string;
		title: string;
		uptime: number;
		setting: { [key: string]: string };
	}

	interface Chat {
		current: number | null;
		chatSetting: ChatSetting[];
		chatCache: { id: number; uptime: number; data: any }[];
	}
}
