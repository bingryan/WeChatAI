<script setup lang="ts">
	import { computed, onMounted, onUnmounted, ref } from 'vue';
	import { createParser } from 'eventsource-parser';
	import { useRoute } from 'vue-router';
	import { router } from '@/router';
	import { Spin } from '@arco-design/web-vue';
	import { useChatStore } from '@/store';
	import Textarea from '@/components/Textarea/index.vue';
	import { useCopyCode } from './hooks/useCopyCode';
	import { useChat } from './hooks/useChat';
	import { useScroll } from './hooks/useScroll';
	import { Message } from './components';

	const chatStore = useChatStore();

	const currentActiveId = computed(() => chatStore.current);

	const route = useRoute();

	const { id } = route.params as { id: string };
	if (!id) {
		router.push(`/chat/${currentActiveId.value}`);
	}

	const cacheData = computed(() => chatStore.getCacheById(+id));
	const chatSetting = computed(() => chatStore.getChatSettingById(+id));
	const contentSize = computed(
		() => chatSetting.value?.setting.content_size ?? '2'
	);

	// filter error messages
	const cacheContextList = computed(() => {
		let chatRecords =
			cacheData.value
				?.filter((ele: any) => !ele.error)
				.slice(-(+contentSize.value + 1))
				// @ts-ignore
				.map(({ role, content }) => ({ role, content })) ?? [];

		const systemMessage = chatSetting.value?.setting.system_message;
		if (systemMessage) {
			chatRecords = [
				{
					role: 'system',
					content: systemMessage,
				},
				...chatRecords,
			];
		}
		return chatRecords;
	});

	let controller = new AbortController();

	useCopyCode();

	const { scrollRef, scrollToBottom } = useScroll();

	const { addCache } = useChat(scrollToBottom);

	const prompt = ref<string>('');
	const loading = ref<boolean>(false);

	async function* iterableStreamAsync(
		stream: ReadableStream
	): AsyncIterableIterator<Uint8Array> {
		const reader = stream.getReader();
		try {
			while (true) {
				// eslint-disable-next-line no-await-in-loop
				const { value, done } = await reader.read();
				if (done) {
					return;
				}
				yield value;
			}
		} finally {
			reader.releaseLock();
		}
	}

	async function handleSSE(
		response: Response,
		onMessage: (message: string) => void
	) {
		if (!response.ok) {
			const error = await response.json().catch(() => null);
			throw new Error(
				error
					? JSON.stringify(error)
					: `${response.status} ${response.statusText}`
			);
		}
		if (response.status !== 200) {
			throw new Error(
				`Error from OpenAI: ${response.status} ${response.statusText}`
			);
		}
		if (!response.body) {
			throw new Error('No response body');
		}
		const parser = createParser((event) => {
			if (event.type === 'event') {
				onMessage(event.data);
			}
		});
		// eslint-disable-next-line no-restricted-syntax
		for await (const chunk of iterableStreamAsync(response.body)) {
			const str = new TextDecoder().decode(chunk);
			parser.feed(str);
		}
	}

	async function handleConversation() {
		const message = prompt.value;

		if (loading.value) return;

		if (!message || message.trim() === '') return;

		controller = new AbortController();

		addCache(+id, {
			role: 'user',
			error: false,
			time: new Date().getTime().toString(),
			content: message,
		});

		loading.value = true;
		prompt.value = '';

		addCache(+id, {
			role: 'assistant',
			error: false,
			time: new Date().getTime().toString(),
			content: '',
		});

		let lastContent = '';

		try {
			if (!chatSetting.value) {
				return;
			}

			const {
				engine,
				api_url: apiUrl,
				openai_key: openaiKey,
			} = chatSetting.value.setting;
			const messages = cacheContextList.value.slice(0, -1);
			console.log('messages:', messages);
			const response = await fetch(apiUrl, {
				method: 'POST',
				headers: {
					'Authorization': `Bearer ${openaiKey}`,
					'Content-Type': 'application/json',
				},
				body: JSON.stringify({
					messages,
					model: engine,
					stream: true,
				}),
				signal: controller.signal,
			});

			await handleSSE(response, (message: any) => {
				if (message === '[DONE]') {
					return;
				}
				const data = JSON.parse(message);
				if (data.error) {
					throw new Error(`Error from OpenAI: ${JSON.stringify(data)}`);
				}
				const text = data.choices[0]?.delta?.content;
				if (text !== undefined) {
					lastContent += text;
					addCache(
						+id,
						{
							role: 'assistant',
							error: false,
							time: new Date().getTime().toString(),
							content: lastContent,
						},
						cacheData.value.length - 1
					);
				}
			});
		} catch (e) {
			addCache(
				+id,
				{
					role: 'assistant',
					error: true,
					time: new Date().getTime().toString(),
					content: lastContent,
				},
				cacheData.value.length - 1
			);
		} finally {
			loading.value = false;
		}
	}

	function handleSubmit() {
		handleConversation();
	}

	function handleEnter(event: KeyboardEvent) {
		if (event.key === 'Enter' && !event.shiftKey) {
			event.preventDefault();
			handleSubmit();
		}
	}

	function handleStop() {
		if (loading.value) {
			controller.abort();
			loading.value = false;
		}
	}

	onMounted(() => {
		scrollToBottom();
	});

	onUnmounted(() => {
		if (loading.value) controller.abort();
	});
</script>

<template>
	<div
		class="flex flex-col w-full h-full overflow-auto bg-white dark:bg-[#111111]">
		<main class="flex-1 overflow-hidden">
			<div
				id="scrollRef"
				ref="scrollRef"
				class="h-full overflow-hidden overflow-y-auto p-4">
				<div class="w-full max-w-screen-xl m-auto">
					<div>
						<Message
							v-for="(item, index) of cacheData"
							:key="index"
							:time="item.time"
							:content="item.content"
							:role="item.role"
							:loading="loading" />
						<div class="sticky flex justify-center">
							<div v-if="loading" @click="handleStop">
								<Spin dot></Spin>
							</div>
						</div>
					</div>
				</div>
			</div>
		</main>
		<footer class="p-4">
			<div class="w-full max-w-screen-xl overflow-auto">
				<div class="flex items-center justify-between space-x-2">
					<Textarea v-model="prompt" @keypress="handleEnter" />
				</div>
			</div>
		</footer>
	</div>
</template>
