<script setup lang="ts">
import { Notification, UseAxios, useNotification, type IUser, type IMenuItem } from 'c3k-library';
import { computed } from 'vue';
import { store } from '@/stores/micro';

const props = defineProps<{
  setGlobalState?: (state: {
    user: IUser,
    userModules: any[],
    sideBarMenu: IMenuItem[],
    toggleSidebar: boolean
  }) => void;
}>();
const handleClick = () => {
  if (props.setGlobalState) {
    store.toggleSidebar = !store.toggleSidebar
    props.setGlobalState({
      user: store.user,
      userModules: store.userModules,
      sideBarMenu: store.sideBarMenu,
      toggleSidebar: store.toggleSidebar,
    });
  }
};

UseAxios();

console.log(JSON.stringify(store));

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
    <!-- <AppSidebar :menu-items="store.sideBarMenu" :toggleSidebar="store.toggleSidebar" @toggle-sidebar="handleClick()" /> -->
    <!-- <AuthHeader :showSidebarDropdown="true" class="z-100" /> -->
    <Notification v-for="notification in notificationsWithOffsets" :key="notification.id" :title="notification.title"
      :message="notification.message" :type="notification.type" :position="notification.position"
      :positionOffset="notification.positionOffset" :duration="9000" />
      <button @click="store.toggleSidebar = !store.toggleSidebar">{{ store.toggleSidebar }}</button>      
    <router-view />
  </div>
</template>

<style scoped></style>
