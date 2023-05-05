<template>
	<div ref="container" as="div" class="relative w-max h-max">
		<div v-show="open" class="block-menu">
			<div
				ref="menu"
				class="w-[10rem] lg:w-[12rem] xl:w-[16rem] absolute z-50 rounded py-1 text-neutral-700 text-sm top-20 bg-white max-h-[24rem] overflow-auto focus-visible:outline-none">
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
							class="px-2 py-1 rounded flex items-center gap-2"
							:class="[
								active === i + promptOptions.length ? 'bg-neutral-100' : '',
							]">
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
	import { ref, computed, watch, onMounted } from 'vue';
	import { usePromptStore } from '@/store';

	const open = ref(false);
	const active = ref(0);
	const searchTerm = ref('');

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
	};

	onMounted(() => {
		if (menu.value) {
			menu.value.addEventListener('keydown', (event: KeyboardEvent) => {
				console.log('auto block menu keydown event:', event);
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
				} else if (event.key === 'ArrowLeft' || event.key === 'ArrowRight') {
					// Left/right will exit menu
					if (searchTerm.value.length === 0) open.value = false;
				} else if (event.key === 'Enter') {
					// Enter selects menu option

					console.log('Enter key pressed:', event);
					console.log('Enter key pressed:', promptOptions.value[active.value]);
					console.log('Enter searchTerm:', searchTerm);
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
				} else if (event.key === 'Backspace') {
					// Backspace closes menu if searchTerm is empty
					if (searchTerm.value.length === 0) open.value = false;
					else searchTerm.value = searchTerm.value.slice(0, -1);
					active.value = 0;
				}
			});

			menu.value.addEventListener('keyup', (event: KeyboardEvent) => {
				console.log('auto block menu keyup event:', event);
				if (!open.value) return;
				if (event.key === 'Enter') {
					// Enter selects menu option
					event.preventDefault();
					event.stopPropagation();
				}
			});
		}
	});

	defineExpose({
		open,
	});
</script>
