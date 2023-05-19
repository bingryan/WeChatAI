<script setup lang="ts">
	import { computed, ref, toRefs, watch, onMounted } from 'vue';
	import { useChatStore, usePromptStore } from '@/store';
	import { Notification } from '@arco-design/web-vue';
	import { invoke } from '@tauri-apps/api/tauri';
	import AutoBlockMenu from '@/components/AutoBlockMenu/index.vue';
	import SvgIcon from '@/components/SvgIcon/index.vue';

	interface Props {
		modelValue: string;
		placeholder?: string;
		style: unknown;
	}

	interface Emit {
		(e: 'update:modelValue', value: string): true;
	}

	const props = withDefaults(defineProps<Props>(), {
		placeholder: '',
		style: () => ({}),
	});
	const emit = defineEmits<Emit>();

	const editableContentRef = ref<HTMLElement | null>(null);

	const { modelValue, placeholder } = toRefs(props);

	const promptStore = usePromptStore();

	watch(modelValue, (value) => {
		if (
			value === '' ||
			value === undefined ||
			Object.prototype.toString.call(value) === '[object Null]'
		) {
			modelValue.value = '';
			if (editableContentRef.value) {
				editableContentRef.value.innerHTML = '';
			}
		}
	});

	const updateValue = (value: string) => {
		emit('update:modelValue', value);
	};
	const handleInput = (e: Event) => {
		const { innerText } = e.target as HTMLDivElement;
		if (innerText) {
			// dompurify.sanitize(innerText);
			updateValue(innerText.trim());
		}
	};

	// -------------------handle event-----------------------------------------
	const chatStore = useChatStore();

	const getCurrentCacheData = computed(() => {
		const currentActiveId = chatStore.current;
		if (!currentActiveId) {
			return [];
		}
		return chatStore
			.getCacheById(currentActiveId)
			.filter((ele: any) => !ele.error);
	});

	const getCurrentTitle = computed(() => {
		const currentActiveId = chatStore.current;
		if (!currentActiveId) {
			return '';
		}
		return chatStore.getChatSettingById(currentActiveId)?.title;
	});

	async function saveToFile(filename: string, content: string) {
		try {
			await invoke('write_to_file', { filename, content }).then(() => {
				Notification.success({
					title: 'File saved successfully',
					content: `$HOME/Downloads/${filename}`,
				});
			});
			console.log('File saved successfully');
		} catch (error) {
			console.error('Error saving file:', error);
		}
	}
	const exportChatToJson = () => {
		const data = getCurrentCacheData.value;
		const title = getCurrentTitle.value;
		saveToFile(`${title}.json`, JSON.stringify(data));
	};
	const handleSaveToJson = () => {
		exportChatToJson();
	};

	const getLastChild = computed(() => {
		const editableContent = document.getElementById('editableContent');
		if ((editableContent?.lastChild as any)?.length === 1) {
			return editableContent?.firstChild;
		}
		return editableContent?.lastChild;
	});

	const getEndCoordinates = computed(() => {
		let x = 0;
		let y = 0;

		const lastChild = getLastChild.value;

		if (lastChild) {
			const range = document.createRange();
			range.selectNodeContents(lastChild);
			range.collapse();
			const rect = range.getBoundingClientRect();
			x = rect.left;
			y = rect.top;
		}
		return { x, y };
	});

	const getCursorPosition = () => {
		const x = 0;
		const y = 0;
		const editableContent = document.getElementById('editableContent');
	};

	onMounted(() => {
		const editableContent = document.getElementById('editableContent');
		if (editableContent) {
			editableContent.focus();
			editableContent.addEventListener('paste', (event) => {
				event.preventDefault();
				const items = event.clipboardData && event.clipboardData.items;
				if (items) {
					for (let i = 0; i < items.length; i += 1) {
						// text
						if (items[i].type.indexOf('text/plain') !== -1) {
							items[i].getAsString((str) => {
								// when str is html tag, use insertHTML
								// when str is plain text, use insertText
								document.execCommand('insertText', false, str);
							});
						}

						// image
						if (items[i].type.indexOf('image') !== -1) {
							const blob = items[i].getAsFile();
							const reader = new FileReader();
							if (blob) {
								reader.readAsDataURL(blob);
								reader.onloadend = () => {
									const imageSrc = reader.result;
									if (imageSrc) {
										const img = document.createElement('img');
										img.src = imageSrc as string;
										img.style.width = '160px';
										img.style.height = '80px';
										editableContent.appendChild(img);
									}
								};
							}
						}
					}
				}
			});
		}
	});

	defineExpose({
		editableContentRef,
	});
