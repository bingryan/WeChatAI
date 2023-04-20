<script lang="ts" setup>
	import { computed } from 'vue';
	import { Avatar } from '@arco-design/web-vue';
	import { useAppStore } from '@/store';
	import defaultAvatar from '@/assets/user.jpg';
	import defaultRebotAvatar from '@/assets/logo/chatgpt.png';

	interface Props {
		isUser?: boolean;
	}
	defineProps<Props>();

	const appStore = useAppStore();

	const userImageUrl = computed(() => {
		if (appStore.userAvatarUrl) {
			return appStore.userAvatarUrl;
		}
		return defaultAvatar;
	});
	const rebotImageUrl = computed(() => {
		if (appStore.assistantAvatarUrl) {
			return appStore.assistantAvatarUrl;
		}
		return defaultRebotAvatar;
	});
</script>

<template>
	<!-- 用户头像 -->
	<template v-if="isUser">
		<Avatar :image-url="userImageUrl" />
	</template>
	<!-- GPT机器头像 -->
	<template v-else>
		<Avatar :image-url="rebotImageUrl" />
	</template>
</template>
