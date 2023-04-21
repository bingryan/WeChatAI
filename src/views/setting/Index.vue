<template>
	<Drawer
		:width="600"
		unmount-on-close
		:mask-closable="false"
		:visible="visible"
		:cancel-text="$t('settings.reset')"
		:ok-text="$t('settings.save')"
		@ok="saveSettings"
		@cancel="reset">
		<template #title> {{ $t('settings.title') }} </template>
		<Form ref="formRef" :model="settingform" :style="{ width: '560px' }">
			<div class="ml-14">
				<Upload
					ref="uploadRef"
					action="/"
					:auto-upload="false"
					:file-list="file ? [file] : []"
					:show-file-list="false"
					@change="onFileChange">
					<template #upload-button>
						<div
							:class="`arco-upload-list-item${
								file && file.status === 'error'
									? ' arco-upload-list-item-error'
									: ''
							}`">
							<div
								v-if="file && file.url"
								class="arco-upload-list-picture custom-upload-avatar">
								<img :src="file.url" />
								<div class="arco-upload-list-picture-mask">
									<IconEdit />
								</div>
								<Progress
									v-if="file.status === 'uploading'"
									:percent="file.percent"
									type="circle"
									size="mini"
									:style="{
										position: 'absolute',
										left: '50%',
										top: '50%',
										transform: 'translateX(-50%) translateY(-50%)',
									}" />
							</div>
							<div v-else class="arco-upload-picture-card">
								<div class="arco-upload-picture-card-text">
									<img :src="userImageUrl" class="w-20 h-20 object-cover" />
								</div>
							</div>
						</div>
					</template>
				</Upload>
			</div>
			<FormItem label="Theme:">
				<RadioGroup v-model="settingform.theme" @change="changeTheme">
					<Radio value="light">
						<IconSunFill size="26" />
					</Radio>
					<Radio value="dark">
						<IconMoonFill size="26" />
					</Radio>
					<Radio value="auto">
						<IconComputer size="26" />
					</Radio>
				</RadioGroup>
			</FormItem>
			<FormItem
				field="userBackgroundColor"
				:label="$t('settings.userBackgroundColor')">
				<IconSunFill size="26" />
				<input
					v-model="settingform.content.userLightBackgroundColor"
					type="color"
					class="mr-12" />
				<IconMoonFill size="26" />
				<input
					v-model="settingform.content.userDarkBackgroundColor"
					type="color" />
			</FormItem>
			<FormItem
				field="userBackgroundColor"
				:label="$t('settings.assistantBackgroundColor')">
				<IconSunFill size="26" />
				<input
					v-model="settingform.content.assistantLightBackgroundColor"
					type="color"
					class="mr-12" />
				<IconMoonFill size="26" />
				<input
					v-model="settingform.content.assistantDarkBackgroundColor"
					type="color" />
			</FormItem>
			<FormItem
				field="userContentType"
				:label="$t('settings.userContentType')"
				feedback>
				<Select
					v-model="settingform.content.userContentType"
					default-value="raw">
					<Option value="markdown">markdown</Option>
					<Option value="raw">raw</Option>
				</Select>
			</FormItem>
			<FormItem
				field="assistantContentType"
				:label="$t('settings.assistantContentType')"
				feedback>
				<Select
					v-model="settingform.content.assistantContentType"
					default-value="markdown">
					<Option value="markdown">markdown</Option>
					<Option value="raw">raw</Option>
				</Select>
			</FormItem>
			<FormItem field="locale" :label="$t('settings.language')">
				<Select v-model="settingform.language" @change="changeLanguage">
					<Option value="en-US">English</Option>
					<Option value="zh-CN">简体中文</Option>
				</Select>
			</FormItem>

			<Collapse>
				<CollapseItem :header="$t('settings.chatgptConfig')">
					<FormItem
						field="apiKey"
						label="apiKey:"
						:validate-trigger="['change', 'input']">
						<InputPassword
							v-model="settingform.chatgptInfo.api_key"
							placeholder="your Azure/Open AI Key..." />
					</FormItem>
					<FormItem field="temperature" label="temperature" feedback>
						<Input
							v-model="settingform.chatgptInfo.temperature"
							default-value="0.5" />
					</FormItem>
					<FormItem field="top_p" label="top p" feedback>
						<Input
							v-model="settingform.chatgptInfo.top_p"
							default-value="1.0" />
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
						<Input
							v-model="settingform.chatgptInfo.reply_count"
							default-value="1" />
					</FormItem>
					<FormItem field="context_size" label="contextSize" feedback>
						<Input
							v-model="settingform.chatgptInfo.context_size"
							default-value="2" />
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
				</CollapseItem>
			</Collapse>
		</Form>
	</Drawer>
</template>

<script lang="ts" setup>
	import { computed, reactive, toRaw, ref, onMounted } from 'vue';
	import {
		Drawer,
		Upload,
		Progress,
		RadioGroup,
		Radio,
		Form,
		FormItem,
		Input,
		Textarea,
		Select,
		Option,
		Collapse,
		CollapseItem,
		InputPassword,
		FileItem,
	} from '@arco-design/web-vue';
	import {
		IconMoonFill,
		IconSunFill,
		IconComputer,
		IconEdit,
	} from '@arco-design/web-vue/es/icon';
	import { useTheme } from '@/hooks/useTheme';
	import { useAppStore } from '@/store';
	import defaultState from '@/config/app.json';
	import defaultAvatar from '@/assets/user.jpg';

	const appStore = useAppStore();

	const settingform = reactive({
		language: appStore.language,
		theme: appStore.theme,
		content: appStore.content,
		chatgptInfo: appStore.chatgptInfo,
	});
	const { setTheme } = useTheme();

	const uploadRef = ref();

	const file = ref<FileItem | null>(null);

	const userImageUrl = computed(() => {
		if (appStore.userAvatarUrl) {
			return appStore.userAvatarUrl;
		}
		return defaultAvatar;
	});

	const changeTheme = (value: string | number | boolean, ev: Event) => {
		// @ts-ignore
		setTheme(value);
	};

	const onFileChange = (_: any, currentFile: any) => {
		file.value = {
			...currentFile,
		};
	};

	async function uploadFileItem(fileItem: FileItem): Promise<void> {
		try {
			const response = await fetch(
				`http://localhost:19999/upload/${fileItem.name}`,
				{
					method: 'POST',
					body: fileItem.file,
					headers: {
						'Content-Type': 'multipart/form-data',
					},
				}
			);

			if (response.ok) {
				appStore.updateSettings({
					userAvatarUrl: `http://localhost:19999/assets/image/${fileItem.name}`,
				});
			} else {
				fileItem.status = 'error';
			}
		} catch (error) {
			fileItem.status = 'error';
		}
	}
	const submitUpload = async () => {
		try {
			if (!file.value) {
				return;
			}
			await uploadFileItem(file.value);
		} catch (error) {
			console.log('submitUpload error:', error);
		}
	};

	const changeLanguage = (
		value:
			| string
			| number
			| Record<string, any>
			| (string | number | Record<string, any>)[]
	) => {
		// @ts-ignore
		appStore.setLanguage(value);
	};

	const visible = computed(() => appStore.globalSettings);

	const reset = () => {
		appStore.updateSettings(defaultState);
	};

	const saveSettings = async () => {
		const { language, theme, content, chatgptInfo } = toRaw(settingform);
		await submitUpload();
		appStore.updateSettings({
			language,
			theme,
			content,
			chatgptInfo,
			globalSettings: false,
		});
	};
</script>
