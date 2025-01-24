import './assets/main.css'

import { type App, createApp, h } from 'vue'

import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import * as Icons from '@element-plus/icons-vue';
import PlusProComponents from 'plus-pro-components'
import 'plus-pro-components/index.css'

import singleSpaVue from 'single-spa-vue'
import singleSpaCss from 'single-spa-css'

import SkApp from './App.vue'
import router from './router'
import { config } from './helpers/config'
import { configKey, SingleSpaKey, type SingleSpaProps } from './models'
import { installPinia } from 'c3-library'

const cssLifecycles = singleSpaCss({
  cssUrls: import.meta.env.DEV
    ? [`${config.APP_MICRO_URL}src/assets/main.css`]
    : [`${config.APP_MICRO_URL}src/assets/main.css?a=${Math.floor(Math.random() * 10000)}`],
  webpackExtractedCss: false,
  shouldUnmount: false,
  timeout: 5000,
  createLink(url) {
    const linkEl = document.createElement('link')
    linkEl.rel = 'stylesheet'
    linkEl.href = url
    return linkEl
  },
})
const vueLifecycles = singleSpaVue({
  createApp,
  appOptions: {
    render() {
      return h(SkApp)
    },
  },
  handleInstance: (app: App, props: SingleSpaProps) => {
    app.use(router)
    installPinia(app);
    app.provide(SingleSpaKey, props)
    app.provide(configKey, config)
    app.use(ElementPlus)
    app.use(PlusProComponents)
    for (const [key, component] of Object.entries(Icons)) {
      app.component(key, component);
    }
  },
})

// Render directly in development mode
// if (import.meta.env.DEV) {
//   const app = createApp(SkApp)
//   app.use(createPinia()).use(router)
//   app.provide(SingleSpaKey, {})
//   app.provide(configKey, config)
//   app.mount('#app')
// }

export const bootstrap = [cssLifecycles.bootstrap, vueLifecycles.bootstrap]
export const mount = [cssLifecycles.mount, vueLifecycles.mount]
export const unmount = [vueLifecycles.unmount, cssLifecycles.unmount]
