import type { App } from 'vue';
import type { RouteRecordRaw } from 'vue-router';
import { createRouter, createWebHashHistory } from 'vue-router';
import defaultLayout from '@/views/layout/Index.vue';

const routes: RouteRecordRaw[] = [
	{
		path: '/',
		name: 'Root',
		component: defaultLayout,
		redirect: '/chat',
		children: [
			{
				path: '/chat/:id?',
				name: 'chat',
				component: () => import('@/views/chat/Index.vue'),
			},
		],
	},
	{
		path: '/403',
		name: '403',
		component: () => import('@/views/exception/403/Index.vue'),
	},

	{
		path: '/404',
		name: '404',
		component: () => import('@/views/exception/404/Index.vue'),
	},

	{
		path: '/:pathMatch(.*)*',
		name: 'notFound',
		redirect: '/404',
	},
];

export const router = createRouter({
	history: createWebHashHistory(),
	routes,
	scrollBehavior: () => ({ left: 0, top: 0 }),
});

export async function setupRouter(app: App) {
	app.use(router);
	await router.isReady();
}
