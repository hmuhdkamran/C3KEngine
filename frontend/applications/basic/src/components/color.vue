<script setup lang="ts">
import { useThemePaletteStore, Drawer } from 'c3-library';
import { ref } from 'vue';

const store = useThemePaletteStore();

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
                    <h3 class="text-lg font-semibold">Select Preset</h3>
                    <button @click="isOpen = false" class="text-white hover:text-gray-300 transition duration-200">
                        <span class="fa-solid fa-xmark"></span>
                    </button>
                </div>
            </template>

            <!-- Color Preset Grid -->
            <div class="p-4">
                <h2 class="mb-4">Color Presets</h2>
                <input type="color" v-model="store.selectedColor" />
            </div>
        </Drawer>
    </div>
</template>

<style scoped>
/* Add any necessary styles here */
</style>