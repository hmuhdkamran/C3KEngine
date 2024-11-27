import './assets/styles/main.css'

import { createApp } from 'vue'

import App from './App.vue'
import router from './router'

import { installPinia } from 'c3k-library';

const app = createApp(App)

installPinia(app);
app.use(router);

app.mount('#app')
