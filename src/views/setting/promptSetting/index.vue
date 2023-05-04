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
					type="text"
					placeholder="Search"
					class="block w-full py-2 pr-5 text-gray-700 bg-white border border-gray-200 rounded-lg md:w-80 placeholder-gray-400/70 pl-11 rtl:pr-11 rtl:pl-5 dark:bg-gray-900 dark:text-gray-300 dark:border-gray-600 focus:border-blue-400 dark:focus:border-blue-300 focus:ring-blue-300 focus:outline-none focus:ring focus:ring-opacity-40" />
			</div>
			<div class="flex items-center mt-4 gap-x-3">
				<button
					class="flex items-center justify-center w-1/2 px-5 py-2 text-sm text-gray-700 transition-colors duration-200 bg-white border rounded-lg gap-x-2 sm:w-auto dark:hover:bg-gray-800 dark:bg-gray-900 hover:bg-gray-100 dark:text-gray-200 dark:border-gray-700">
					<i class="fa-solid fa-cloud-arrow-up"></i>
					<span>Import</span>
				</button>

				<button
					class="flex items-center justify-center w-1/2 px-5 py-2 text-sm text-gray-700 transition-colors duration-200 bg-white border rounded-lg gap-x-2 sm:w-auto dark:hover:bg-gray-800 dark:bg-gray-900 hover:bg-gray-100 dark:text-gray-200 dark:border-gray-700">
					<i class="fa-solid fa-download"></i>
					<span>Export</span>
				</button>
				<button
					class="flex items-center justify-center w-1/2 px-5 py-2 text-sm tracking-wide text-white transition-colors duration-200 bg-blue-500 rounded-lg shrink-0 sm:w-auto gap-x-2 hover:bg-blue-600 dark:hover:bg-blue-500 dark:bg-blue-600">
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
						<Table :pagination="false" :columns="columns" :data="data">
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
									@click="handleClick(record)">
									<i class="fa-solid fa-ellipsis-vertical"></i>
								</button>
							</template>
						</Table>
						<Modal
							id="menu-prompt"
							v-model:visible="visible"
							class="prompt-box"
							width="40vw"
							title="Edit Prompt"
							@ok="handleOk"
							@cancel="handleCancel">
							<Textarea
								v-model="promptForm.content"
								:auto-size="{
									minRows: 4,
									maxRows: 20,
								}"
								style="margin: -20px 0; outline: none; border: none" />
						</Modal>
					</div>
				</div>
			</div>
		</div>
	</section>
</template>

<script setup lang="ts">
	import { Table, Modal, Textarea } from '@arco-design/web-vue';
	import { ref, reactive } from 'vue';

	const promptForm = reactive({
		content: '',
	});

	const visible = ref(false);

	const handleClick = (record: any) => {
		console.log('record:', record);
		visible.value = true;
	};
	const handleOk = () => {
		visible.value = false;
	};
	const handleCancel = () => {
		visible.value = false;
	};

	function getRandomInt(min: number, max: number) {
		return Math.floor(Math.random() * (max - min + 1)) + min;
	}

	function getRandomString(length: number) {
		let result = '';
		const characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz';
		const charactersLength = characters.length;
		for (let i = 0; i < length; i += 1) {
			result += characters.charAt(Math.floor(Math.random() * charactersLength));
		}
		return result;
	}

	function generateRandomArray(size: number): App.PromptTemplate[] {
		const array = [];
		for (let i = 0; i < size; i += 1) {
			array.push({
				key: (i + 1).toString(),
				name: getRandomString(10),
				content: `${getRandomString(80)}@example.com`,
			});
		}
		return array;
	}

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

	const data = generateRandomArray(40);
</script>

<style scoped></style>
