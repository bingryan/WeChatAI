<template>
	<Draggable
		:list="chatList"
		item-key="id"
		class="list-group space-y-1"
		ghost-class="ghost"
		@end="onDragEnd">
		<template #item="{ element }">
			<div
				class="list-group-item"
				@contextmenu.prevent="onContextMenu($event, element.id)">
				<button
					class="flex items-center w-full px-5 py-2 transition-colors duration-200 gap-x-2 focus:outline-none"
					:class="isActive(element.id) && ['bg-[#f2f3f5]', 'dark:bg-[#363636]']"
					@click="handleSelect(element.id)">
					<div class="relative">
						<template v-if="isActive(element.id)">
							<span
								class="h-2 w-2 rounded-full bg-emerald-500 absolute right-0.5 ring-1 ring-white bottom-0 z-50">
							</span>
						</template>
						<Avatar :image-url="chatGPTLogo" />
					</div>
					<div class="text-left rtl:text-right">
						<h1
							class="text-sm font-medium text-gray-700 capitalize dark:text-white">
							{{ element.title }}
						</h1>
					</div>
				</button>
			</div>
		</template>
	</Draggable>
</template>

<script setup lang="ts">
	import Draggable from 'vuedraggable';
	import chatGPTLogo from '@/assets/logo/chatgpt.png';
	import { useChatStore, useAppStore } from '@/store';
	import { computed, h } from 'vue';
	import { Avatar } from '@arco-design/web-vue';
	import ContextMenu from '@imengyu/vue3-context-menu';
	import type { MenuOptions } from '@imengyu/vue3-context-menu';
	import { useI18n } from 'vue-i18n';

	const chatStore = useChatStore();
	const appStore = useAppStore();
	const { t } = useI18n();

	const chatList = computed(() => {
		return chatStore.chatSetting;
	});

	const onDragEnd = () => {
		chatStore.updateChatSetting(chatList.value);
	};

	function isActive(id: number) {
		return chatStore.current === id;
	}

	async function handleSelect(id: number) {
		if (isActive(id)) return;
		await chatStore.setCurrent(id);
	}

	async function deleteSelect(id: number) {
		await chatStore.removeChatSetting(id);
	}

	async function editSelect(id: number) {
		appStore.updateSettings({
			chatModelVisible: true,
			newChatModal: false,
		});
	}

	async function clearSelect(id: number) {
		await chatStore.clearCache(id);
	}

	const contextMenuTheme = computed(() => {
		return appStore.theme === 'dark' ? 'mac dark' : 'mac';
	});

	async function onContextMenu(e: MouseEvent, id: number) {
		await handleSelect(id);
		ContextMenu.showContextMenu({
			theme: contextMenuTheme.value,
			items: [
				{
					label: t('common.delete'),
					icon: h('i', {
						class: 'fa-solid fa-trash',
					}),
					onClick: () => deleteSelect(id),
				},
				{
					label: t('common.edit'),
					icon: h('i', {
						class: 'fa-solid fa-pen-to-square',
					}),
					onClick: () => editSelect(id),
				},
				{
					label: t('common.clear'),
					icon: h('i', {
						class: 'fa-solid fa-broom',
					}),
					onClick: () => clearSelect(id),
				},
			],
			zIndex: 60,
			minWidth: 100,
			x: e.x,
			y: e.y,
		} as MenuOptions);
	}
</script>
