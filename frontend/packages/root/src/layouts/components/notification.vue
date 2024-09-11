<script setup lang="ts">
import { defineProps, defineEmits } from 'vue';

defineProps<{ isOpen: boolean }>();
defineEmits(['toggleDropdown']);
</script>

<template>
  <div @click.stop="$emit('toggleDropdown')" class="relative">
    <button class="btn btn-ghost btn-circle">
      <div class="flex items-center p-2 text-gray-600 hover:text-gray-800 focus:outline-none">
        <span class="icon-[mdi--bell-outline] text-white h-6 w-6"></span>
        <span
          class="absolute top-0 right-0 block w-4 h-4 text-xs text-white bg-red-600 rounded-full flex items-center justify-center">4</span>
      </div>
    </button>
    <transition name="dropdown">
      <div v-if="isOpen" class="absolute right-0 mt-2 w-80 bg-white shadow-lg rounded-lg overflow-hidden z-50">
        <div class="py-2">
          <div class="px-4 py-3 bg-gray-100 border-b border-gray-200">
            <h3 class="text-lg font-semibold text-gray-800">Notifications</h3>
          </div>
          <div class="max-h-60 overflow-y-auto">
            <div class="px-4 py-3 flex items-start space-x-3 border-b border-gray-200">
              <span class="icon-[mdi--email] text-blue-500 h-5 w-5"></span>
              <div>
                <p class="text-sm text-gray-700">You have a new message from John Doe</p>
                <span class="text-xs text-gray-500">2 minutes ago</span>
              </div>
            </div>

            <div class="px-4 py-3 flex items-start space-x-3 border-b border-gray-200">
              <span class="icon-[mdi--bell-ring] text-yellow-500 h-5 w-5"></span>
              <div>
                <p class="text-sm text-gray-700">Reminder: Meeting at 3 PM</p>
                <span class="text-xs text-gray-500">10 minutes ago</span>
              </div>
            </div>

            <div class="px-4 py-3 flex items-start space-x-3 border-b border-gray-200">
              <span class="icon-[mdi--file-document] text-green-500 h-5 w-5"></span>
              <div>
                <p class="text-sm text-gray-700">New report available: Sales Summary</p>
                <span class="text-xs text-gray-500">30 minutes ago</span>
              </div>
            </div>

            <div class="px-4 py-3 flex items-start space-x-3 border-b border-gray-200">
              <span class="icon-[mdi--account] text-purple-500 h-5 w-5"></span>
              <div>
                <p class="text-sm text-gray-700">Jane Smith updated her profile</p>
                <span class="text-xs text-gray-500">1 hour ago</span>
              </div>
            </div>
          </div>

        </div>
        <div class="px-4 py-3 bg-gray-100 text-center">
          <button class="text-sm text-blue-500 hover:underline">View All</button>
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