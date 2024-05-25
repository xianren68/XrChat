import { createRouter, createWebHistory } from 'vue-router'

const routes = [
	{
		path: '/home',
		name: 'session',
		component: () => import('@/views/Session/index.vue'),
	},
	{
		path: '/setting',
		name: 'setting',
		component: () => import('@/views/Setting/index.vue'),
	},
	{
		path: '/relation',
		name: 'relation',
		component: () => import('@/views/Relation/index.vue'),
	},
	{
		path: '/self',
		name: 'self',
		component: () => import('@/views/Self/index.vue'),
	},
	{
		path: '/add',
		name: 'add',
		component: () => import('@/views/Add/index.vue'),
	},
	{
		path: '/',
		name: 'login',
		component: () => import('@/views/Login/index.vue'),
	},
	{
		path: '/register',
		name: 'register',
		component: () => import('@/views/Register/index.vue'),
	},
]

const router = createRouter({
	history: createWebHistory(),
	routes,
})
export default router
