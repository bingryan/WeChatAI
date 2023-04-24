<template>
	<Form ref="formRef" :model="settingform" :style="{ width: '560px' }">
		<FormItem
			field="openaiKey"
			label="OpenAIKey:"
			:validate-trigger="['change', 'input']">
			<InputPassword
				v-model="settingform.chatgptInfo.openai_key"
				placeholder="your Open AI Key..." />
		</FormItem>
		<FormItem field="temperature" label="temperature" feedback>
			<Input
				v-model="settingform.chatgptInfo.temperature"
				default-value="0.5" />
		</FormItem>
		<FormItem field="top_p" label="top p" feedback>
			<Input v-model="settingform.chatgptInfo.top_p" default-value="1.0" />
		</FormItem>
		<FormItem field="presence_penalty" label="presence" feedback>
			<Input
				v-model="settingform.chatgptInfo.presence_penalty"
				default-value="0.0"
				placeholder="presence penalty(default: 0)" />
		</FormItem>
		<FormItem field="frequency_penalty" label="frequency" feedback>
			<Input
				v-model="settingform.chatgptInfo.frequency_penalty"
				default-value="0.0"
				placeholder="frequency penalty(default: 0)" />
		</FormItem>
		<FormItem field="reply_count" label="reply count" feedback>
			<Input v-model="settingform.chatgptInfo.reply_count" default-value="1" />
		</FormItem>
		<FormItem field="context_size" label="contextSize" feedback>
			<Input v-model="settingform.chatgptInfo.context_size" default-value="2" />
		</FormItem>
		<FormItem field="api_url" label="api url" feedback>
			<Input
				v-model="settingform.chatgptInfo.api_url"
				default-value="https://api.openai.com/v1/chat/completions" />
		</FormItem>
		<FormItem field="engine" label="engine" feedback>
			<Select
				v-model="settingform.chatgptInfo.engine"
				default-value="gpt-3.5-turbo">
				<Option value="gpt-3.5-turbo">gpt-3.5-turbo</Option>
				<Option value="gpt-3.5-turbo-0301">gpt-3.5-turbo-0301</Option>
				<Option value="gpt-4">gpt-4</Option>
				<Option value="gpt-4-32k">gpt-4-32k</Option>
				<Option value="gpt-4-0314">gpt-4-0314</Option>
				<Option value="gpt-4-32k-0314">gpt-4-32k-0314</Option>
			</Select>
		</FormItem>
		<FormItem field="sys message" label="sysMessage" feedback>
			<Textarea
				v-model="settingform.chatgptInfo.system_message"
				show-word-limit />
		</FormItem>
	</Form>
</template>

<script lang="ts" setup>
	import { reactive, toRaw } from 'vue';
	import {
		Form,
		FormItem,
		Input,
		Textarea,
		Select,
		Option,
		InputPassword,
	} from '@arco-design/web-vue';
	import { useAppStore } from '@/store';

	const appStore = useAppStore();

	const settingform = reactive({
		chatgptInfo: appStore.chatgptInfo,
	});

	const saveSettings = async () => {
		const { chatgptInfo } = toRaw(settingform);
		appStore.updateSettings({
			chatgptInfo,
		});
	};

	defineExpose({
		saveSettings,
	});
</script>
