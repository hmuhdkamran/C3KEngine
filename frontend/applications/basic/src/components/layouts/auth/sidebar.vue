<script setup lang="ts">
import { useRouter } from 'vue-router';
import logo from '@/assets/logo.svg';
import { useSidebar } from '@/stores/useSidebar';
import { useSystemStore, Drawer } from 'c3-library';
import { menuItems } from '@/stores/menuData';
import { AuthenticationService } from '@/services/authentication-service';

import MenuItem from './menu-item.vue';
import UserProfile from './user-profile.vue';
import SidebarButton from './sidebar-button.vue';
import { computed, ref } from 'vue';

const { isSidebarOpen, toggleSidebar } = useSidebar();

const isLogoutDialogOpen = ref(false);
const isProcessingLogout = ref(false);
const isSmallScreen = ref(window.innerWidth <= 500 ? true : false);
const router = useRouter();
const store = useSystemStore();

const buttons = [
  { icon: 'fa-right-from-bracket', tooltip: 'Logout', action: 'openLogoutDialog', show: true, showOnCollapsed: true },
  { icon: 'fa-gear', tooltip: 'Settings', show: true, showOnCollapsed: false },
  { icon: 'fa-square-question', tooltip: 'Help', show: true, showOnCollapsed: false },
  { icon: 'fa-comment', tooltip: 'Chat', show: true, showOnCollapsed: false },
  { icon: 'fa-lock', tooltip: 'Password', show: true, showOnCollapsed: false },
  { icon: 'fa-bell', tooltip: 'Feeds', show: true, showOnCollapsed: false },
];

interface Item {
  label: string;
  icon: string;
  link: string;
}
const items: Item[] = [
  { label: 'Profile', icon: 'fa-solid fa-user', link: '/profile' },
  { label: 'Password', icon: 'fa-solid fa-lock', link: '/password' },
  { label: 'Feeds', icon: 'fa-solid fa-bell', link: '/feeds' },
  { label: 'Settings', icon: 'fa-solid fa-gear', link: '/settings' },
  { label: 'Help', icon: 'fa-solid fa-square-question', link: '/help' },
  { label: 'Chat', icon: 'fa-solid fa-comment', link: '/chat' },
];

const openLogoutDialog = () => (isLogoutDialogOpen.value = true);
const cancelLogout = () => (isLogoutDialogOpen.value = false);

const auth_repo = new AuthenticationService();

const confirmLogout = () => {
  isProcessingLogout.value = true;
  let response = auth_repo.logout();

  if (response) {
    isProcessingLogout.value = false;
    isLogoutDialogOpen.value = false;
    router.push('/authentication/login');
  }
};

</script>

<template>
  <aside v-if="!isSmallScreen" :class="{ 'w-64': isSidebarOpen, 'w-16': !isSidebarOpen }"
    class="shadow-md flex flex-col relative transition-all duration-500 ease-[cubic-bezier(0.4, 0.0, 0.2, 1)] z-50"
    :style="{ backgroundColor: store.application.sidebarColor }">
    <div class="flex items-center p-3 space-x-2">
      <img :src="logo" alt="Logo" class="h-8 w-8" />
      <h2 v-if="isSidebarOpen"
        class="hidden md:block text-md font-semibold text-gray-800 whitespace-nowrap overflow-hidden text-ellipsis">
        {{ store.application.name }}
      </h2>
    </div>
    <nav class="flex-1 mt-4">
      <ul class="text-sm">
        <menu-item v-for="item in menuItems" :key="item.name" :item="item" />
      </ul>
    </nav>
    <div class="p-4 border-t border-gray-200 fixed bottom-0 left-0">
      <UserProfile :items="items" />
      <div class="button-group">
        <div class="flex flex-col space-y-2 mt-4">
          <div class="flex" :class="{ 'space-x-2': isSidebarOpen, 'space-y-2': !isSidebarOpen }">
            <SidebarButton v-for="(btn, index) in buttons" :key="index" :icon="btn.icon" :tooltip="btn.tooltip"
              :showOnCollapsed="btn.showOnCollapsed" @click="btn.action === 'openLogoutDialog' && openLogoutDialog()" />
          </div>
        </div>
      </div>
    </div>

    <div v-if="isLogoutDialogOpen" class="fixed inset-0 bg-gray-900/50 flex items-center justify-center z-50">
      <div class="bg-white rounded-sm shadow-md w-1/4 p-6 text-center">
        <h3 class="text-lg font-bold mb-4">Confirm Logout</h3>
        <p class="text-gray-600 mb-6">Are you sure you want to log out?</p>
        <div class="flex justify-center space-x-4">
          <button class="px-4 py-2 rounded-sm text-white transition cursor-pointer" @click="confirmLogout"
            :style="{ backgroundColor: store.application.primaryColor }" :disabled="isProcessingLogout">
            <span v-if="!isProcessingLogout">Yes, Logout</span>
            <span v-else class="flex items-center">
              <span class="fa-duotone fa-light fa-spinner-scale animate-spin mr-2"></span>Logging Out...
            </span>
          </button>
          <button class="px-4 py-2 cursor-pointer rounded-sm text-gray-800 bg-gray-200 hover:bg-gray-300 transition"
            @click="cancelLogout" :disabled="isProcessingLogout">
            Cancel
          </button>
        </div>
      </div>
    </div>
  </aside>

  <Drawer v-else :is-open="isSidebarOpen" title="" position="left" size="w-full" @click="toggleSidebar">
    <template #header>
      <div class="flex items-center justify-between w-full px-4 py-3">
        <div class="flex items-center w-full space-x-2 mr-10">
          <img :src="logo" alt="Logo" class="h-8 w-8" />
          <h2 v-if="isSidebarOpen" class="text-md font-semibold text-gray-800">
            {{ store.application.name }}
          </h2>
        </div>
        <button @click="toggleSidebar" class="text-gray-800 hover:text-gray-500 transition cursor-pointer duration-200">
          <span class="fa-solid fa-xmark"></span>
        </button>
      </div>
    </template>

    <nav class="flex-1 mt-4">
      <ul class="text-sm">
        <menu-item v-for="item in menuItems" :key="item.name" :item="item" />
      </ul>
    </nav>
  </Drawer>
</template>

<style lang="css" scoped>
.tooltip {
  display: none;
  transform: scale(0.9);
  opacity: 0;
  transition: all 0.3s ease-in-out;
}

.group:hover .tooltip {
  display: block;
  transform: scale(1);
  opacity: 1;
}

.dropdown-enter-active,
.dropdown-leave-active {
  transition: all 0.3s ease-in-out;
}

.dropdown-enter-from,
.dropdown-leave-to {
  opacity: 0;
  transform: translateY(-10px);
}

@media (max-width: 768px) {
  aside {
    display: none;
  }
}
</style>
