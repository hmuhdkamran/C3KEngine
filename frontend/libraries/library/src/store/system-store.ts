import { defineStore } from 'pinia'

import type { IUser } from '../models'
import { DefaultUser } from '../models'
import { TokenHelper } from '../helpers'

export interface ICommonState {
  isLoading: boolean
  user: IUser
  toggleSidebar: boolean
}

export const useSystemStore = defineStore('system', {
  state: (): ICommonState => ({
    isLoading: false,
    user: DefaultUser,
    toggleSidebar: false,
  }),

  actions: {
    updateLoading(loading: boolean) {
      this.isLoading = loading
    },

    updateUser(user: IUser | string) {
      if (typeof user === 'string')
        this.user = TokenHelper.parseUserToken(user)
      else
        this.user = user
    },

    updateToggleSidebar(changed: boolean) {
      this.toggleSidebar = changed
    },
  },
})
