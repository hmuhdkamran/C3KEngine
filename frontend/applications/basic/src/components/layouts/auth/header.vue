<script setup lang="ts">
import logo from '@/assets/logo.svg';
import { useSidebar } from '@/stores/useSidebar';
import { selectColor } from '@/stores/colorPalette';

const { isSidebarOpen } = useSidebar();

const toggleSidebar = () => {
    isSidebarOpen.value = !isSidebarOpen.value;
};
</script>

<template>
    <div :style="{ backgroundColor: selectColor() }"
        class="shadow-md p-3 flex items-center justify-between sticky top-0 z-10 border-b border-gray-200">
        <div class="flex items-center justify-between space-x-4">
            <div class="flex items-center w-full space-x-2 mr-10">
                <img :src="logo" alt="Logo" class="h-8 w-8" />
                <h2 v-if="isSidebarOpen" class="text-md font-semibold text-gray-50">
                    Ultimate ERP Solution
                </h2>
            </div>

            <button @click="toggleSidebar" class="text-gray-50 flex items-center justify-center">
                <span :class="isSidebarOpen ? 'fa-solid fa-bars' : 'fa-solid fa-bars'"></span>
            </button>

            <div class="flex items-center text-gray-50 relative w-full">
                <button class=" transition duration-200 absolute flex items-center justify-center w-8 h-8">
                    <span class="fa-solid fa-magnifying-glass fa-sm"></span>
                </button>
                <input type="text" placeholder="Type for Search..."
                    class="pl-9 py-1 text-gray-50 w-96 text-sm bg-transparent hover:bg-gray-50/10 rounded-sm focus:outline-none focus:ring-1 focus:ring-gray-500 transition duration-200" />
            </div>
        </div>

        <div class="flex items-center space-x-4">
            <div class="relative">
                <button class="relative text-gray-50 hover:text-gray-100 transition duration-200">
                    <span class="fa-solid fa-bell"></span>
                    <div class="absolute top-0 right-0 h-2 w-2 bg-red-400 rounded-full"></div>
                </button>
            </div>

            <button class="text-gray-50 hover:text-gray-100 transition duration-200">
                <span class="fa-solid fa-message"></span>
            </button>

            <button class="text-gray-50 hover:text-gray-100 transition duration-200">
                <span class="fa-solid fa-user"></span>
            </button>

            <button class="text-gray-50 hover:text-gray-100 transition duration-200">
                <span class="fa-solid fa-gear"></span>
            </button>
        </div>
    </div>
</template>
