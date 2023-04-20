<template>
	<ConfigProvider class="h-screen" :locale="locale" :global="true">
		<router-view />
		<Setting />
	</ConfigProvider>
</template>

<script setup lang="ts">
	import { ConfigProvider } from '@arco-design/web-vue';
	import { useLocale } from '@/hooks/useLocale';
	import Setting from '@/views/setting/Index.vue';

	const { locale } = useLocale();
</script>
