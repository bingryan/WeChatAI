<template>
	<div
		class="h-screen py-8 overflow-y-auto bg-white sm:w-64 w-60 dark:bg-[#191919]">
		<h2
			class="px-5 text-lg font-medium text-gray-800 dark:text-white text-center hover:bg-green-50 dark:hover:bg-green-800 cursor-pointer"
			@click="handleAdd">
			<IconPlus />
		</h2>

		<!-- new chat modal -->
		<Modal
			v-model:visible="chatModelVisible"
			draggable
			@ok="handleOk"
			@cancel="handleCancel">
			<template v-if="newChatModal" #title>
				{{ $t('common.newChat') }}</template
			>
			<template v-else #title> {{ $t('common.editChat') }} </template>
			<!-- Modal body -->
			<Form :model="chatInfo">
				<FormItem field="title" label="chat title" feedback>
					<Input
						v-model="chatInfo.title"
						default-value="New Chat"
						placeholder="New CHat" />
				</FormItem>
				<FormItem
					field="apiKey"
					label="apiKey:"
					:validate-trigger="['change', 'input']">
					<InputPassword
						v-model="chatInfo.setting.api_key"
						placeholder="your Azure/Open AI Key..." />
				</FormItem>
				<FormItem field="temperature" label="temperature" feedback>
					<Input v-model="chatInfo.setting.temperature" default-value="0.5" />
				</FormItem>
				<FormItem field="top_p" label="top p" feedback>
					<Input v-model="chatInfo.setting.top_p" default-value="1.0" />
				</FormItem>
				<FormItem field="presence_penalty" label="presence" feedback>
					<Input
						v-model="chatInfo.setting.presence_penalty"
						default-value="0.0"
						placeholder="presence penalty(default: 0)" />
				</FormItem>
				<FormItem field="frequency_penalty" label="frequency" feedback>
					<Input
						v-model="chatInfo.setting.frequency_penalty"
						default-value="0.0"
						placeholder="frequency penalty(default: 0)" />
				</FormItem>
				<FormItem field="reply_count" label="reply count" feedback>
					<Input v-model="chatInfo.setting.reply_count" default-value="1" />
				</FormItem>
				<FormItem field="context_size" label="context size" feedback>
					<Input v-model="chatInfo.setting.context_size" default-value="2" />
				</FormItem>
				<FormItem field="api_url" label="api url" feedback>
					<Input
						v-model="chatInfo.setting.api_url"
						default-value="https://api.openai.com/v1/chat/completions" />
				</FormItem>
				<FormItem field="section" label="engine" feedback>
					<Select
						v-model="chatInfo.setting.engine"
						default-value="gpt-3.5-turbo">
						<Option value="gpt-3.5-turbo">gpt-3.5-turbo</Option>
						<Option value="gpt-3.5-turbo-0301">gpt-3.5-turbo-0301</Option>
						<Option value="gpt-4">gpt-4</Option>
						<Option value="gpt-4-32k">gpt-4-32k</Option>
						<Option value="gpt-4-0314">gpt-4-0314</Option>
						<Option value="gpt-4-32k-0314">gpt-4-32k-0314</Option>
					</Select>
				</FormItem>
				<FormItem field="sys message" label="sys message" feedback>
					<Textarea v-model="chatInfo.setting.system_message" show-word-limit />
				</FormItem>
			</Form>
		</Modal>
		<div class="space-y-2">
			<List />
		</div>
	</div>
</template>

<script setup lang="ts">
	import { ref, reactive, toRaw, computed } from 'vue';
	import { IconPlus } from '@arco-design/web-vue/es/icon';
	import {
		Modal,
		Select,
		Option,
		Input,
		InputPassword,
		Form,
		FormItem,
		Textarea,
	} from '@arco-design/web-vue';
	import { useAppStore, useChatStore } from '@/store';
	import List from './List.vue';

	const chatStore = useChatStore();
	const appStore = useAppStore();

	const chatModelVisible = computed(() => {
		return appStore.chatModelVisible;
	});

	const newChatModal = computed(() => {
		return appStore.newChatModal;
	});
	// TODO: fix warn
	const chatInfo = computed(() => {
		const cs = chatStore.getChatSettingByCurrent;
		if (newChatModal.value || !cs) {
			return reactive({
				id: Date.now(),
				model: 'chatgpt',
				title: 'New Chat',
				uptime: Date.now(),
				setting: appStore.chatgptInfo,
			});
		}
		return reactive({
			id: cs.id,
			title: cs.title,
			model: cs.model,
			uptime: cs.uptime,
			setting: cs.setting,
		});
	});

	const handleAdd = () => {
		appStore.updateSettings({
			chatModelVisible: true,
			newChatModal: true,
		});
	};

	async function handleOk() {
		if (newChatModal.value) {
			// add new
			chatStore.setChatSetting(chatInfo.value);
		} else {
			// update
			chatStore.updateChatSettingById(chatInfo.value);
		}

		appStore.updateSettings({
			chatModelVisible: false,
			newChatModal: false,
		});
	}

	const handleCancel = () => {
		appStore.updateSettings({
			chatModelVisible: false,
			newChatModal: false,
		});
	};
</script>
