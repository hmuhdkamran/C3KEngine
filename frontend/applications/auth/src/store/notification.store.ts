import { acceptHMRUpdate, defineStore } from 'pinia'
import type { Notification } from '~/models/Notification'

export const useNotificationStore = defineStore('Notification', () => {
  const notifications = ref<Notification[]>([])
  const isLoading = ref(false)
  async function getNotifications() {
    notifications.value = []
  }

  function clearAll() {
    notifications.value = []
  }

  return {
    notifications,
    isLoading,
    getNotifications,
    clearAll,
  }
})

if (import.meta.hot)
  import.meta.hot.accept(acceptHMRUpdate(useNotificationStore, import.meta.hot))
