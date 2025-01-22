<script setup lang="ts">
import logo from '@/assets/logo.svg';

import { useSidebar } from '@/stores/useSidebar';
import { useColorPalette } from '@/stores/colorPalette';


const { state } = useColorPalette();

const { isSidebarOpen } = useSidebar();

const props = defineProps({
    bgColor: String,
});

const toggleSidebar = () => {
    isSidebarOpen.value = !isSidebarOpen.value;
};
</script>

<template>
    <div :style="{ backgroundColor: state.selectedColor }"
        class="shadow-md p-3 flex items-center justify-between sticky top-0 z-10 border-b border-gray-200">
        <div class="flex items-center justify-between space-x-4">
            <div class="flex items-center w-full space-x-2 mr-10">
                <img :src="logo" alt="Logo" class="h-8 w-8" />
                <h2 v-if="isSidebarOpen" class="text-md font-semibold text-gray-700">
                    Ultimate ERP Solution {{ state.selectedColor }}
                </h2>
            </div>

            <button @click="toggleSidebar" class="text-gray-50 flex items-center justify-center">
                <span
                    :class="isSidebarOpen ? 'icon-[fluent--navigation-16-filled]' : 'icon-[fluent--navigation-16-filled]'"
                    class="h-5 w-5"></span>
            </button>

            <div class="flex items-center relative w-full">
                <button
                    class="text-gray-600 hover:text-blue-500 transition duration-200 absolute flex items-center justify-center w-8 h-8">
                    <span class="icon-[mdi--magnify] h-4 w-4"></span>
                </button>
                <input type="text" placeholder="Type for Search..."
                    class="pl-9 py-1 w-max-sm text-sm border border-gray-300 rounded-sm focus:outline-none focus:ring-1 focus:ring-blue-500 transition duration-200" />
            </div>
        </div>

        <div class="flex items-center space-x-4">
            <div class="relative">
                <button class="relative text-gray-600 hover:text-blue-500 transition duration-200">
                    <span class="icon-[mdi--bell] h-5 w-5"></span>
                    <div class="absolute top-0 right-0 h-2 w-2 bg-red-500 rounded-full"></div>
                </button>
            </div>

            <button class="text-gray-600 hover:text-blue-500 transition duration-200">
                <span class="icon-[mdi--message] h-5 w-5"></span>
            </button>

            <button class="text-gray-600 hover:text-blue-500 transition duration-200">
                <span class="icon-[mdi--user] h-5 w-5"></span>
            </button>

            <button class="text-gray-600 hover:text-blue-500 transition duration-200">
                <span class="icon-[mdi--cog] h-5 w-5"></span>
            </button>
        </div>
    </div>
</template>
