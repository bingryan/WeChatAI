<template>
	<svg :class="svgClass" aria-hidden="true">
		<use :xlink:href="symbolId" :fill="color" />
	</svg>
</template>

<script lang="ts">
	import { defineComponent, computed } from 'vue';

	export default defineComponent({
		name: 'SvgIcon',
		props: {
			prefix: {
				type: String,
				default: 'icon',
			},
			name: {
				type: String,
				required: true,
			},
			color: {
				type: String,
				default: 'currentColor',
			},
			svgClass: {
				type: String,
				default: 'w-5 h-5',
			},
		},
		setup(props) {
			const symbolId = computed(() => `#${props.prefix}-${props.name}`);
			return { symbolId };
		},
	});
</script>
