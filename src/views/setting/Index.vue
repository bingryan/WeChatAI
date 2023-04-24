<template>
	<Modal
		id="menu-content"
		class="setting-box"
		width="80vw"
		:visible="setting.visible"
		:cancel-text="$t('settings.reset')"
		:ok-text="$t('settings.save')"
		:closable="false"
		@ok="saveSettings"
		@cancel="reset">
		<Layout :style="{ height: '80vh' }">
			<LayoutSider>
				<Menu
					auto-open:true
					:default-selected-keys="['user-setting']"
					:style="{ borderRadius: '8px', height: '100%' }"
					:collapsed="false"
					:auto-open="true"
					@menuItemClick="onClickMenuItem">
					<SubMenu title="常规设置">
						<MenuItem key="user-setting">
							{{ $t('settings.menu.userConfig') }}
						</MenuItem>
						<MenuItem key="chatgpt-setting">
							{{ $t('settings.menu.chatgptConfig') }}
						</MenuItem>
					</SubMenu>
					<SubMenu title="一级标题1">
						<MenuItem key="demo-setting"> 二级标题 </MenuItem>
					</SubMenu>
					<SubMenu title="一级标题2">
						<MenuItem key="demo-setting-2"> 二级标题2 </MenuItem>
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
	import defaultState from '@/config/app.json';
	import UserSetting from './userSetting/index.vue';
	import ChagptSetting from './chatgptSetting/index.vue';
	import DemoSetting from './demoSetting/index.vue';

	const childRef = ref(null);

	const selectedKey = ref('user-setting');

	// {菜单key，组件}
	const menus: any = {
		'user-setting': UserSetting,
		'chatgpt-setting': ChagptSetting,
		'demo-setting': DemoSetting,
	};

	const appStore = useAppStore();
	const setting = computed(() => ({
		visible: appStore.globalSettings,
		selectedKeys: appStore.selectedKeys,
	}));

	const reset = () => {
		appStore.updateSettings(defaultState);
	};

	const saveSettings = () => {
		// @ts-ignore
		childRef.value?.saveSettings();
		appStore.updateSettings({
			globalSettings: false,
		});
	};

	const closeModal = () => {
		appStore.updateSettings({
			globalSettings: false,
		});
	};

	const onClickMenuItem = (key: string) => {
		selectedKey.value = key;
	};
</script>
