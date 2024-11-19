import { ref } from 'vue'
import { defineStore } from 'pinia'

export interface IApplicationStore {
  toggleDrawer: boolean
};

export const useApplicationStore = defineStore('application', {
  state: (): IApplicationStore => ({
    toggleDrawer: false
  }),
})
