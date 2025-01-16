import type { Configuration } from '@/models'

export const config: Configuration = {
  APP_API_URL: import.meta.env.VITE_APP_API_URL as string,
  APP_MICRO_URL: `http://${import.meta.env.VITE_APP_MICRO_SERVER}:${import.meta.env.VITE_APP_MICRO_PORT}/` as string,
}
