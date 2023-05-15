<template>
	<div
		v-show="open"
		ref="menu"
		style="box-shadow: 0 4px 10px #0000001a"
		class="w-[10rem] lg:w-[12rem] xl:w-[16rem] fixed z-50 rounded py-1 text-neutral-700 text-sm top-20 bg-white max-h-[16rem] overflow-auto focus-visible:outline-none">
		<div class="text-left divide-y">
			<div
				v-if="searchTerm"
				class="block-menu-search px-2 py-2 flex gap-2 w-full">
				<div class="truncate">
					{{ searchTerm }}
				</div>
			</div>
			<div v-if="promptOptions.length" class="px-2 py-2">
				<div
					v-for="(option, i) in promptOptions"
					:key="i"
					class="px-2 py-1 rounded flex items-center gap-2 cursor-pointer"
					:class="[active === i ? 'bg-neutral-100' : '']"
					@click="setBlockType(i)">
					<i :class="option.icon"></i>
					<span class="truncate">{{ option.name }}</span>
				</div>
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
	import { nextTick, onMounted, reactive, computed, ref } from 'vue';

	const props = withDefaults(
		defineProps<{
			parentId: string;
			command?: string; // 唤起菜单command
			menuList: App.PromptTemplate[];
		}>(),
		{
			command: '/',
		}
	);

	const cursorPosition = reactive({
		x: 0,
		y: 0,
	});

	const open = ref(false);
	const active = ref(0);
	const searchTerm = ref('');
	const menu = ref<HTMLDivElement | null>(null);

	const promptList = computed(() => {
		const searchValue = searchTerm.value.trim();
		const templateList = props.menuList;
		if (searchValue) {
			return templateList.filter((template) => {
				const { name, content } = template;
				return name.includes(searchValue) || content.includes(searchValue);
			});
		}
		return templateList;
	});

	const promptOptions = computed(() => {
		const templateList = promptList.value;
		return templateList.map((template) => {
			return {
				icon: 'fa-solid fa-font',
				...template,
			};
		});
	});

	const insertCurrentCursor = (input: string) => {
		const editableContent = document.getElementById(props.parentId);
		const selection = window.getSelection();

		if (!selection || selection.rangeCount === 0) return;

		const range = selection.getRangeAt(0);
		const textNode = document.createTextNode(input);

		// replace slash and searchTerm with input
		const termLength = searchTerm.value.length + 1;
		const removedSearchTerm = searchTerm.value.slice(0, -termLength);
		range.setStart(range.startContainer, range.startOffset - termLength);
		range.deleteContents();
		range.insertNode(document.createTextNode(removedSearchTerm));

		range.insertNode(textNode);
		range.setStartAfter(textNode);
		range.setEndAfter(textNode);
		selection.removeAllRanges();
		selection.addRange(range);
		editableContent?.focus();
	};

	const setBlockType = (index: number) => {
		active.value = index;
		setTimeout(() => {
			open.value = false;
		}, 500);
	};

	const handleEnter = (e: Event, input: string) => {
		insertCurrentCursor(input);
		searchTerm.value = '';
		open.value = false;
	};

	const getCursorPosition = () => {
		const range = getSelection()?.getRangeAt(0);
		if (range) {
			const rect = range.getBoundingClientRect();
			cursorPosition.x = rect.left;
			cursorPosition.y = rect.top;
		}
	};

	const setMenuPosition = async () => {
		const menuDom: any = menu.value;
		await nextTick();
		menuDom.style.top = `${cursorPosition.y - menuDom.offsetHeight}px`;
		menuDom.style.left = `${cursorPosition.x}px`;
	};

	onMounted(() => {
		if (menu.value) {
			const editableContent = document.getElementById(props.parentId);
			editableContent?.addEventListener(
				'keydown',
				async (event: KeyboardEvent) => {
					// console.log('auto block menu keydown event:', event, open.value);
					if (event.key === props.command) {
						if (!open.value) {
							open.value = true;
							active.value = 0;
							// 延时才能及时获取光标所处位置
							setTimeout(() => {
								getCursorPosition();
								setMenuPosition();
							});
						}
					}
					if (!open.value) return;

					if (['ArrowUp', 'ArrowDown'].includes(event.key)) {
						// TODO: if active value not in view, scroll to it
						// Support up/down navigation with keyboard
						if (event.key === 'ArrowUp') {
							// Move up
							// Scroll to bottom of menu if at top
							active.value =
								active.value - 1 >= 0
									? active.value - 1
									: promptOptions.value.length - 1;
						} else {
							// Move down
							// Scroll to top of menu if at bottom
							active.value =
								active.value + 1 <= promptOptions.value.length - 1
									? active.value + 1
									: 0;
						}
						event.preventDefault();
						setMenuPosition();
					} else if (event.key === 'Enter') {
						handleEnter(event, promptOptions.value[active.value].name);
						event.preventDefault();
					} else if (event.key === 'Escape') {
						// Escape closes menu
						open.value = false;
						searchTerm.value = '';
						active.value = 0;
					} else if (event.key.match(/^([a-zA-Z]|[0-9]| )$/)) {
						// Alphanumeric searches menu
						searchTerm.value += event.key;
						active.value = 0;
						setMenuPosition();
					} else if (event.key === 'Backspace') {
						// Backspace closes menu if searchTerm is empty
						if (searchTerm.value.length === 0) open.value = false;
						else searchTerm.value = searchTerm.value.slice(0, -1);
						active.value = 0;
						setMenuPosition();
					}
				}
			);
		}
	});
</script>
