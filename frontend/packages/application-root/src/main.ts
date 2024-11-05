import './assets/styles/main.css'

import { createApp } from 'vue'
import { createPinia } from 'pinia'

import App from './App.vue'
import router from './router'

import { installPinia } from 'c3k-library';

const app = createApp(App)

app.use(createPinia())
app.use(router)

installPinia(app);

app.mount('#app')
