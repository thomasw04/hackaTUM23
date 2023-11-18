import { createApp } from 'vue'
import { createRouter, createWebHistory } from 'vue-router'
import './style.css'
import App from './App.vue'
import Search from './components/Search.vue'
import Map from './components/Map.vue'
import PostcodeInputPage from './components/PostcodeInputPage.vue'

const router = createRouter({
	history: createWebHistory(),
	routes: [
		{ path: '/', component: PostcodeInputPage },
		{ path: '/search', component: Search},
		{ path: '/map', component: Map }
	],
})

createApp(App).use(router).mount('#app')
