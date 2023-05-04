<template>
	<section class="container px-4 mx-auto">
		<div class="sm:flex sm:items-center sm:justify-between">
			<div>
				<div class="flex items-center gap-x-3">
					<h2 class="text-lg font-medium text-gray-800 dark:text-white"
						>Prompt Management</h2
					>
				</div>

				<p class="mt-1 text-sm text-gray-500 dark:text-gray-300"
					>prompt template for you input.</p
				>
			</div>
		</div>

		<div class="mt-6 md:flex md:items-center md:justify-between">
			<div class="flex items-center mt-4">
				<span class="relative -mr-8">
					<i
						class="fa-solid fa-magnifying-glass w-4 h-4 mx-2 text-gray-400 dark:text-gray-600"></i>
				</span>

				<input
					v-model="search"
					type="text"
					placeholder="Search"
					class="block w-full py-2 pr-5 text-gray-700 bg-white border border-gray-200 rounded-lg md:w-80 placeholder-gray-400/70 pl-11 rtl:pr-11 rtl:pl-5 dark:bg-gray-900 dark:text-gray-300 dark:border-gray-600 focus:border-blue-400 dark:focus:border-blue-300 focus:ring-blue-300 focus:outline-none focus:ring focus:ring-opacity-40" />
			</div>
			<div class="flex items-center mt-4 gap-x-3">
				<Upload
					ref="uploadRef"
					action="/"
					:auto-upload="false"
					:show-file-list="false"
					@change="onFileChange">
					<template #upload-button>
						<button
							class="flex items-center justify-center w-1/2 px-5 py-2 text-sm text-gray-700 transition-colors duration-200 bg-white border rounded-lg gap-x-2 sm:w-auto dark:hover:bg-gray-800 dark:bg-gray-900 hover:bg-gray-100 dark:text-gray-200 dark:border-gray-700">
							<i class="fa-solid fa-cloud-arrow-up"></i>
							<span>Import</span>
						</button>
					</template>
				</Upload>

				<button
					class="flex items-center justify-center w-1/2 px-5 py-2 text-sm text-gray-700 transition-colors duration-200 bg-white border rounded-lg gap-x-2 sm:w-auto dark:hover:bg-gray-800 dark:bg-gray-900 hover:bg-gray-100 dark:text-gray-200 dark:border-gray-700"
					@click="handleSaveToJson">
					<i class="fa-solid fa-download"></i>
					<span>Export</span>
				</button>
				<button
					class="flex items-center justify-center w-1/2 px-5 py-2 text-sm tracking-wide text-white transition-colors duration-200 bg-blue-500 rounded-lg shrink-0 sm:w-auto gap-x-2 hover:bg-blue-600 dark:hover:bg-blue-500 dark:bg-blue-600"
					@click="handleAddPromptClick">
					<i class="fa-sharp fa-solid fa-plus"></i>
					<span>Add</span>
				</button>
			</div>
		</div>
		<div class="flex flex-col mt-6">
			<div class="-mx-4 -my-2 overflow-x-auto sm:-mx-6 lg:-mx-8">
				<div class="inline-block min-w-full py-2 align-middle md:px-6 lg:px-8">
					<div
						class="overflow-hidden border border-gray-200 dark:border-gray-700 md:rounded-lg">
						<Table :pagination="false" :columns="columns" :data="promptList">
							<template #content="{ record }">
								<Textarea
									:default-value="record.content"
									:auto-size="{
										minRows: 2,
										maxRows: 5,
									}"
									style="outline: none; border: none" />
							</template>
							<template #optional="{ record }">
								<button
									class="px-1 py-1 text-right text-gray-500 transition-colors duration-200 rounded-lg dark:text-gray-300 hover:bg-gray-100"
									@click="handleEditPromptClick(record)">
									<i class="fa-solid fa-ellipsis-vertical"></i>
								</button>
							</template>
						</Table>
						<Modal
							id="menu-prompt"
							v-model:visible="visible"
							:mask-closable="false"
							:closable="false"
							class="prompt-box"
							width="40vw"
							title="Edit Prompt"
							:ok-text="newPromptModal ? 'Add' : 'Save'"
							:cancel-text="newPromptModal ? 'Cancel' : 'Delete'"
							@ok="handleOk"
							@cancel="handleCancel">
							<template v-if="newPromptModal" #title>
								{{ $t('common.newPrompt') }}</template
							>
							<template v-else #title> {{ $t('common.editPrompt') }} </template>
							<Form ref="formRef" :model="promptForm">
								<div class="mb-1">
									<label
										for="name"
										class="block text-sm font-medium text-gray-900 dark:text-white"
										>Name</label
									>
									<Input v-model="promptForm.name" default-value="new prompt" />
								</div>

								<div class="mb-1">
									<label
										for="content"
										class="block text-sm font-medium text-gray-900 dark:text-white"
										>Content</label
									>
									<div class="w-full">
										<Textarea
											v-model="promptForm.content"
											show-word-limit
											:auto-size="{
												minRows: 3,
												maxRows: 10,
											}"
											style="outline: none; border: none" />
									</div>
								</div>
							</Form>
						</Modal>
					</div>
				</div>
			</div>
		</div>
	</section>
