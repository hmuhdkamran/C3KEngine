import { ref } from 'vue'
import { acceptHMRUpdate, defineStore } from 'pinia'

import type { IUser } from '../models'
import { DefaultUser } from '../models'
import { TokenHelper } from '../helpers'

export interface ICommonState {
  isLoading: boolean
  user: IUser
  toggleSidebar: boolean
}

export const useSystemStore = defineStore('system', () => {
  const isLoading = ref(false);
  const user = ref<IUser>(DefaultUser);
  const toggleSidebar = ref(false);

  function updateLoading(loading: boolean) {
    isLoading.value = loading
  }

  function updateUser(usr: IUser | string) {
    if (typeof usr === 'string')
      user.value = TokenHelper.parseUserToken(usr)
    else
      user.value = usr
  }

  function updateToggleSidebar(changed: boolean) {
    toggleSidebar.value = changed
  }

  return {
    user,
    isLoading,
    toggleSidebar,
    updateLoading,
    updateUser,
    updateToggleSidebar
  }
}, { persist: true })

if (import.meta.hot)
  import.meta.hot.accept(acceptHMRUpdate(useSystemStore, import.meta.hot))
