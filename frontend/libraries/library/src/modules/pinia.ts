import { type App } from 'vue'
import { createPinia } from 'pinia'
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate'

export const installPinia = (app: App) => {
    if (!app._context.provides.pinia) {
      const pinia = createPinia()
      pinia.use(piniaPluginPersistedstate)
      app.use(pinia)
    }
  }