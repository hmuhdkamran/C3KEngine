<script setup lang="ts">
import { useSystemStore, Drawer } from 'c3-library'
import { ref, computed } from 'vue'

const store = useSystemStore()

const isOpen = ref(false)

const logoUrl = computed({
  get() {
    return store.application.logo?.props?.src || ''
  },
  set(value) {
    store.application.logo = h('img', { src: value, class: 'inline-block w-10' })
  },
})

// Functions to manage social media links
function addSocial() {
  store.application.socialMedia.push({
    name: '',
    link: '',
    icon: '',
  })
}

function removeSocial(index: number) {
  store.application.socialMedia.splice(index, 1)
}

// Save configuration function
function saveConfiguration() {
  store.updateApplication(store.application)
}

// Update colors on change
function updateColors() {
  store.updateApplication(store.application)
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
    <Drawer :is-open="isOpen" title="Theme Setting" position="right" size="w-1/4">
      <template #header>
        <div class="flex items-center justify-between w-full px-4 py-3">
          <h3 class="text-lg font-semibold text-gray-700">Select Preset</h3>
          <button
            @click="isOpen = false"
            class="text-gray-600 hover:text-gray-500 cursor-pointer transition duration-200"
          >
            <span class="fa-solid fa-xmark"></span>
          </button>
        </div>
      </template>

      <!-- Color Preset Grid -->
      <div>
        <h2 class="mb-4 font-semibold text-gray-700">Branding</h2>
        <div class="flex items-center mb-4">
          <label for="appName" class="block text-sm font-medium text-gray-700">Name:</label>
          <input
            id="appName"
            type="text"
            v-model="store.application.name"
            placeholder="Application Name"
            class="ml-4 p-2 border rounded-lg w-full focus:ring-indigo-500 focus:border-indigo-500"
          />
        </div>
        <div class="flex items-center mb-4">
          <label for="appLogo" class="block text-sm font-medium text-gray-700">Logo URL:</label>
          <input
            id="appLogo"
            type="text"
            v-model="logoUrl"
            placeholder="https://example.com/logo.png"
            class="ml-4 p-2 border rounded-lg w-full focus:ring-indigo-500 focus:border-indigo-500"
            @input="updateColors"
          />
          <img :src="logoUrl" />
        </div>
      </div>

      <!-- Internationalization -->
      <div>
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
            class="ml-4 p-2 border rounded-lg w-full focus:ring-indigo-500 focus:border-indigo-500"
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

      <!-- Social Media Links -->
      <div>
        <h2 class="mb-4 font-semibold text-gray-700">Social Media Links</h2>
        <div
          v-for="(item, index) in store.application.socialMedia"
          :key="index"
          class="flex items-center mb-4 pb-4 border-b border-[#dee2e6]"
        >
          <input
            type="text"
            v-model="item.name"
            placeholder="Facebook"
            class="p-2 border rounded-lg w-1/4 focus:ring-indigo-500 focus:border-indigo-500"
            @input="updateColors"
          />
          <input
            type="text"
            v-model="item.link"
            placeholder="https://facebook.com/user"
            class="p-2 border rounded-lg w-1/2 ml-4 focus:ring-indigo-500 focus:border-indigo-500"
            @input="updateColors"
          />
          <input
            type="text"
            v-model="item.icon"
            placeholder="fab fa-facebook"
            class="p-2 border rounded-lg w-1/4 ml-4 focus:ring-indigo-500 focus:border-indigo-500"
            @input="updateColors"
          />
          <button
            @click="removeSocial(index)"
            class="ml-4 text-red-500 hover:text-red-700"
            title="Remove Entry"
          >
            <span class="fa-solid fa-trash"></span>
          </button>
        </div>
        <button
          @click="addSocial"
          class="px-4 py-2 mt-4 text-sm text-blue-700 bg-blue-100 rounded-lg hover:bg-blue-200"
        >
          Add Social Media
        </button>
      </div>

      <!-- Color Presets -->
      <div>
        <h2 class="mb-4 font-semibold text-gray-700">Color Presets</h2>
        <div class="grid grid-cols-2 gap-4">
          <div>
            <label class="text-sm">Primary Color</label>
            <input
              type="color"
              v-model="store.application.primaryColor"
              class="w-full mt-2"
              @change="updateColors"
            />
          </div>
          <div>
            <label class="text-sm">Title Color</label>
            <input
              type="color"
              v-model="store.application.titleColor"
              class="w-full mt-2"
              @change="updateColors"
            />
          </div>
          <div>
            <label class="text-sm">Background Color</label>
            <input
              type="color"
              v-model="store.application.backgroundColor"
              class="w-full mt-2"
              @change="updateColors"
            />
          </div>
          <div>
            <label class="text-sm">Sidebar Color</label>
            <input
              type="color"
              v-model="store.application.sidebarColor"
              class="w-full mt-2"
              @change="updateColors"
            />
          </div>
        </div>
      </div>

      <!-- Footer Buttons -->
      <div class="flex justify-end mt-4 mb-4">
        <button
          @click="isOpen = false"
          class="px-4 py-2 mr-4 text-gray-700 border rounded-lg hover:bg-gray-100 focus:outline-none"
        >
          Cancel
        </button>
        <button
          @click="saveConfiguration"
          class="px-4 py-2 text-white bg-indigo-600 rounded-lg hover:bg-indigo-700 focus:outline-none"
        >
          Save
        </button>
      </div>
    </Drawer>
  </div>
</template>

<style scoped>
/* Add any necessary styles here */
</style>
