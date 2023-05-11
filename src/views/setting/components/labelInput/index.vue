<template>
	<div class="input-box flex items-center justify-between w-full">
		<label
			for="title"
			class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
			>{{ title }}</label
		>
		<input
			:type="type"
			class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
			:placeholder="placeholder"
			required />
	</div>
</template>

<script lang="ts" setup>
	import { toRefs } from 'vue';

	interface Props {
		title: string;
		type: string;
		placeholder: string;
	}

	const props = defineProps<Props>();

	const { title, type, placeholder } = toRefs(props);
</script>
