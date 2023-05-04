/// <reference types="vite/client" />

// vue
declare module '*.vue' {
	import type { DefineComponent } from 'vue';

	const component: DefineComponent<object, object, any>;
	export default component;
}

// markdown-it-task-lists
declare module 'markdown-it-task-lists' {
	const classes: any;
	export default classes;
}

// handlebars-helpers
declare module 'handlebars-helpers' {
	const classes: any;
	export default classes;
}