</script>

<template>
	<div class="w-full mb-4 bg-white dark:bg-[#111111]">
		<div class="flex items-center justify-between px-3 py-2 dark:bg-[#111111]">
			<div
				class="flex flex-wrap items-center divide-gray-200 sm:divide-x dark:bg-[#111111]">
				<div class="flex items-center space-x-1 sm:pr-4">
					<button
						type="button"
						class="p-2 text-gray-500 rounded cursor-pointer hover:text-gray-900 hover:bg-gray-100 dark:text-gray-400 dark:hover:text-white dark:hover:bg-gray-600">
						<SvgIcon name="file" svg-class="w-5 h-5" />
						<span class="sr-only">Attach file</span>
					</button>
					<button
						type="button"
						class="p-2 text-gray-500 rounded cursor-pointer hover:text-gray-900 hover:bg-gray-100 dark:text-gray-400 dark:hover:text-white dark:hover:bg-gray-600">
						<SvgIcon name="map" svg-class="w-5 h-5" />
						<span class="sr-only">Embed map</span>
					</button>
					<button
						type="button"
						class="p-2 text-gray-500 rounded cursor-pointer hover:text-gray-900 hover:bg-gray-100 dark:text-gray-400 dark:hover:text-white dark:hover:bg-gray-600">
						<SvgIcon name="image" svg-class="w-5 h-5" />
						<span class="sr-only">Upload image</span>
					</button>
					<button
						type="button"
						class="p-2 text-gray-500 rounded cursor-pointer hover:text-gray-900 hover:bg-gray-100 dark:text-gray-400 dark:hover:text-white dark:hover:bg-gray-600">
						<SvgIcon name="code" svg-class="w-5 h-5" />
						<span class="sr-only">Format code</span>
					</button>
					<button
						type="button"
						class="p-2 text-gray-500 rounded cursor-pointer hover:text-gray-900 hover:bg-gray-100 dark:text-gray-400 dark:hover:text-white dark:hover:bg-gray-600">
						<SvgIcon name="emoji" svg-class="w-5 h-5" />
						<span class="sr-only">Add emoji</span>
					</button>
				</div>
				<div class="flex flex-wrap items-center space-x-1 sm:pl-4">
					<button
						type="button"
						class="p-2 text-gray-500 rounded cursor-pointer hover:text-gray-900 hover:bg-gray-100 dark:text-gray-400 dark:hover:text-white dark:hover:bg-gray-600">
						<SvgIcon name="list" svg-class="w-5 h-5" />
						<span class="sr-only">Add list</span>
					</button>
					<button
						type="button"
						class="p-2 text-gray-500 rounded cursor-pointer hover:text-gray-900 hover:bg-gray-100 dark:text-gray-400 dark:hover:text-white dark:hover:bg-gray-600">
						<SvgIcon name="setting" svg-class="w-5 h-5" />
						<span class="sr-only">Settings</span>
					</button>
					<button
						type="button"
						class="p-2 text-gray-500 rounded cursor-pointer hover:text-gray-900 hover:bg-gray-100 dark:text-gray-400 dark:hover:text-white dark:hover:bg-gray-600">
						<SvgIcon name="timeline" svg-class="w-5 h-5" />
						<span class="sr-only">Timeline</span>
					</button>
					<button
						type="button"
						class="p-2 text-gray-500 rounded cursor-pointer hover:text-gray-900 hover:bg-gray-100 dark:text-gray-400 dark:hover:text-white dark:hover:bg-gray-600"
						@click="handleSaveToJson">
						<SvgIcon name="download" svg-class="w-5 h-5" />
						<span class="sr-only">Download</span>
					</button>
				</div>
			</div>
		</div>
		<div class="px-4 py-2 bg-white dark:bg-[#111111]">
			<div
				id="editableContent"
				ref="editableContentRef"
				contenteditable="true"
				:placeholder="placeholder"
				class="block w-full px-0 text-sm text-gray-800 bg-white border-0 dark:bg-[#111111] focus:ring-0 dark:text-white dark:placeholder-gray-400 border-none resize-none outline-none"
				:style="style"
				@input="handleInput">
			</div>
			<AutoBlockMenu
				:menu-list="promptStore.getTemplate"
				parent-id="editableContent" />
		</div>
	</div>
</template>
