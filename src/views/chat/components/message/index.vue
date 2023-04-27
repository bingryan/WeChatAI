<!-- eslint-disable default-case -->
<script setup lang="ts">
	import { ref } from 'vue';
	import Avatar from '@/components/Avatar/index.vue';
	import TextComponent from './Text.vue';

	interface Props {
		time: string;
		index: number;
		content: string;
		role: string;
		loading?: boolean;
	}

	const props = defineProps<Props>();

	const textRef = ref<HTMLElement>();

	const isUser = props.role === 'user';
	const dateTime = new Date(+props.time).toLocaleString();
</script>

<template>
	<div
		class="flex w-full mb-6 overflow-hidden"
		:class="[{ 'flex-row-reverse': isUser }]">
		<div
			class="flex items-center justify-center flex-shrink-0 h-8 overflow-hidden rounded-full basis-8"
			:class="[isUser ? 'ml-2' : 'mr-2']">
			<Avatar :is-user="isUser" />
		</div>
		<div
			class="overflow-hidden text-sm"
			:class="[isUser ? 'items-end' : 'items-start']">
			<p
				class="text-xs text-[#b4bbc4]"
				:class="[isUser ? 'text-right' : 'text-left']">
				{{ dateTime }}
			</p>
			<div
				class="flex items-end gap-1 mt-2"
				:class="[isUser ? 'flex-row-reverse' : 'flex-row']">
				<TextComponent
					ref="textRef"
					:is-user="isUser"
					:index="props.index"
					:content="props.content"
					:loading="props.loading" />
			</div>
		</div>
	</div>
</template>
