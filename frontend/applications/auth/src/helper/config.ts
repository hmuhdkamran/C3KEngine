import { App } from "vue"
import { Router } from "vue-router"
import { AppModule } from "~/types"

export interface Configuration {
    APP_API_URL: string
    APP_MICRO_URL: string
}

export const config: Configuration = {
    APP_API_URL: import.meta.env.VITE_APP_API_URL as string,
    APP_MICRO_URL: `http://${import.meta.env.VITE_APP_MICRO_SERVER}:${import.meta.env.VITE_APP_MICRO_PORT}/` as string,
}

export const installModules = (app: App, router: Router) => {
    Object.values(import.meta.glob<{ install: AppModule }>('../modules/*.ts', { eager: true }))
        .forEach(i => i.install?.(app, router))

    // register filters
    app.config.globalProperties.$filters = {}
    Object.values(import.meta.glob<any>('../common/filters/*.filter.ts', { eager: true, import: 'default' }))
        .forEach(filters => Object.keys(filters).forEach(func => app.config.globalProperties.$filters[func] = filters[func]))
}