<template>
	<div ref="container" as="div" class="relative w-max h-max">
		<div v-show="open" class="block-menu">
			<div
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
		</div>
	</div>
</template>

<script setup lang="ts">
	import { ref, computed, watch, onMounted, nextTick, reactive } from 'vue';
	import { usePromptStore } from '@/store';

	const open = ref(false);
	const active = ref(0);
	const searchTerm = ref('');

	const cursorPosition = reactive({
		x: 0,
		y: 0,
	});

	const container = ref<HTMLDivElement | null>(null);
	const menu = ref<HTMLDivElement | null>(null);
	const promptStore = usePromptStore();

	const promptList = computed(() => {
		const searchValue = searchTerm.value.trim();
		if (searchValue) {
			const templateList = promptStore.getTemplate;
			return templateList.filter((template) => {
				const { name, content } = template;
				return name.includes(searchValue) || content.includes(searchValue);
			});
		}
		return promptStore.getTemplate;
	});

	const promptOptions = computed(() => {
		const templateList = promptList.value;
		return templateList.map((template) => {
			const { name, content } = template;
			return {
				icon: 'fa-solid fa-font',
				name,
				content,
			};
		});
	});

	const handleEnter = () => {
		searchTerm.value = '';
		open.value = false;
		console.log('"Enter" key pressed');
	};

	const setBlockType = (index: number) => {
		active.value = index;
		setTimeout(() => {
			open.value = false;
		}, 500);
	};

	const setMenuPosition = async () => {
		const menuDom: any = menu.value;
		await nextTick();
		menuDom.style.top = `${cursorPosition.y - menuDom.offsetHeight}px`;
		menuDom.style.left = `${cursorPosition.x}px`;
	};

	const changeMenuPosition = async (x: number, y: number) => {
		cursorPosition.x = x;
		cursorPosition.y = y;
		setMenuPosition();
	};

	function insertCurrentCursor(input: string) {
		const selection = window.getSelection();

		if (!selection || selection.rangeCount === 0) return;

		const range = selection.getRangeAt(0);
		const textNode = document.createTextNode(input);

		// slash auto complete
		const textBeforeCursor = range.startContainer.textContent?.slice(
			range.startOffset - 1,
			range.startOffset
		);

		if (textBeforeCursor?.endsWith('/')) {
			const removedSlash = textBeforeCursor.slice(0, -1);
			range.setStart(range.startContainer, range.startOffset - 1);
			range.deleteContents();
			range.insertNode(document.createTextNode(removedSlash));
		}

		range.insertNode(textNode);
		range.setStartAfter(textNode);
		range.setEndAfter(textNode);
		selection.removeAllRanges();
		selection.addRange(range);
		// add space after auto complete
		const space = document.createTextNode(' ');
		range.insertNode(space);
		range.setStartAfter(space);
		range.setEndAfter(space);
		const newRange = document.createRange();
		newRange.setStartAfter(space);
		newRange.setEndAfter(space);
		selection.removeAllRanges();
		selection.addRange(newRange);
	}

	onMounted(() => {
		if (menu.value) {
			const editableContent = document.getElementById('editableContent');

			editableContent?.addEventListener('keydown', (event: KeyboardEvent) => {
				// console.log('auto block menu keydown event:', event);
				if (!open.value) return;

				if (['ArrowUp', 'ArrowDown'].includes(event.key)) {
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
				} else if (event.key === 'Enter') {
					insertCurrentCursor(promptOptions.value[active.value].name);
					handleEnter();
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
			});
		}
	});

	defineExpose({
		open,
		changeMenuPosition,
	});
</script>
