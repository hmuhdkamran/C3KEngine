import { createApp } from 'vue'
import { createPinia } from 'pinia'
import router from "@/router";

import '@/assets/styles/main.css';
import './public-path'

import App from './App.vue'
import { installPinia } from 'c3k-library';

const pinia = createPinia();
const app = createApp(App);

app.use(pinia)
app.use(router)

installPinia(app);

app.mount('#app')
