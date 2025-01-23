<script setup lang="ts">
import { colors, setColor, selectColor } from '@/stores/colorPalette';
import { ref } from 'vue';

const isOpen = ref(false);
</script>

<template>
    <div class="fixed top-20 right-0 flex flex-col items-end z-50">
        <button @click="isOpen = !isOpen"
            class="p-2 text-white rounded-l-lg shadow-md focus:outline-none transition-colors duration-300 flex items-center justify-center"
            :style="{ backgroundColor: selectColor() }">
            <span class="icon-[ic--round-color-lens] w-6 h-6"></span>
        </button>

        <div v-if="isOpen" class="absolute top-16 right-0 w-64 p-3 bg-white rounded-l-lg shadow-lg z-50">
            <div class="flex justify-between items-center mb-4">
                <h3 class="text-lg font-medium text-gray-700">Select Color</h3>
                <button @click="isOpen = false" class="text-gray-500 hover:text-gray-700 transition duration-200">
                    <span class="icon-[ic--round-close] w-5 h-5"></span>
                </button>
            </div>

            <div class="grid grid-cols-5 gap-2">
                <div v-for="color in colors" :key="color" :style="{ backgroundColor: color }"
                    class="relative w-10 h-10 rounded-full cursor-pointer transition-transform duration-300 ease-in-out transform hover:translate-y-[-2px] hover:ring-2 hover:ring-gray-300"
                    @click="setColor(color)">
                    <span v-if="selectColor() === color"
                        class="absolute inset-0 flex items-center justify-center text-white text-lg">
                        <span class="icon-[ic--round-check] w-6 h-6"></span>
                    </span>
                </div>
            </div>
        </div>
        <div v-if="isOpen" class="fixed top-0 left-0 w-full h-full bg-gray-900 opacity-40 z-40" @click="isOpen = false">
        </div>
    </div>
</template>

<style scoped></style>