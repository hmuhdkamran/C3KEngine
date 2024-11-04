import './assets/styles/main.css'
import 'c3k-library/style.css';

import { createApp } from 'vue'
import { createPinia } from 'pinia'

import App from './App.vue'
import { routeHash } from './router'
import { installPinia } from 'c3k-library';

const app = createApp(App)

app.use(createPinia())

installPinia(app);

app.use(routeHash('/'))

app.mount('#app')
