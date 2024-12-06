<script setup lang="ts">
import { defineProps, defineEmits } from 'vue';
import { Drawer, VNodeRenderer, config, type IMenuItem } from '@/.';
import MenuItem from './MenuItem.vue';

interface Props {
  menuItems: IMenuItem[],
  toggleSidebar: boolean,
}
const props = defineProps<Props>();
const emit = defineEmits<{
  (event: 'toggle-sidebar'): void;
}>();

</script>

<template>
  <Drawer :isOpen="props.toggleSidebar" title="My Sidebar" size="w-72" position="left"
    @toggleDrawer="emit('toggle-sidebar')" class="bg-black bg-opacity-50 z-50">
    <template #header>
      <div
        class="w-full flex items-center justify-between px-6 py-2 bg-white border-b border-gray-200 shadow-sm text-gray-700 space-x-6">
        <div>
          <a href="/" class="flex items-center space-x-2 hover:text-indigo-600">
            <VNodeRenderer :nodes="config.logo" />
            <span>{{ config.application }}</span>
          </a>
        </div>
        <button @click="emit('toggle-sidebar')" class="text-gray-400 hover:text-gray-600 focus:outline-none">
          <span class="icon-[fluent--dismiss-20-filled] h-4 w-4"></span>
        </button>
      </div>
    </template>
    <div
      class="fixed inset-y-0 top-20 left-0 w-72 bg-white text-gray-800 border-r border-gray-200 shadow-lg z-40 rounded-tr-lg rounded-br-lg">
      <div class="flex flex-col h-full">
        <div class="overflow-y-auto flex-grow px-4 py-3">
          <ul class="space-y-2">
            <MenuItem v-for="(item, index) in props.menuItems" :key="index" :menuItem="item" />
          </ul>
        </div>
        <div class="px-4 py-3 bg-white border-t border-gray-200 shadow-sm mt-auto">
          <a href="#" class="flex items-center justify-center btn-primary py-2 space-x-2 text-sm ">
            <span class="icon-[ic--baseline-person] h-4 w-4"></span>
            <span>Profile</span>
          </a>
          <a href="#" class="flex items-center justify-center btn-secondary py-2 mt-3 space-x-2 text-sm">
            <span class="icon-[ic--baseline-logout] h-4 w-4"></span>
            <span>Logout</span>
          </a>
        </div>
      </div>
    </div>
  </Drawer>
</template>
