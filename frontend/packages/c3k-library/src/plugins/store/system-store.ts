import { defineStore } from 'pinia'

import type { IUser } from '../models'
import { DefaultUser } from '../models'
import { TokenHelper } from '../helper/token-helper'

export interface ICommonState {
  isLoading: boolean
  user: IUser
  passwordChanged: boolean
}

export const useSystemStore = defineStore('system', {
  state: (): ICommonState => ({
    isLoading: false,
    user: DefaultUser,
    passwordChanged: false,
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

    updatePasswordChanged(changed: boolean) {
      this.passwordChanged = changed
    },
  },
})
