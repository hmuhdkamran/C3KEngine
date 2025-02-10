<script setup lang="ts">
import type { ToastNotification } from '~/store/notify.store'

const notification = useNotification()
const notificationsStore = useNotifyStore()
watch(() => notificationsStore.messages, (newVal: ToastNotification[], oldVal: ToastNotification[]) => {
  if (newVal.length < oldVal.length)
    return
  const lastMessage = newVal[newVal.length - 1]
  if (lastMessage !== undefined) {
    notification.create({
      type: lastMessage.type,
      content: lastMessage.body,
      duration: !lastMessage.permanent ? lastMessage.duration : undefined,
      closable: !lastMessage.permanent,
    })
  }
}, { deep: true })
</script>

<template>
  <main class="text-gray-700 dark:text-gray-200">
    <RouterView v-slot="{ Component }">
      <Transition name="fade" mode="out-in">
        <Component :is="Component" />
      </Transition>
    </RouterView>
  </main>
</template>