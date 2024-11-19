import { ref } from 'vue'
import { defineStore } from 'pinia'

export interface IApplicationEvent {
  ToggleDrawer: boolean
}

export const useApplicationEventStore = defineStore('application', {
  state: (): IApplicationEvent =>({
    ToggleDrawer: false
  })
})