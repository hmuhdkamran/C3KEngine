<script setup lang="ts">
import { useSystemStore, Drawer } from 'c3-library';
import { ref } from 'vue';

const store = useSystemStore();

const isOpen = ref(false);

function updateColors () {
    store.updateApplication(store.application);
}

</script>

<template>
    <div class="fixed right-0 flex flex-col items-end z-50">
        <!-- Theme Palette Button -->
        <button @click="isOpen = !isOpen"
            class="p-3 cursor-pointer flex items-center justify-center rounded-l-lg shadow-md focus:outline-none transition duration-300 hover:shadow-lg"
            :style="{ backgroundColor: store.application.primaryColor }">
            <span class="fa-solid fa-palette text-white"></span>
        </button>

        <!-- Color Selection Drawer -->
        <Drawer :is-open="isOpen" title="Theme Setting" position="right" size="w-1/4">
            <template #header>
                <div class="flex items-center justify-between w-full px-4 py-3"
                    >
                    <h3 class="text-lg font-semibold text-gray-700">Select Preset</h3>
                    <button @click="isOpen = false" class="text-gray-600 hover:text-gray-500 cursor-pointer transition duration-200">
                        <span class="fa-solid fa-xmark"></span>
                    </button>
                </div>
            </template>

            <!-- Color Preset Grid -->
            <div class="p-4">
                <h2 class="mb-4">Primary Color</h2>
                <input type="color" v-model="store.application.primaryColor" @chage="updateColors" />
            </div>
            <div class="p-4">
                <h2 class="mb-4">Title Color</h2>
                <input type="color" v-model="store.application.titleColor" @chage="updateColors" />
            </div>
            <div class="p-4">
                <h2 class="mb-4">Background Color</h2>
                <input type="color" v-model="store.application.backgroundColor" @chage="updateColors" />
            </div>
            <div class="p-4">
                <h2 class="mb-4">Sidebar Color</h2>
                <input type="color" v-model="store.application.sidebarColor" @chage="updateColors" />
            </div>
        </Drawer>
    </div>
</template>

<style scoped>
/* Add any necessary styles here */
</style>