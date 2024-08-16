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
  <div class="fixed top-0 left-0 w-full px-4 sm:px-6 lg:px-8 flex h-12 items-center bg-violet-500 justify-between">
    <div class="md:flex md:items-center">
      <div class="relative">
        <button @click="toggleModule" class="flex items-center p-2 text-gray-600 hover:text-gray-800 focus:outline-none">
          <span class="icon-[fluent--navigation-unread-20-filled] text-white h-7 w-7"></span>
        </button>
      </div>
      <Sidebar @toggleModule="toggleModule" :showModuleDropdown="showModuleDropdown" />
      <a href="/" class="text-xl font-semibold text-gray-800 flex items-center space-x-2 ml-4">
        <img :src="logo" alt="C3K Engine Logo" class="h-12 w-auto object-contain" />
        <span class="text-white">C3K Engine</span>
      </a>
    </div>
    <div class="flex items-center">
      <button class="btn btn-ghost btn-circle">
        <div class="flex items-center p-2 text-gray-600 hover:text-gray-800 focus:outline-none">
          <span class="icon-[ic--baseline-search] text-white h-6 w-6"></span>
        </div>
      </button>
      <Notifications @toggleDropdown="toggleDropdown" :isOpen="isOpen" />
      <ProfileDropdown @toggleProfileDropdown="toggleProfileDropdown" :showProfileDropdown="showProfileDropdown" />
    </div>
  </div>
</template>