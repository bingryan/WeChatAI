<template>
	<main
		class="grid min-h-full place-items-center bg-white px-6 py-24 sm:py-32 lg:px-8">
		<div class="text-center">
			<p class="text-base font-semibold text-indigo-600">404</p>
			<h1
				class="mt-4 text-3xl font-bold tracking-tight text-gray-900 sm:text-5xl">
				Page not found
			</h1>
			<div class="mt-10 flex items-center justify-center gap-x-6">
				<a
					class="rounded-md bg-indigo-600 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
					@click="goHome"
					>Go back home</a
				>
			</div>
		</div>
	</main>
</template>

<script setup lang="ts">
	import { useRouter } from 'vue-router';

	const router = useRouter();

	function goHome() {
		router.push('/');
	}
</script>
