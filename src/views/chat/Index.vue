<script setup lang="ts">
	import { computed, onMounted, onUnmounted, ref } from 'vue';
	import { createParser } from 'eventsource-parser';
	import { useRoute } from 'vue-router';
	import { router } from '@/router';
	import { Spin } from '@arco-design/web-vue';
	import { useChatStore, usePromptStore } from '@/store';
	import EditableContent from '@/components/EditableContent/index.vue';
	import { parseDomain } from '@/utils/misc';
	import { render, getHandlebarsVars } from '@/utils/renderer';
	import { useCopyCode } from './hooks/useCopyCode';
	import { useChat } from './hooks/useChat';
	import { useScroll } from './hooks/useScroll';
	import { Message } from './components';

	const chatStore = useChatStore();
	const promptStore = usePromptStore();

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

	function handleRequestHeader(apiUrl: string, apiKey: string) {
		const { domain } = parseDomain(apiUrl);
		if (domain === 'azure.com') {
			return {
				'api-key': apiKey,
				'Content-Type': 'application/json',
			};
		}
		return {
			'Authorization': `Bearer ${apiKey}`,
			'Content-Type': 'application/json',
		};
	}

	function getPrompt(input: string): string {
		// re match: -p prompt value  -h hh , match prompt value
		// const regex = /-p\s+([\s\S]*?)(?=\s*-|\s*$)/i; not working
		const prompt = input.split(
			/-a|-b|-c|-d|-e|-f|-g|-h|-i|-j|-k|-l|-m|-n|-o|-p|-q|-r|-s|-t|-u|-v|-w|-x|-y|-z/,
			2
		)[1];
		const template = promptStore.getTemplateByName(prompt?.trim() ?? '');
		return template ? template.content : '';
	}

	function parseOptArgs(message: string, hbVars: { [key: string]: string }) {
		const hbVarsKeys = Object.keys(hbVars);

		const separator = new RegExp(hbVarsKeys.map((e) => `-${e}`).join('|'));
		const splitValue = message.split(separator);

		const res: { [key: string]: string } = {};
		for (let i = 0; i < hbVarsKeys.length; i += 1) {
			const key = hbVarsKeys[i];
			const value = splitValue[i + 1]?.trim();
			if (key && value) {
				res[hbVars[key]] = value;
			}
		}
		return res;
	}

	const editableContentRef = ref<HTMLElement | null>(null);
	function parseUserMessage() {
		const editableContent = document.getElementById('editableContent');
		const message = editableContent?.innerText;
		if (!message) {
			return '';
		}

		const firstLine = message.split('\n')[0];
		const template = getPrompt(firstLine);
		if (!template) {
			return message;
		}
		const hbVars = getHandlebarsVars(template);

		const argsParsed = parseOptArgs(message.split('\n').join(' \n '), hbVars);
		return render(template, argsParsed);
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
			content: parseUserMessage(),
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
				api_key: apiKey,
			} = chatSetting.value.setting;
			const messages = cacheContextList.value.slice(0, -1);
			const response = await fetch(apiUrl, {
				method: 'POST',
				// @ts-ignore
				headers: handleRequestHeader(apiUrl, apiKey),
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
							:index="index"
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
					<EditableContent
						ref="editableContentRef"
						v-model="prompt"
						:style="{ height: '100px' }"
						@keypress="handleEnter" />
				</div>
			</div>
		</footer>
	</div>
</template>
