<template>
	<div class="label-box">
		<Row align="center">
			<Col :span="20">
				<div class="text-sm">{{ title }}</div>
				<div class="text-xs">{{ desc }}</div>
			</Col>
			<Col :span="4">
				<slot></slot>
			</Col>
		</Row>
	</div>
</template>

<script lang="ts" setup>
	import { toRefs } from 'vue';
	import { Row, Col } from '@arco-design/web-vue';

	const props = defineProps({
		title: String,
		desc: String,
	});

	const { title, desc } = toRefs(props);
</script>
