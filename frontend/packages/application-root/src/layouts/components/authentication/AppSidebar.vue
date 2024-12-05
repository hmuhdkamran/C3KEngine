<script setup lang="ts">
import { ref } from 'vue';
import { defineProps } from 'vue';
import { Drawer, VNodeRenderer, config } from 'c3k-library';
import { store } from '@/stores';
import MenuItem from './MenuItem.vue';

interface Props {
  showSidebarDropdown: boolean
}

interface Emit {
  (e: 'toggleSidebar'): void;
}
const props = defineProps<Props>()
const emit = defineEmits<Emit>()
const openSections = ref<{ [key: number]: boolean }>({});

const toggleSection = (index: number) => {
  openSections.value[index] = !openSections.value[index];
};
</script>

<template>
  <Drawer :isOpen="props.showSidebarDropdown" title="My Sidebar" size="w-72" position="left"
    @toggleDrawer="emit('toggleSidebar')" class="bg-black bg-opacity-50 z-50">
    <template #header>
      <div
        class="w-full flex items-center justify-between px-6 py-2 bg-white border-b border-gray-200 shadow-sm text-gray-700 space-x-6">
        <div>
          <a href="/" class="flex items-center space-x-2 hover:text-indigo-600">
            <VNodeRenderer :nodes="config.logo" />
            <span>{{ config.application }}</span>
          </a>
        </div>
        <button @click="emit('toggleSidebar')" class="text-gray-400 hover:text-gray-600 focus:outline-none">
          <span class="icon-[fluent--dismiss-20-filled] h-4 w-4"></span>
        </button>
      </div>
    </template>
    <div
      class="fixed inset-y-0 top-20 left-0 w-72 bg-white text-gray-800 border-r border-gray-200 shadow-lg z-40 rounded-tr-lg rounded-br-lg">
      <div class="flex flex-col h-full">
        <div class="overflow-y-auto flex-grow px-4 py-3">
          <ul class="space-y-2">
            <MenuItem v-for="(item, index) in store.sideBarMneu" :key="index" :menuItem="item" />
          </ul>
        </div>
        <div class="px-4 py-3 bg-white border-t border-gray-200 shadow-sm mt-auto">
          <a href="/profile" class="flex items-center justify-center btn-primary py-2 space-x-2 text-sm ">
            <span class="icon-[ic--baseline-person] h-4 w-4"></span>
            <span>Profile</span>
          </a>
          <a href="/logout" class="flex items-center justify-center btn-secondary py-2 mt-3 space-x-2 text-sm">
            <span class="icon-[ic--baseline-logout] h-4 w-4"></span>
            <span>Logout</span>
          </a>
        </div>
      </div>
    </div>
  </Drawer>
</template>
<style scoped>
.slide-enter-active,
.slide-leave-active {
  transition: transform 0.4s ease;
}

.slide-enter-from,
.slide-leave-to {
  transform: translateX(-100%);
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.slide-down-enter-active,
.slide-down-leave-active {
  transition: max-height 0.3s ease, opacity 0.3s ease;
}

.slide-down-enter-from,
.slide-down-leave-to {
  max-height: 0;
  opacity: 0;
}

.slide-down-enter-to,
.slide-down-leave-from {
  max-height: 200px;
  /* Adjust as needed for the content size */
  opacity: 1;
}
</style>