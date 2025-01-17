import { createPinia } from 'pinia'
import { type App } from 'vue'

export const installPinia = (app: App) => {
  if (!app._context.provides.pinia) {
    const pinia = createPinia()
    app.use(pinia)
  }
}

export * from '@/axios'
export * from '@/models'
export * from '@/helpers'
export * from '@/service'
export * from '@/router'
export * from '@/store'

export * from '@/axios'
export * from '@/config'
export * from '@/typings'

export * from '@/composables'

export * from '@/components'

export {
  requiredValidator,
  emailValidator,
  passwordValidator,
  lengthValidator,
  alphaValidator,
  confirmedValidator,
} from '@/helpers/validators'
