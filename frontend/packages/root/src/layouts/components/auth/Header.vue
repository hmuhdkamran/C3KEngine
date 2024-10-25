<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from 'vue';
import Sidebar from './AppSidebar.vue';
import Moduledropdown from './ModuleMenu.vue';
import Notifications from './UserNotification.vue';
import ProfileDropdown from './UserProfile.vue';

const isOpen = ref(false);
const showSidebarDropdown = ref(false);
const showModuleDropdown = ref(false);
const showProfileDropdown = ref(false);
const isSmallScreen = ref(false);

defineProps<{ selectedCardTitle: string }>();

function toggleSidebar() {
  showSidebarDropdown.value = !showSidebarDropdown.value;
  isOpen.value = false;
  showProfileDropdown.value = false;
  showModuleDropdown.value = false;
}

function toggleModuleDropdown() {
  showModuleDropdown.value = !showModuleDropdown.value;
  isOpen.value = false;
  showSidebarDropdown.value = false;
  showProfileDropdown.value = false;
}

function toggleNotifications() {
  isOpen.value = !isOpen.value;
  showProfileDropdown.value = false;
  showSidebarDropdown.value = false;
  showModuleDropdown.value = false;
}

function toggleProfileDropdown() {
  showProfileDropdown.value = !showProfileDropdown.value;
  isOpen.value = false;
  showSidebarDropdown.value = false;
  showModuleDropdown.value = false;
}

const checkScreenSize = () => {
  isSmallScreen.value = window.innerWidth < 640;
};

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

useClickOutside(() => {
  showModuleDropdown.value = false;
  isOpen.value = false;
  showProfileDropdown.value = false;
});

</script>

<template>
  <div
    class="fixed top-0 left-0 w-full px-4 sm:px-6 lg:px-8 flex items-center justify-between h-11 bg-gradient-to-r from-violet-600 to-indigo-500 shadow-[0_4px_10px_rgba(0,0,0,0.3)] z-50">
    <div class="flex items-center space-x-2 sm:space-x-4">
      <Sidebar @toggleSidebar="toggleSidebar" :showSidebarDropdown="showSidebarDropdown" />
      <button @click="toggleSidebar" class="p-1 text-white hover:text-gray-300 focus:outline-none">
        <span class="icon-[fluent--navigation-unread-20-filled] h-6 w-6"></span>
      </button>
      <Moduledropdown :selectedCardTitle="selectedCardTitle" :showModuleDropdown="showModuleDropdown" class="dropdown"
        @toggleModuleDropdown="toggleModuleDropdown" />
      <nav class="hidden justify-between md:flex space-x-6">
        <a href="#" class="text-white text-sm hover:text-gray-200 transition duration-200">CRM Dashboard</a>
        <a href="#" class="text-white text-sm hover:text-gray-200 transition duration-200">Sales Overview</a>
        <a href="#" class="text-white text-sm hover:text-gray-200 transition duration-200">Customer Engagement</a>
        <a href="#" class="text-white text-sm hover:text-gray-200 transition duration-200">Pipeline Management</a>
      </nav>
    </div>
    <div class="flex items-center space-x-2">
      <Notifications @toggleNotifications="toggleNotifications" :isOpen="isOpen" :isSmallScreen="isSmallScreen"
        class="notification" />
      <button class="hidden sm:block p-1 text-white hover:text-gray-300 focus:outline-none">
        <span class="icon-[ic--baseline-chat] h-4 w-4 sm:h-5 sm:w-5"></span>
      </button>
      <button class="hidden sm:block p-1 text-white hover:text-gray-300 focus:outline-none">
        <span class="icon-[ic--baseline-help] h-4 w-4 sm:h-5 sm:w-5"></span>
      </button>
      <button class="hidden sm:block p-1 text-white hover:text-gray-300 focus:outline-none">
        <span class="icon-[ic--baseline-settings] h-4 w-4 sm:h-5 sm:w-5"></span>
      </button>
      <ProfileDropdown @toggleProfileDropdown="toggleProfileDropdown" :showProfileDropdown="showProfileDropdown"
        class="profile" />
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
