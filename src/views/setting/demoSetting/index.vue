<template>
	<Form ref="formRef" :model="settingform">
		<div class="text-base font-semibold mb-2">General</div>
		<LabelBox
			title="Default view for new tabs"
			desc="The default view that a new markdown tab gets opened in.">
			<div class="w-36">
				<FormItem name="selectStr">
					<Select v-model="settingform.selectStr">
						<Option value="markdown">markdown</Option>
						<Option value="raw">raw</Option>
					</Select>
				</FormItem>
			</div>
		</LabelBox>

		<LabelBox
			title="Default editing mode"
			desc="The default editing mode a new tab will start with.">
			<div class="w-36">
				<FormItem name="selectStr">
					<Select v-model="settingform.selectStr">
						<Option value="markdown">markdown</Option>
						<Option value="raw">raw</Option>
					</Select>
				</FormItem>
			</div>
		</LabelBox>

		<LabelBox
			title="Editor status"
			desc="Display and change the current editor mode in the status bar.">
			<FormItem name="selectStr">
				<Switch />
			</FormItem>
		</LabelBox>

		<div class="text-base font-semibold mb-2 mt-6">Display</div>
		<LabelBox
			title="Readable line length"
			desc="Limit maximum line length. Less content fits onscreen, but long blocks of text are more readable.">
			<FormItem name="selectStr">
				<Switch />
			</FormItem>
		</LabelBox>

		<LabelBox
			title="Strict line breaks"
			desc="Markdown specs ignore single line breaks in reading view. Turn this off to make single line breaks visible.">
			<FormItem name="selectStr">
				<Switch />
			</FormItem>
		</LabelBox>
		<div v-for="item in 10" :key="item">
			<LabelBox
				title="Strict line breaks"
				desc="Markdown specs ignore single line breaks in reading view. Turn this off to make single line breaks visible.">
				<FormItem name="selectStr">
					<Switch />
				</FormItem>
			</LabelBox>
		</div>
	</Form>
</template>

<script lang="ts" setup>
	import { Form, FormItem, Select, Option, Switch } from '@arco-design/web-vue';
	import { reactive } from 'vue';
	import LabelBox from '../components/labelBox/index.vue';

	const settingform = reactive({
		selectStr: '',
	});
</script>
