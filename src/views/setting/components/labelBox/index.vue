<template>
	<div class="label-box flex items-center justify-between w-full">
		<div>
			<div class="text-sm font-medium">{{ title }}</div>
			<div class="text-xs font-normal opacity-60">{{ desc }}</div>
		</div>
		<div class="ml-4">
			<slot></slot>
		</div>
	</div>
</template>

<script lang="ts" setup>
	import { toRefs } from 'vue';

	interface Props {
		title: string;
		desc?: string;
	}

	const props = defineProps<Props>();

	const { title, desc } = toRefs(props);
</script>
