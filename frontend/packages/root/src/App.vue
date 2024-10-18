<script setup lang="ts">
import { UseAxios, Notification, useNotification } from 'c3k-library';

UseAxios();
const { notifications } = useNotification();

</script>

<template>
  <div class="bg-gray-50 font-inter tracking-tight text-gray-900 antialiased">
    <Notification v-for="notification in notifications" :key="notification.id" :message="notification.message"
      :type="notification.type" :position="notification.position" :duration="3000" />
    <router-view />
  </div>
</template>

<style scoped></style>
