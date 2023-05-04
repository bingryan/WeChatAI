<template>
	<Modal
		id="menu-content"
		class="setting-box"
		width="80vw"
		:visible="setting.visible"
		:footer="false"
		:cancel-text="$t('settings.reset')"
		:ok-text="$t('settings.save')"
		:closable="false">
		<Layout :style="{ height: '80vh' }">
			<LayoutSider>
				<Menu
					auto-open:true
					:default-selected-keys="['user-setting']"
					:style="{
						borderRadius: '8px',
						height: '100%',
					}"
					:collapsed="false"
					:auto-open="true"
					level-indent="0"
					@menu-item-click="onClickMenuItem">
					<SubMenu :title="$t('settings.options')">
						<MenuItem key="user-setting">
							{{ $t('settings.menu.userConfig') }}
						</MenuItem>
					</SubMenu>
					<SubMenu :title="$t('settings.models')">
						<MenuItem key="chatgpt-setting">
							{{ $t('settings.menu.chatgptConfig') }}
						</MenuItem>
					</SubMenu>
					<SubMenu title="Other">
						<MenuItem key="prompt-setting"> Prompt </MenuItem>
					</SubMenu>
				</Menu>
			</LayoutSider>
			<LayoutContent class="layout-content">
				<div class="close-icon">
					<Button type="text" shape="circle" @click="closeModal">
						<IconClose />
					</Button>
				</div>
				<div class="layout-box">
					<component :is="menus[selectedKey]" ref="childRef" />
				</div>
			</LayoutContent>
		</Layout>
	</Modal>
</template>

<script lang="ts" setup>
	import { useAppStore } from '@/store/app';
	import { computed, ref } from 'vue';
	import {
		Modal,
		Menu,
		MenuItem,
		Layout,
		LayoutSider,
		LayoutContent,
		SubMenu,
		Button,
	} from '@arco-design/web-vue';
	import { IconClose } from '@arco-design/web-vue/es/icon';
	import UserSetting from './userSetting/index.vue';
	import ChagptSetting from './chatgptSetting/index.vue';
	import DemoSetting from './demoSetting/index.vue';
	import PromptSetting from './promptSetting/index.vue';

	const childRef = ref(null);

	const selectedKey = ref('user-setting');

	const menus: any = {
		'user-setting': UserSetting,
		'chatgpt-setting': ChagptSetting,
		'demo-setting': DemoSetting,
		'prompt-setting': PromptSetting,
	};

	const appStore = useAppStore();
	const setting = computed(() => ({
		visible: appStore.globalSettings,
		selectedKeys: appStore.selectedKeys,
	}));

	const closeModal = () => {
		appStore.updateSettings({
			globalSettings: false,
		});
	};

	const onClickMenuItem = (key: string) => {
		selectedKey.value = key;
	};
</script>
