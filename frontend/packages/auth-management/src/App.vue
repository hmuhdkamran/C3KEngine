<script setup lang="ts">
import { Notification, UseAxios, useNotification } from 'c3k-library';
import { computed } from 'vue';

UseAxios();
const { notifications } = useNotification();

const notificationsWithOffsets = computed(() =>
  notifications.value.map((notification: any, index: number) => ({
    ...notification,
    positionOffset: index * 110
  }))
);

</script>

<template>
  <div class="bg-gray-50 font-inter tracking-tight text-gray-900 antialiased">
    <Notification v-for="notification in notificationsWithOffsets" :key="notification.id" :title="notification.title"
      :message="notification.message" :type="notification.type" :position="notification.position"
      :positionOffset="notification.positionOffset" :duration="300" />
    <router-view />
  </div>
</template>

<style scoped></style>
