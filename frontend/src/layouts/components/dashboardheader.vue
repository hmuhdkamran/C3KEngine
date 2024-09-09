<script setup lang="ts">
import { ref } from 'vue';
import Sidebar from './sidebar.vue';
import Notifications from './notification.vue';
import ProfileDropdown from './profile.vue';
import logo from "@/assets/images/vue.svg";

const isOpen = ref(false);
const showModuleDropdown = ref(false);
const showProfileDropdown = ref(false);

function toggleModule() {
  showModuleDropdown.value = !showModuleDropdown.value;
  isOpen.value = false;
  showProfileDropdown.value = false;
}

function toggleDropdown() {
  isOpen.value = !isOpen.value;
  showProfileDropdown.value = false;
  showModuleDropdown.value = false;
}

function toggleProfileDropdown() {
  showProfileDropdown.value = !showProfileDropdown.value;
  isOpen.value = false;
  showModuleDropdown.value = false;
}
</script>

<template>
  <div class="fixed top-0 left-0 w-full px-4 sm:px-6 lg:px-8 flex h-16 items-center bg-gradient-to-r from-violet-600 to-blue-500 justify-between shadow-lg z-50">
    <div class="flex items-center">
      <button @click="toggleModule" class="flex items-center text-white hover:text-gray-200 focus:outline-none">
        <span class="icon-[fluent--navigation-unread-20-filled] h-8 w-8"></span>
      </button>
      <Sidebar @toggleModule="toggleModule" :showModuleDropdown="showModuleDropdown" />
      <a href="/" class="text-2xl font-semibold text-white flex items-center space-x-2">
        <img :src="logo" alt="C3K Engine Logo" class="h-10 w-auto object-contain" />
        <span>C3K Engine</span>
      </a>
    </div>
    <div class="flex items-center">
      <button class="flex items-center text-white hover:text-gray-200 focus:outline-none">
        <span class="icon-[ic--baseline-search] h-6 w-6"></span>
      </button>
      <Notifications @toggleDropdown="toggleDropdown" :isOpen="isOpen" />
      <ProfileDropdown @toggleProfileDropdown="toggleProfileDropdown" :showProfileDropdown="showProfileDropdown" />
    </div>
  </div>
</template>