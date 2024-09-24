import type { AxiosError, AxiosRequestConfig, AxiosResponse } from 'axios'
import Axios from 'axios'
import { useRouter } from 'vue-router'
import { TokenHelper } from './token-helper'

let initialized = false

function handle(status: number, exclude: number[]) {
    if (exclude.length === 0)
        return true
    else return exclude.find(o => o === status) === undefined
}

export function UseAxios() {
    const router = useRouter()

    if (!initialized) {
        Axios.interceptors.request.use((config: AxiosRequestConfig | any) => {
            if (!config.headers.Authorization) {
                const bearerToken = TokenHelper.getBearerToken()

                if (bearerToken.Authorization)
                    Object.assign(config.headers, bearerToken)
            }

            if (!config.maxRedirects || config.maxRedirects === 5)
                config.maxRedirects = 0

            return config
        })

        Axios.interceptors.response.use(undefined, (config: AxiosError) => {
            const response: AxiosResponse | undefined = config.response
            const exclude: any = []

            if (response?.status === 401 && handle(response?.status, exclude)) {
                const location: string
                    = response?.headers.location || response?.headers.Location

                if (location) {
                    const redirectTo = `/${location}`

                    window.setTimeout(() => router.replace(redirectTo), 200)
                }
            }

            if (response?.status === 403 && handle(response?.status, exclude))
                window.setTimeout(() => router.replace('/forbidden'), 200)

            return config
        })

        initialized = true
    }
}