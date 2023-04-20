<template>
	<Draggable
		:list="chatList"
		item-key="id"
		class="list-group space-y-1"
		ghost-class="ghost"
		@end="onDragEnd">
		<template #item="{ element }">
			<div class="list-group-item">
				<button
					class="flex items-center w-full px-5 py-2 transition-colors duration-200 gap-x-2 focus:outline-none"
					:class="isActive(element.id) && ['bg-[#f2f3f5]', 'dark:bg-[#363636]']"
					@click="handleSelect(element.id)">
					<div class="relative">
						<template v-if="isActive(element.id)">
							<span
								class="h-2 w-2 rounded-full bg-emerald-500 absolute right-0.5 ring-1 ring-white bottom-0 z-50">
							</span>
							<Popover position="right" trigger="click">
								<Avatar :image-url="chatGPTLogo" />
								<template #content>
									<div class="w-14">
										<p
											class="hover:bg-[#6098f7]"
											@click="deleteSelect(element.id)"
											>{{ $t('common.delete') }}</p
										>
										<p
											class="hover:bg-[#6098f7]"
											@click="editSelect(element.id)"
											>{{ $t('common.edit') }}</p
										>
									</div>
								</template>
							</Popover>
						</template>
						<template v-else>
							<Avatar :image-url="chatGPTLogo" />
						</template>
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
	import { computed } from 'vue';
	import { Avatar, Popover } from '@arco-design/web-vue';

	const chatStore = useChatStore();
	const appStore = useAppStore();

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
</script>
