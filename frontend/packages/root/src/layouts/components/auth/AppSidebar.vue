<script setup lang="ts">
import { ref } from 'vue';
import { defineProps, defineEmits } from 'vue';
import { config, VNodeRenderer } from 'c3k-library';
import sidebarMenu from '@/navigation/sidebarConfig';

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
  <transition name="slide">
    <div v-if="props.showSidebarDropdown"
    class="fixed inset-y-0 left-0 w-64 bg-white text-gray-800 border-r border-gray-200 shadow-lg z-50 rounded-tr-lg rounded-br-lg">
      <div class="flex flex-col h-full">
        <div class="flex items-center justify-between px-5 py-4 bg-white border-b border-gray-200 shadow-sm">
          <a href="/" class="flex items-center space-x-2">
            <VNodeRenderer :nodes="config.logo" />{{ config.application }}
          </a>
          <button @click="emit('toggleSidebar')" class="text-gray-400 hover:text-gray-600 focus:outline-none">
            <span class="icon-[fluent--dismiss-20-filled] h-5 w-5"></span>
          </button>
        </div>
        <div class="overflow-y-auto flex-grow px-4 py-3">
          <ul class="space-y-2">
            <li v-for="(section, index) in sidebarMenu" :key="index">
              <div @click="toggleSection(index)"
                class="flex items-center justify-between px-3 py-2 rounded-md cursor-pointer bg-gray-50 hover:text-violet-700 hover:bg-violet-50 transition-all duration-200 ease-in-out">
                <span class="flex items-center space-x-2">
                  <span :class="`${section.icon} h-5 w-5 text-violet-600`"></span>
                  <span class="text-sm font-semibold text-gray-700">{{ section.title }}</span>
                </span>
                <span :class="openSections[index] ? 'icon-[mdi--chevron-up] text-gray-600' : 'icon-[mdi--chevron-down] text-gray-500'"></span>
              </div>
              <ul v-show="openSections[index]" class="mt-2 pl-4 space-y-1">
                <li v-for="(item, subIndex) in section.items" :key="subIndex">
                  <router-link :to="item.link"
                    class="flex items-center space-x-2 py-1 px-3 rounded-md text-sm text-gray-600 hover:text-violet-700 hover:bg-violet-50 hover:border-l-2 border-violet-500 transition-all duration-200 ease-in-out">
                    <span>{{ item.name }}</span>
                  </router-link>
                </li>
              </ul>
            </li>
          </ul>
        </div>
        <div class="px-4 py-3 bg-white border-t border-gray-200 shadow-sm mt-auto">
          <a href="/profile"
            class="flex items-center justify-center btn-primary py-2 space-x-2 text-sm ">
            <span class="icon-[ic--baseline-person] h-4 w-4"></span>
            <span>Profile</span>
          </a>
          <a href="/logout"
            class="flex items-center justify-center btn-secondary py-2 mt-3 space-x-2 text-sm">
            <span class="icon-[ic--baseline-logout] h-4 w-4"></span>
            <span>Logout</span>
          </a>
        </div>
      </div>
    </div>
  </transition>
  <transition name="fade">
    <div v-if="props.showSidebarDropdown" class="fixed inset-0 bg-black bg-opacity-50 z-40"
      @click="emit('toggleSidebar')"></div>
  </transition>
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
</style>