<script setup lang="ts">
import { ref } from 'vue';
import Sidebar from './sidebar.vue';
import Moduledropdown from './moduledropdown.vue';
import Notifications from './notification.vue';
import ProfileDropdown from './profile.vue';
import logo from "@/assets/images/vue.svg";

const isOpen = ref(false);
const showSidebarDropdown = ref(false);
const showModuleDropdown = ref(false);
const showProfileDropdown = ref(false);

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

</script>

<template>
  <div
    class="fixed top-0 left-0 w-full sm:px-6 lg:px-8 flex items-center justify-between h-16 bg-gradient-to-r from-violet-600 to-indigo-500 shadow-md z-50">
    <div class="flex items-center space-x-4 sm:space-x-6">
      <button @click="toggleSidebar" class="p-2 text-white hover:text-gray-300 focus:outline-none">
        <span class="icon-[fluent--navigation-unread-20-filled] h-8 w-8"></span>
      </button>
      <Moduledropdown 
        :selectedCardTitle="selectedCardTitle" 
        :showModuleDropdown="showModuleDropdown" 
        @toggleModuleDropdown="toggleModuleDropdown"
      />
      <Sidebar @toggleSidebar="toggleSidebar" :showSidebarDropdown="showSidebarDropdown" />
      <nav class="hidden md:flex space-x-4">
        <a href="#" class="text-white font-medium hover:text-gray-200 transition duration-200">CRM Dashboard</a>
        <a href="#" class="text-white font-medium hover:text-gray-200 transition duration-200">Sales Overview</a>
        <a href="#" class="text-white font-medium hover:text-gray-200 transition duration-200">Customer Engagement</a>
        <a href="#" class="text-white font-medium hover:text-gray-200 transition duration-200">Pipeline Management</a>
      </nav>
    </div>
    <div class="flex items-center">
      <Notifications @toggleNotifications="toggleNotifications" :isOpen="isOpen" />
      <button class="p-2 text-white hover:text-gray-300 focus:outline-none">
        <span class="icon-[ic--baseline-chat] h-5 w-5 sm:h-6 sm:w-6"></span>
      </button>
      <button class="p-2 text-white hover:text-gray-300 focus:outline-none">
        <span class="icon-[ic--baseline-help] h-5 w-5 sm:h-6 sm:w-6"></span>
      </button>
      <button class="p-2 text-white hover:text-gray-300 focus:outline-none">
        <span class="icon-[ic--baseline-settings] h-5 w-5 sm:h-6 sm:w-6"></span>
      </button>
      <ProfileDropdown @toggleProfileDropdown="toggleProfileDropdown" :showProfileDropdown="showProfileDropdown" />
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
</style>