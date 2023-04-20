<script setup lang="ts">
	import { Layout, LayoutContent } from '@arco-design/web-vue';
	import { useAppStore } from '@/store';
	import { computed } from 'vue';
	import SiderBar from '@/views/siderbar/index.vue';
	import ActivityBar from '@/views/activitybar/index.vue';

	const appStore = useAppStore();

	const sidebar = computed(() => appStore.sidebar);
	const avtivitybar = computed(() => appStore.avtivitybar);
</script>

<template>
	<div class="h-full overflow-hidden">
		<Layout class="z-40 transition">
			<div class="flex h-full overflow-hidden">
				<div v-if="avtivitybar">
					<ActivityBar class="h-full" />
				</div>
				<div v-if="sidebar">
					<SiderBar class="h-full" />
				</div>
				<LayoutContent class="h-screen">
					<RouterView v-slot="{ Component, route }">
						<component :is="Component" :key="route.fullPath" />
					</RouterView>
				</LayoutContent>
			</div>
		</Layout>
	</div>
</template>
