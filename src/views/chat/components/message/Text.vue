<script lang="ts" setup>
	import { computed, ref, h } from 'vue';
	import markdownToHtml from '@/utils/markdown';
	import { useAppStore, useChatStore } from '@/store';
	import { useRoute } from 'vue-router';
	import ContextMenu from '@imengyu/vue3-context-menu';
	import type { MenuOptions } from '@imengyu/vue3-context-menu';
	import { useI18n } from 'vue-i18n';
	import { useClipboard } from '@vueuse/core';

	interface Props {
		isUser: boolean;
		index: number;
		content: string;
		loading?: boolean;
	}

	const appStore = useAppStore();
	const chatStore = useChatStore();
	const { t } = useI18n();
	const { copy } = useClipboard();
	const route = useRoute();

	const { id } = route.params as { id: string };

	const userLightBgColor = computed(() => {
		return appStore.content.userLightBackgroundColor;
	});
	const userDarkBgColor = computed(() => {
		return appStore.content.userDarkBackgroundColor;
	});
	const assistantLightBgColor = computed(() => {
		return appStore.content.assistantLightBackgroundColor;
	});
	const assistantDarkBgColor = computed(() => {
		return appStore.content.assistantDarkBackgroundColor;
	});

	const props = defineProps<Props>();

	const textRef = ref<HTMLElement>();

	const styleCond = computed(() => ({
		theme: appStore.theme,
		isUser: props.isUser,
	}));

	const bdStyle = computed(() => {
		switch (JSON.stringify(styleCond.value)) {
			// light user style
			case '{"theme":"light","isUser":true}':
				return `background-color: ${userLightBgColor.value}`;
			// light assistant style
			case '{"theme":"light","isUser":false}':
				return `background-color: ${assistantLightBgColor.value}`;
			// dark user style
			case '{"theme":"dark","isUser":true}':
				return `background-color: ${userDarkBgColor.value}`;
			// dark assistant style
			case '{"theme":"dark","isUser":false}':
				return `background-color: ${assistantDarkBgColor.value}`;
			default:
				return ``;
		}
	});

	const isRenderMarkdown = computed(() => {
		if (props.isUser) return appStore.content.userContentType === 'markdown';
		return appStore.content.assistantContentType === 'markdown';
	});

	const text = computed(() => {
		const value = props.content ?? '';
		if (isRenderMarkdown.value) return markdownToHtml(value);
		return value;
	});

	defineExpose({ textRef });

	function deleteSelect(index: number) {
		chatStore.removeCache(+id, index);
	}

	function copyQA(index: number) {
		const contextList = chatStore.selectContextQA(+id, index);
		const copyContent = `### ${contextList[0].content} \n\n${contextList[1].content}`;
		copy(copyContent);
	}

	const contextMenuTheme = computed(() => {
		return appStore.theme === 'dark' ? 'mac dark' : 'mac';
	});

	async function onContextMenu(e: MouseEvent, index: number) {
		ContextMenu.showContextMenu({
			theme: contextMenuTheme.value,
			items: [
				{
					label: t('common.delete'),
					icon: h('i', {
						class: 'fa-solid fa-trash',
					}),
					onClick: () => deleteSelect(index),
				},
				{
					label: t('common.copy'),
					icon: h('i', {
						class: 'fa fa-clipboard',
					}),
					onClick: () => copy(props.content),
				},
				{
					label: t('common.copyQA'),
					icon: h('i', {
						class: 'fa-solid fa-copy',
					}),
					onClick: () => copyQA(index),
				},
			],
			zIndex: 60,
			minWidth: 100,
			x: e.x,
			y: e.y,
		} as MenuOptions);
	}
</script>

<template>
	<div
		class="text-wrap min-w-[20px] rounded-md p-3"
		:style="bdStyle"
		@contextmenu.prevent="onContextMenu($event, props.index)">
		<div ref="textRef" class="leading-relaxed break-words">
			<div
				v-if="isRenderMarkdown"
				class="markdown-body max-w-full"
				v-html="text" />
			<div v-else class="whitespace-pre-wrap" v-text="text" />
		</div>
	</div>
</template>

<style lang="less">
	@import url(./themes.less);
</style>
