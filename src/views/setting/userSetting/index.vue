<template>
	<Form ref="formRef" :model="settingform">
		<div class="w-full p-6 mb-3 rounded-lg avatar-box flex items-center">
			<Avatar
				:size="80"
				shape="square"
				:image-url="file && file.url ? file.url : userImageUrl"
				trigger-type="mask"
				@click="uploadFile">
				<template #trigger-icon>
					<Upload
						ref="uploadRef"
						action="/"
						:auto-upload="false"
						:show-file-list="false"
						@change="onFileChange">
						<template #upload-button> </template>
					</Upload>
				</template>
			</Avatar>
			<div>
				<div class="ml-4 text-base font-medium"> Modify user settings</div>
				<div class="ml-4 text-sm font-normal opacity-60"
					>Upload a picture to modify the avatar</div
				>
			</div>
		</div>

		<LabelBox :title="$t('settings.theme')">
			<FormItem>
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
		</LabelBox>
		<LabelBox :title="$t('settings.userBackgroundColor')">
			<FormItem field="userBackgroundColor">
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
		</LabelBox>
		<LabelBox :title="$t('settings.assistantBackgroundColor')">
			<FormItem field="userBackgroundColor">
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
		</LabelBox>
		<LabelBox :title="$t('settings.userContentType')">
			<div class="w-36">
				<FormItem field="userContentType" feedback>
					<Select
						v-model="settingform.content.userContentType"
						default-value="raw">
						<Option value="markdown">markdown</Option>
						<Option value="raw">raw</Option>
					</Select>
				</FormItem>
			</div>
		</LabelBox>
		<LabelBox :title="$t('settings.assistantContentType')">
			<div class="w-36">
				<FormItem field="assistantContentType" feedback>
					<Select
						v-model="settingform.content.assistantContentType"
						default-value="markdown">
						<Option value="markdown">markdown</Option>
						<Option value="raw">raw</Option>
					</Select>
				</FormItem>
			</div>
		</LabelBox>
		<LabelBox :title="$t('settings.language')">
			<div class="w-36">
				<FormItem field="locale">
					<Select v-model="settingform.language" @change="changeLanguage">
						<Option value="en-US">English</Option>
						<Option value="zh-CN">简体中文</Option>
					</Select>
				</FormItem>
			</div>
		</LabelBox>
		<LabelBox :title="$t('settings.sidebar')" :desc="$t('settings.sidebarDes')">
			<div class="w-36">
				<FormItem field="sidebar">
					<Switch v-model="settingform.sidebar" @change="changeSidebar" />
				</FormItem>
			</div>
		</LabelBox>
	</Form>
</template>

<script lang="ts" setup>
	import { computed, reactive, toRaw, ref } from 'vue';
	import {
		Upload,
		RadioGroup,
		Radio,
		Form,
		FormItem,
		Switch,
		Select,
		Option,
		FileItem,
		Avatar,
	} from '@arco-design/web-vue';
	import {
		IconMoonFill,
		IconSunFill,
		IconComputer,
	} from '@arco-design/web-vue/es/icon';
	import { useTheme } from '@/hooks/useTheme';
	import { useAppStore } from '@/store';
	import defaultAvatar from '@/assets/user.jpg';
	import LabelBox from '../components/labelBox/index.vue';

	const appStore = useAppStore();

	const settingform = reactive({
		language: appStore.language,
		theme: appStore.theme,
		content: appStore.content,
		sidebar: appStore.sidebar,
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
	const onFileChange = (_: any, currentFile: any) => {
		file.value = {
			...currentFile,
		};
		submitUpload();
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

	const changeSidebar = (value: string | number | boolean) => {
		// @ts-ignore
		appStore.updateSettings({
			sidebar: value as boolean,
		});
	};

	const uploadFile = () => {
		uploadRef.value.$el.click();
	};
</script>
