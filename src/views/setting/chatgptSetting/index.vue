<template>
	<!-- layout="vertical"  -->
	<Form ref="formRef" :model="settingform">
		<div class="text-base font-semibold mb-2">Chatgpt Config</div>
		<LabelBox title="OpenAIKey" desc="">
			<div class="w-96">
				<FormItem field="openaiKey" :validate-trigger="['change', 'input']">
					<InputPassword
						v-model="settingform.chatgptInfo.api_key"
						placeholder="Your OpenAI API/Azure OpenAI Key..." />
				</FormItem>
			</div>
		</LabelBox>

		<LabelBox title="api url">
			<div class="w-96">
				<FormItem field="api_url" feedback>
					<Input
						v-model="settingform.chatgptInfo.api_url"
						default-value="https://api.openai.com/v1/chat/completions" />
				</FormItem>
			</div>
		</LabelBox>

		<LabelBox
			title="temperature"
			desc="What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.">
			<div class="w-36">
				<FormItem field="temperature" feedback>
					<Input
						v-model="settingform.chatgptInfo.temperature"
						default-value="0.5" />
				</FormItem>
			</div>
		</LabelBox>
		<LabelBox
			title="top p"
			desc="An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.">
			<div class="w-36">
				<FormItem field="top_p" feedback>
					<Input v-model="settingform.chatgptInfo.top_p" default-value="1.0" />
				</FormItem>
			</div>
		</LabelBox>
		<LabelBox
			title="presence"
			desc="Number between -2.0 and 2.0. Positive values penalize new tokens based on whether they appear in the text so far, increasing the model's likelihood to talk about new topics.">
			<div class="w-36">
				<FormItem field="presence_penalty" feedback>
					<Input
						v-model="settingform.chatgptInfo.presence_penalty"
						default-value="0.0"
						placeholder="presence penalty(default: 0)" />
				</FormItem>
			</div>
		</LabelBox>
		<LabelBox
			title="frequency"
			desc="Number between -2.0 and 2.0. Positive values penalize new tokens based on their existing frequency in the text so far, decreasing the model's likelihood to repeat the same line verbatim.">
			<div class="w-36">
				<FormItem field="frequency_penalty" feedback>
					<Input
						v-model="settingform.chatgptInfo.frequency_penalty"
						default-value="0.0"
						placeholder="frequency penalty(default: 0)" />
				</FormItem>
			</div>
		</LabelBox>
		<LabelBox title="reply count" desc="default 1">
			<div class="w-36">
				<FormItem field="reply_count" feedback>
					<Input
						v-model="settingform.chatgptInfo.reply_count"
						default-value="1" />
				</FormItem>
			</div>
		</LabelBox>
		<LabelBox title="context size" desc="chat records send to chatgpt">
			<div class="w-36">
				<FormItem field="context_size" feedback>
					<Input
						v-model="settingform.chatgptInfo.context_size"
						default-value="2" />
				</FormItem>
			</div>
		</LabelBox>

		<LabelBox
			title="engine"
			desc="These endpoints describe and provide access to the various engines available in the API.">
			<div class="w-36">
				<FormItem field="engine" feedback>
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
			</div>
		</LabelBox>
		<LabelBox
			title="system message"
			desc="gpt-3.5-turbo-0301 does not always pay strong attention to system messages. Future models will be trained to pay stronger attention to system messages.">
			<div class="w-96">
				<FormItem field="sys message" feedback>
					<Textarea
						v-model="settingform.chatgptInfo.system_message"
						show-word-limit
						auto-size />
				</FormItem>
			</div>
		</LabelBox>
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
	import LabelBox from '../components/labelBox/index.vue';

	const appStore = useAppStore();

	const settingform = reactive({
		chatgptInfo: appStore.chatgptInfo,
	});
</script>