</template>

<script setup lang="ts">
	import {
		Table,
		Modal,
		Input,
		Textarea,
		Form,
		Upload,
		Notification,
		FileItem,
	} from '@arco-design/web-vue';
	import { ref, reactive, computed } from 'vue';
	import { usePromptStore } from '@/store';
	import { invoke } from '@tauri-apps/api/tauri';

	const promptStore = usePromptStore();
	const file = ref<FileItem | null>(null);

	const visible = ref(false);
	const newPromptModal = ref(true);
	const search = ref('');
	const promptList = computed(() => {
		const searchValue = search.value.trim();
		if (searchValue) {
			const templateList = promptStore.getTemplate;
			return templateList.filter((template) => {
				const { name, content } = template;
				return name.includes(searchValue) || content.includes(searchValue);
			});
		}
		return promptStore.getTemplate;
	});

	const promptForm = reactive({
		key: '',
		name: 'new prompt',
		content: '',
	});

	const handleAddPromptClick = () => {
		newPromptModal.value = true;
		visible.value = true;
	};

	const handleEditPromptClick = (record: any) => {
		newPromptModal.value = false;
		visible.value = true;
		promptForm.key = record.key;
		promptForm.name = record.name;
		promptForm.content = record.content;
	};
	const handleOk = () => {
		const { name, content } = promptForm;
		if (newPromptModal.value) {
			const key = Date.now().toString();
			promptStore.addTemplate({ key, name, content });
		} else {
			promptStore.updateTemplate(promptForm);
		}
		visible.value = false;
	};
	const handleCancel = () => {
		if (!newPromptModal.value) {
			promptStore.removeTemplate(promptForm.key);
		}
		visible.value = false;
	};

	const columns = [
		{
			title: 'Name',
			dataIndex: 'name',
		},
		{
			title: 'Content',
			slotName: 'content',
			align: 'center' as 'center' | 'left' | 'right' | undefined,
		},
		{
			title: 'Optional',
			slotName: 'optional',
			align: 'right' as 'center' | 'left' | 'right' | undefined,
		},
	];

	const submitUpload = async () => {
		try {
			if (!file.value?.file) {
				return;
			}
			const fileReader = new FileReader();
			fileReader.readAsText(file.value.file);
			fileReader.onload = () => {
				const { result } = fileReader;
				if (typeof result === 'string') {
					const templateList: App.PromptTemplate[] = JSON.parse(result);
					let key = Date.now().toString();
					templateList.forEach((template) => {
						const { name, content } = template;
						const data = {
							key,
							name,
							content,
						};
						promptStore.addTemplate(data);
						// key must unique
						key = `${+key + 1}`;
					});
				}
			};
		} catch (error) {
			console.log('submitUpload error:', error);
		}
	};
	const onFileChange = (_: any, currentFile: any) => {
		file.value = {
			...currentFile,
		};
		submitUpload();
	};

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
		const data: Partial<App.PromptTemplate>[] = promptList.value;
		data.forEach((item) => {
			delete item.key;
		});
		const title = `prompt-${Date.now()}`;
		saveToFile(`${title}.json`, JSON.stringify(data));
	};
	const handleSaveToJson = () => {
		exportChatToJson();
	};
</script>

<style scoped></style>
