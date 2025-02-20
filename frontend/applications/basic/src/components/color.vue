<script setup lang="ts">
import { useSystemStore, Drawer } from 'c3-library'
import { ref } from 'vue'

const store = useSystemStore()

const isOpen = ref(false)

function updateSettings() {
  document.documentElement.style.setProperty('--primary-color', store.application.primaryColor);
  document.documentElement.style.setProperty('--title-color', store.application.titleColor);
  document.documentElement.style.setProperty('--background-color', store.application.backgroundColor);
  document.documentElement.style.setProperty('--sidebar-color', store.application.sidebarColor);
  document.documentElement.style.setProperty('--font-size', store.application.fontSize);
  document.documentElement.style.setProperty('--font-family', store.application.fontFamily);
}

// Save configuration function
function saveConfiguration() {
  store.updateApplication({
    ...store.application,
    primaryColor: store.application.primaryColor,
    titleColor: store.application.titleColor,
    backgroundColor: store.application.backgroundColor,
    sidebarColor: store.application.sidebarColor,
    fontSize: store.application.fontSize,
    fontFamily: store.application.fontFamily,
  });
}
</script>

<template>
  <div class="fixed right-0 flex flex-col items-end z-50">
    <!-- Theme Palette Button -->
    <button
      @click="isOpen = !isOpen"
      class="p-3 cursor-pointer flex items-center justify-center rounded-l-lg shadow-md focus:outline-none transition duration-300 hover:shadow-lg"
      :style="{ backgroundColor: store.application.primaryColor }"
    >
      <span class="fa-solid fa-palette text-white"></span>
    </button>

    <!-- Color Selection Drawer -->
    <Drawer :is-open="isOpen" title="Theme Settings" position="right" size="w-1/3">
      <template #header>
        <div class="flex items-center justify-between w-full px-4 py-3">
          <h3 class="text-lg font-semibold text-gray-700">Theme Settings</h3>
          <button
            @click="isOpen = false"
            class="text-gray-600 hover:text-gray-500 cursor-pointer transition duration-200"
          >
            <span class="fa-solid fa-xmark"></span>
          </button>
        </div>
      </template>

      <!-- Color Presets -->
      <div class="px-4">
        <h2 class="mb-4 font-semibold text-gray-700">Color Presets</h2>
        <div class="grid grid-cols-2 gap-4">
          <div>
            <label class="text-sm block mb-2">Primary Color</label>
            <input
              type="color"
              v-model="store.application.primaryColor"
              class="w-full"
              @change="updateSettings"
            />
          </div>
          <div>
            <label class="text-sm block mb-2">Title Color</label>
            <input
              type="color"
              v-model="store.application.titleColor"
              class="w-full"
              @change="updateSettings"
            />
          </div>
          <div>
            <label class="text-sm block mb-2">Background Color</label>
            <input
              type="color"
              v-model="store.application.backgroundColor"
              class="w-full"
              @change="updateSettings"
            />
          </div>
          <div>
            <label class="text-sm block mb-2">Sidebar Color</label>
            <input
              type="color"
              v-model="store.application.sidebarColor"
              class="w-full"
              @change="updateSettings"
            />
          </div>
        </div>
      </div>

      <!-- Typography Settings -->
      <div class="px-4 mt-6">
        <h2 class="mb-4 font-semibold text-gray-700">Typography</h2>
        <div class="grid grid-cols-2 gap-4">
          <div>
            <label class="text-sm block mb-2">Font Size</label>
            <input
              type="range"
              min="12"
              max="24"
              v-model="store.application.fontSize"
              class="w-full"
              @change="updateSettings"
            />
            <span class="text-xs">{{ store.application.fontSize }}px</span>
          </div>
          <div>
            <label class="text-sm block mb-2">Font Family</label>
            <select
              v-model="store.application.fontFamily"
              class="w-full"
              @change="updateSettings"
            >
              <option value="'Arial', sans-serif">Arial</option>
              <option value="'Times New Roman', serif">Times New Roman</option>
              <option value="'Courier New', monospace">Courier New</option>
              <option value="'Roboto', sans-serif">Roboto</option>
            </select>
          </div>
        </div>
      </div>

      <!-- Internationalization -->
      <div class="px-4 mt-6">
        <h2 class="mb-4 font-semibold text-gray-700">Internationalization</h2>
        <div class="flex items-center mb-4">
          <label for="enableI18n" class="block text-sm font-medium text-gray-700"
            >Enable I18n:</label
          >
          <input
            id="enableI18n"
            type="checkbox"
            v-model="store.application.enableI18n"
            class="ml-2"
          />
        </div>
        <div class="flex items-center mb-4">
          <label for="language" class="block text-sm font-medium text-gray-700">Language:</label>
          <select
            id="language"
            v-model="store.application.language"
            class="mt-1 block w-full p-1 shadow-sm focus:ring-[var(--ring-color)] focus:border-[var(--border-color)]"
          >
            <option value="en">English</option>
            <option value="fr">French</option>
            <!-- Add more languages -->
          </select>
        </div>
        <div class="flex items-center mb-4">
          <label for="isRtl" class="block text-sm font-medium text-gray-700">Is RTL:</label>
          <input id="isRtl" type="checkbox" v-model="store.application.isRtl" class="ml-2" />
        </div>
      </div>

      <!-- Footer Buttons -->
      <div class="flex justify-end gap-3 col-span-2 fixed bottom-0 right-0 p-4">
          <button type="button" class="px-3 py-1.5 bg-gray-200 rounded-sm hover:bg-gray-300 transition"
          @click="isOpen = false">Cancel</button>
          <button  @click="saveConfiguration"
            class="px-3 py-1.5 text-white rounded-sm hover:bg-indigo-600 focus:ring-2 focus:ring-indigo-500"
            :style="{ backgroundColor: store.application.primaryColor  }">Save</button>
        </div>
    </Drawer>
  </div>
</template>

<style scoped>
/* Add any necessary styles here */
</style>