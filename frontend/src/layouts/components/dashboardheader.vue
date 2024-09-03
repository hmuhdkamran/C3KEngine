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
  <div class="fixed top-0 left-0 w-full px-4 sm:px-6 lg:px-8 flex h-12 items-center bg-pink-900 justify-between">
    <div class="sm:flex sm:items-center md:flex md:items-center">
      <div class="relative">
        <button @click="toggleModule"
          class="flex items-center p-2 text-gray-600 hover:text-gray-800 focus:outline-none">

          <svg class="h-8 w-8 text-white" width="24" height="24" viewBox="0 0 24 24" stroke-width="2"
            stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
            <path stroke="none" d="M0 0h24v24H0z" />
            <circle cx="5" cy="5" r="1" />
            <circle cx="12" cy="5" r="1" />
            <circle cx="19" cy="5" r="1" />
            <circle cx="5" cy="12" r="1" />
            <circle cx="12" cy="12" r="1" />
            <circle cx="19" cy="12" r="1" />
            <circle cx="5" cy="19" r="1" />
            <circle cx="12" cy="19" r="1" />
            <circle cx="19" cy="19" r="1" />
          </svg>
        </button>
      </div>
      <Sidebar @toggleModule="toggleModule" :showModuleDropdown="showModuleDropdown" />
      <a href="/" class="text-xl font-semibold text-gray-800 flex items-center space-x-2 ml-4">
        <img :src="logo" alt="C3K Engine Logo" class="h-12 w-auto object-contain" />
        <span class="text-white">C3K Engine</span>
      </a>
    </div>
    <div class="flex items-center">

      <Notifications @toggleDropdown="toggleDropdown" :isOpen="isOpen" />

      <ProfileDropdown @toggleProfileDropdown="toggleProfileDropdown" :showProfileDropdown="showProfileDropdown" />
    </div>
  </div>
</template>