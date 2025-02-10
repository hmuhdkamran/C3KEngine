import { acceptHMRUpdate, defineStore } from 'pinia'
import type { Account, LoginViewModel } from '~/models/Account'
import { AuthenticationService } from '~/services/authentication-service'

export const useAccountStore = defineStore('account', () => {
  const service: AuthenticationService = new AuthenticationService();
  const user = ref<Account | null>()
  const isLoading = ref(false)
  const loginFailed = ref(false)

  async function login(loginInfo: LoginViewModel): Promise<boolean> {
    isLoading.value = true
    try {
      const response = await service.login(loginInfo)
      if (response?.authenticated) {
        return true
      }

      return false
    }
    catch (error) {
      return false
    }
    finally {
      isLoading.value = false
    }
  }

  function logout() {
    user.value = null
  }

  async function register(registerInfo: any) {
    isLoading.value = true
    try {
      const response = await service.register(registerInfo)
      if (response) {
        return true
      }

      return false
    }
    catch (error) {
      return false
    }
    finally {
      isLoading.value = false
    }
  }

  function resetPassword(forgetInfo: any) {
    return Promise.resolve(true)
  }

  function isAuthenticated() {
    return (user.value?.token && user.value.token !== null) ?? false
  }

  return {
    user,
    isLoading,
    loginFailed,
    login,
    logout,
    isAuthenticated,
    resetPassword,
    register,
  }
}, { persist: true })

if (import.meta.hot)
  import.meta.hot.accept(acceptHMRUpdate(useAccountStore, import.meta.hot))
