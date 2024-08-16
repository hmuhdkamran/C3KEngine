<script setup lang="ts">
import { defineProps, defineEmits } from 'vue';

defineProps<{ showProfileDropdown: boolean }>();
defineEmits(['toggleProfileDropdown']);

import avatar from "@/assets/images/avatar.jpg"
</script>

<template>
  <div class="relative">
    <button @click="$emit('toggleProfileDropdown')"
      class="flex items-center p-2 text-gray-600 hover:text-gray-800 focus:outline-none gap-1">
      <img :src="avatar" alt="Profile" class="w-9 h-9 rounded-full border-2 border-gray-200" />
      <span class="text-white">Mitchell Admin</span>
      <span class="icon-[ri--arrow-drop-down-line] text-white h-7 w-7"></span>
    </button>
    <transition name="dropdown">
      <div v-if="showProfileDropdown"
        class="absolute right-0 mt-2 w-64 bg-white shadow-lg rounded-lg z-10 overflow-hidden">
        <div class="px-4 py-3 bg-gray-50 border-b border-gray-200">
          <div class="flex items-center">
            <img :src="avatar" alt="Profile" class="w-12 h-12 rounded-full border-2 border-gray-200" />
            <div class="ml-3">
              <p class="text-sm font-semibold text-gray-800">Mitchell Admin</p>
              <p class="text-sm text-gray-600">admin@sefam.com</p>
            </div>
          </div>
        </div>
        <div class="py-2">
          <ul class="space-y-2">
            <li>
              <a href="#" class="flex items-center px-4 py-2 text-gray-800 hover:bg-violet-100 rounded-lg">
                <span class="icon-[ic--baseline-person] bg-violet-600 h-5 w-5 mr-3"></span>
                <span>Profile</span>
              </a>
            </li>
            <li>
              <a href="#" class="flex items-center px-4 py-2 text-gray-800 hover:bg-violet-100 rounded-lg">
                <span class="icon-[ic--baseline-settings] text-violet-500 h-5 w-5 mr-3"></span>
                <span>Settings</span>
              </a>
            </li>
            <li>
              <a href="#" class="flex items-center px-4 py-2 text-gray-800 hover:bg-violet-100 rounded-lg">
                <span class="icon-[ic--baseline-history] text-violet-500 h-5 w-5 mr-3"></span>
                <span>Activity Log</span>
              </a>
            </li>
            <li>
              <a href="#" class="flex items-center px-4 py-2 text-gray-800 hover:bg-violet-100 rounded-lg">
                <span class="icon-[ic--baseline-help] text-violet-500 h-5 w-5 mr-3"></span>
                <span>Help Center</span>
              </a>
            </li>
          </ul>
          <hr />
          <ul class="mt-2">
            <li>
              <a href="#" class="flex items-center px-4 py-2 text-gray-800 hover:bg-violet-100 rounded-lg">
                <span class="icon-[ic--baseline-settings-applications] text-violet-500 h-5 w-5 mr-3"></span>
                <span>Account Settings</span>
              </a>
            </li>
            <li>
              <a href="#" class="flex items-center px-4 py-2 text-gray-800 hover:bg-violet-100 rounded-lg">
                <span class="icon-[ic--baseline-logout] text-violet-500 h-5 w-5 mr-3"></span>
                <span>Logout</span>
              </a>
            </li>
          </ul>
        </div>
      </div>

    </transition>
  </div>
</template>

<style scoped>
.dropdown-enter-active,
.dropdown-leave-active {
  transition: opacity 0.3s ease;
}

.dropdown-enter-from,
.dropdown-leave-to {
  opacity: 0;
}
</style>