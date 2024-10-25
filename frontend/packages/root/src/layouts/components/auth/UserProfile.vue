<script setup lang="ts">
import { defineProps, defineEmits } from 'vue';
import { useSystemStore } from "c3k-library";
import avatar from "@/assets/images/avatar.jpg"

defineProps<{ showProfileDropdown: boolean }>();
defineEmits(['toggleProfileDropdown']);

const store = useSystemStore()

</script>

<template>
  <div class="relative">
    <button @click="$emit('toggleProfileDropdown')"
      class="flex items-center p-1 text-gray-600 hover:text-gray-800 focus:outline-none gap-1">
      <img :src="avatar" alt="Profile" class="w-8 h-8 rounded-full border-2 border-gray-200" />
      <span class="icon-[ri--arrow-drop-down-line] text-white h-5 w-5"></span>
    </button>
    <transition name="dropdown">
      <div v-if="showProfileDropdown"
        class="absolute right-0 mt-2 w-48 bg-white shadow-lg rounded-md z-10 overflow-hidden">
        <div class="px-3 py-2 bg-gray-50 border-b border-gray-200">
          <div class="flex items-center">
            <img :src="avatar" alt="Profile" class="w-9 h-9 rounded-full border-2 border-gray-200" />
            <div class="ml-2">
              <p class="text-sm font-semibold text-gray-800">{{ store.user.displayName }}</p>
              <p class="text-sm text-gray-600">{{ store.user.email }}</p>
            </div>
          </div>
        </div>
        <div class="py-2">
          <ul class="space-y-2">
            <li>
              <a href="#"
                class="flex items-center space-x-2 py-1 px-3 rounded-md text-sm text-gray-600 hover:text-violet-700 hover:bg-violet-50 hover:border-l-2 border-violet-500 transition-all duration-200 ease-in-out">
                <span class="icon-[ic--baseline-person] bg-violet-600 h-4 w-4 mr-2"></span>
                <span class="text-sm">Profile</span>
              </a>
            </li>
            <li>
              <a href="#"
                class="flex items-center space-x-2 py-1 px-3 rounded-md text-sm text-gray-600 hover:text-violet-700 hover:bg-violet-50 hover:border-l-2 border-violet-500 transition-all duration-200 ease-in-out">
                <span class="icon-[ic--baseline-settings] text-violet-500 h-4 w-4 mr-2"></span>
                <span class="text-sm">Settings</span>
              </a>
            </li>
            <li>
              <a href="#"
                class="flex items-center space-x-2 py-1 px-3 rounded-md text-sm text-gray-600 hover:text-violet-700 hover:bg-violet-50 hover:border-l-2 border-violet-500 transition-all duration-200 ease-in-out">
                <span class="icon-[ic--baseline-history] text-violet-500 h-4 w-4 mr-2"></span>
                <span class="text-sm">Activity Log</span>
              </a>
            </li>
            <li>
              <a href="#"
                class="flex items-center space-x-2 py-1 px-3 rounded-md text-sm text-gray-600 hover:text-violet-700 hover:bg-violet-50 hover:border-l-2 border-violet-500 transition-all duration-200 ease-in-out">
                <span class="icon-[ic--baseline-help] text-violet-500 h-4 w-4 mr-2"></span>
                <span class="text-sm">Help Center</span>
              </a>
            </li>
          </ul>
          <hr class="my-1 border-gray-200" />
          <ul class="mt-1">
            <li>
              <a href="#"
                class="flex items-center space-x-2 py-1 px-3 rounded-md text-sm text-gray-600 hover:text-violet-700 hover:bg-violet-50 hover:border-l-2 border-violet-500 transition-all duration-200 ease-in-out">
                <span class="icon-[ic--baseline-settings-applications] text-violet-500 h-4 w-4 mr-2"></span>
                <span class="text-sm">Account Settings</span>
              </a>
            </li>
            <li>
              <a href="#"
                class="flex items-center space-x-2 py-1 px-3 rounded-md text-sm text-gray-600 hover:text-violet-700 hover:bg-violet-50 hover:border-l-2 border-violet-500 transition-all duration-200 ease-in-out">
                <span class="icon-[ic--baseline-logout] text-violet-500 h-4 w-4 mr-2"></span>
                <span class="text-sm">Logout</span>
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