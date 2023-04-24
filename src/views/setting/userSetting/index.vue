<template>
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
			<Select v-model="settingform.content.userContentType" default-value="raw">
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
	</Form>
</template>

<script lang="ts" setup>
	import { computed, reactive, toRaw, ref } from 'vue';
	import {
		Upload,
		Progress,
		RadioGroup,
		Radio,
		Form,
		FormItem,
		Input,
		Select,
		Option,
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
	import defaultAvatar from '@/assets/user.jpg';

	const appStore = useAppStore();

	const settingform = reactive({
		language: appStore.language,
		theme: appStore.theme,
		content: appStore.content,
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

	const saveSettings = async () => {
		const { language, theme, content } = toRaw(settingform);
		await submitUpload();
		appStore.updateSettings({
			language,
			theme,
			content,
		});
	};
	defineExpose({
		saveSettings,
	});
</script>
