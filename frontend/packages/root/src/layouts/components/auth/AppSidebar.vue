<script setup lang="ts">
import { ref } from 'vue';
import { defineProps } from 'vue';
import { Drawer, VNodeRenderer, config } from 'c3k-library';
import sidebarMenu from '@/navigation/sidebarConfig';

interface Props {
  showSidebarDropdown: boolean
}

const props = defineProps<Props>()
const openSections = ref<{ [key: number]: boolean }>({});

const toggleSection = (index: number) => {
  openSections.value[index] = !openSections.value[index];
};
</script>

<template>
  <Drawer :isOpen="props.showSidebarDropdown" size="w-80" @toggleDrawer="emit('toggleDrawer')">
    <template #header>
      <div class="flex items-center justify-between px-5 py-4 bg-white border-b border-gray-200 shadow-sm text-indigo-600">
        <a href="/" class="flex items-center space-x-2">
          <VNodeRenderer :nodes="config.logo" />
          <span>{{ config.application }}</span>
        </a>
      </div>
    </template>
    <div
      class="fixed inset-y-0 top-28 left-0 w-80 bg-white text-gray-800 border-r border-gray-200 shadow-lg z-50 rounded-tr-lg rounded-br-lg">
      <div class="flex flex-col h-full">
        <div class="overflow-y-auto flex-grow px-4 py-3">
          <ul class="space-y-2">
            <li v-for="(section, index) in sidebarMenu" :key="index">
              <div @click="toggleSection(index)"
                class="flex items-center justify-between px-3 py-2 rounded-md cursor-pointer bg-gray-50 hover:text-violet-700 hover:bg-violet-50 transition-all duration-200 ease-in-out">
                <span class="flex items-center space-x-2">
                  <span :class="`${section.icon} h-5 w-5 text-violet-600`"></span>
                  <span class="text-sm font-semibold text-gray-700">{{ section.title }}</span>
                </span>
                <span
                  :class="openSections[index] ? 'icon-[mdi--chevron-up] text-gray-600' : 'icon-[mdi--chevron-down] text-gray-500'"></span>
              </div>
              <transition name="slide-down">
                <ul v-show="openSections[index]" class="mt-2 pl-4 space-y-1">
                  <li v-for="(item, subIndex) in section.items" :key="subIndex">
                    <router-link :to="item.link"
                      class="flex items-center space-x-2 py-1 px-3 rounded-md text-sm text-gray-600 hover:text-violet-700 hover:bg-violet-50 hover:border-l-2 border-violet-500 transition-all duration-200 ease-in-out">
                      <span>{{ item.name }}</span>
                    </router-link>
                  </li>
                </ul>
              </transition>
            </li>
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