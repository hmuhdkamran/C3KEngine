<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from 'vue';
import Moduledropdown from './ModuleMenu.vue';
import Notifications from './UserNotification.vue';
import ProfileDropdown from './UserProfile.vue';
import { store } from '@/stores';
import { PubSub } from 'c3k-library';

const dropdownStates = ref({
  sidebar: false,
  module: false,
  profile: false,
  notifications: false,
});
const isSmallScreen = ref(false);

defineProps<{ selectedCardTitle: string }>();

// General toggle function for dropdowns
function toggleDropdown(key: keyof typeof dropdownStates.value) {
  if (key === 'sidebar') {
    store.toggleSidebar = !store.toggleSidebar;
    const event = new CustomEvent('custom-event', { detail: store.toggleSidebar });
    window.dispatchEvent(event);
    return;
  }

  // Close all dropdowns first, then toggle the selected one
  Object.keys(dropdownStates.value).forEach((state) => {
    dropdownStates.value[state as keyof typeof dropdownStates.value] = false;
  });
  dropdownStates.value[key] = !dropdownStates.value[key];
}

// Handle screen size changes
const checkScreenSize = () => {
  isSmallScreen.value = window.innerWidth < 640;
};

// Close all dropdowns
function closeAllDropdowns() {
  Object.keys(dropdownStates.value).forEach((key) => {
    dropdownStates.value[key as keyof typeof dropdownStates.value] = false;
  });
}

// Handle clicks outside specific elements
function useClickOutside(callback: () => void) {
  const handleClickOutside = (event: MouseEvent) => {
    const target = event.target as HTMLElement;
    if (!target.closest('.dropdown, .notification, .profile')) {
      callback();
    }
  };

  onMounted(() => {
    document.addEventListener('click', handleClickOutside);
    checkScreenSize();
    window.addEventListener('resize', checkScreenSize);
  });

  onBeforeUnmount(() => {
    document.removeEventListener('click', handleClickOutside);
    window.removeEventListener('resize', checkScreenSize);
  });
}

useClickOutside(closeAllDropdowns);
</script>

<template>
  <div
    class="fixed top-0 left-0 w-full px-2 sm:px-6 lg:px-8 flex items-center bg-gradient-to-r from-indigo-500 to-violet-600 justify-between h-11 z-40">
    <div class="flex items-center space-x-2 sm:space-x-4">
      <button @click="() => toggleDropdown('sidebar')" class="p-1 text-white hover:text-gray-300 focus:outline-none">
        <span class="icon-[fluent--navigation-unread-20-filled] h-6 w-6"></span>
      </button>
      <Moduledropdown :selectedCardTitle="selectedCardTitle" :showModuleDropdown="dropdownStates.module"
        class="dropdown" @toggleModuleDropdown="() => toggleDropdown('module')" />
      <nav class="hidden md:flex space-x-6">
        <RouterLink to="/dashboard" class="text-white text-sm hover:text-gray-200 transition duration-200">
          Home Dashboard {{ store.toggleSidebar }}
        </RouterLink>
      </nav>
    </div>
    <div class="flex items-center space-x-2">
      <Notifications @toggleNotifications="() => toggleDropdown('notifications')" :isOpen="dropdownStates.notifications"
        :isSmallScreen="isSmallScreen" class="notification" />
      <template v-for="icon in ['chat', 'help', 'settings']" :key="icon">
        <button class="hidden sm:block p-1 text-white hover:text-gray-300 focus:outline-none">
          <span :class="`icon-[ic--baseline-${icon}] h-4 w-4 sm:h-5 sm:w-5`"></span>
        </button>
      </template>
      <ProfileDropdown @toggleProfileDropdown="() => toggleDropdown('profile')"
        :showProfileDropdown="dropdownStates.profile" class="profile" />
    </div>
  </div>
</template>

<style scoped>
@media (max-width: 768px) {
  nav {
    display: none;
  }
}

@media (min-width: 768px) {
  .mobile-menu-button {
    display: none;
  }
}

@media (max-width: 640px) {
  .h-10 {
    height: 8vh;
  }

  .text-sm {
    font-size: 0.75rem;
  }
}
</style>
