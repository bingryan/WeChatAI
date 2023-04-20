<script lang="ts" setup>
	import { computed, ref } from 'vue';
	import markdownToHtml from '@/utils/markdown';
	import { useAppStore } from '@/store';

	interface Props {
		isUser?: boolean;
		content?: string;
		loading?: boolean;
	}

	const appStore = useAppStore();

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
</script>

<template>
	<div class="text-wrap min-w-[20px] rounded-md p-3" :style="bdStyle">
		<div ref="textRef" class="leading-relaxed break-words">
			<div v-if="isRenderMarkdown" class="markdown-body" v-html="text" />
			<div v-else class="whitespace-pre-wrap" v-text="text" />
		</div>
	</div>
</template>

<style lang="less">
	@import url(./themes.less);
</style>
