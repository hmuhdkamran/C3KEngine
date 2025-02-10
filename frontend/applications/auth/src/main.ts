import { config, installModules } from './helper/config'

import SkApp from './App.vue'

import '@unocss/reset/tailwind-compat.css'
import 'uno.css'
import './styles/main.scss'

import singleSpaVue from 'single-spa-vue'
import singleSpaCss from 'single-spa-css'
import { App } from 'vue'
import { configKey, SingleSpaKey, SingleSpaProps } from './helper/spa'
import router from './router'

const cssLifecycles = singleSpaCss({
  cssUrls: import.meta.env.DEV
    ? [`${config.APP_MICRO_URL}src/styles/main.scss`]
    : [`${config.APP_MICRO_URL}src/styles/main.scss?a=${Math.floor(Math.random() * 10000)}`],
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

// declare module '@vue/runtime-core' {
//   export interface ComponentCustomProperties {
//     $filters: any
//   }
// }

const vueLifecycles = singleSpaVue({
  createApp,
  appOptions: {
    render() {
      return h(SkApp)
    },
  },
  handleInstance: (app: App, props: SingleSpaProps) => {
    app.use(router);
    
    installModules(app, router);

    app.provide(SingleSpaKey, props)
    app.provide(configKey, config)
  },
})

export const bootstrap = [cssLifecycles.bootstrap, vueLifecycles.bootstrap]
export const mount = [cssLifecycles.mount, vueLifecycles.mount]
export const unmount = [vueLifecycles.unmount, cssLifecycles.unmount]