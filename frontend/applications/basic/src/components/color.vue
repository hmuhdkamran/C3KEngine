<script setup lang="ts">
import { useThemePalleteStore, Drawer } from 'c3-library';
import { ref } from 'vue';

const store = useThemePalleteStore();
const isOpen = ref(false);

</script>

<template>
    <div class="fixed right-0 flex flex-col items-end z-50">
        <!-- Theme Palette Button -->
        <button @click="isOpen = !isOpen"
            class="p-3 cursor-pointer flex items-center justify-center rounded-l-lg shadow-md focus:outline-none transition duration-300 hover:shadow-lg"
            :style="{ backgroundColor: store.selectedColor }">
            <span class="fa-solid fa-palette text-white"></span>
        </button>

        <!-- Color Selection Drawer -->
        <Drawer :is-open="isOpen" title="Theme Setting" position="right" size="w-1/4">
            <template #header>
                <div class="flex items-center justify-between w-full px-4 py-3"
                    :style="{ backgroundColor: store.selectedColor, color: 'white' }">
                    <h3 class="text-lg font-semibold">Select Color</h3>
                    <button @click="isOpen = false" class="text-white hover:text-gray-300 transition duration-200">
                        <span class="fa-solid fa-xmark"></span>
                    </button>
                </div>
            </template>

            <!-- Color Palette Grid -->
            <div class="p-4">
                <h2>Color</h2>
                <div class="grid grid-cols-5 gap-4 m-10">
                    <div v-for="color in store.colors" :key="color" :style="{ backgroundColor: color }"
                        class="relative w-10 h-10 rounded-full cursor-pointer transition-transform duration-300 ease-in-out transform hover:scale-105"
                        @click="store.setSelectedColor(color)">
                        <!-- Checkmark for Selected Color -->
                        <span v-if="store.selectedColor === color"
                            class="absolute inset-0 flex items-center justify-center text-white text-lg">
                            <span class="fa-solid fa-check"></span>
                        </span>
                    </div>
                </div>
            </div>
        </Drawer>
    </div>
</template>

<style scoped></style>