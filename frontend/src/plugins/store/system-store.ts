import { newGuid } from '../helper'
import type { IPayloadMessage, IUser } from '../models'
import { DefaultUser } from '../models'
import { TokenHelper } from '../helper/token-helper'
import { defineStore } from 'pinia'

export interface ICommonState {
  isLoading: boolean
  user: IUser
  messages: Map<string, IPayloadMessage>
  passwordChanged: boolean
}

export const useSystemStore = defineStore('system', {
  state: (): ICommonState => ({
    isLoading: false,
    user: DefaultUser,
    messages: new Map(),
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

    updateMsg(msg: IPayloadMessage) {
      const messageId = newGuid()

      this.messages.set(messageId, msg)
      setTimeout(() => this.removeMessage(messageId), 3000)
    },

    removeMessage(messageId: string) {
      this.messages.delete(messageId)
    },
  },
})
