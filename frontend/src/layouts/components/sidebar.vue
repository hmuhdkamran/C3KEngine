<script setup lang="ts">
import logo from "@/assets/images/vue.svg";

defineProps<{ showModuleDropdown: boolean }>();
defineEmits(['toggleModule']);
</script>

<template>
  <transition name="slide">
    <div v-if="showModuleDropdown" class="fixed inset-y-0 left-0 w-64 bg-white shadow-lg z-50">
      <div class="flex flex-col h-full style-basic">
        <div class="flex justify-end p-4">
          <button @click="$emit('toggleModule')" class="text-gray-600 hover:text-gray-800 focus:outline-none">
            <span class="icon-[fluent--dismiss-20-filled] h-5 w-5"></span>
          </button>
        </div>
        <a href="/" class="text-xl font-semibold text-gray-800 flex items-center space-x-2 ml-4">
          <img :src="logo" alt="C3K Engine Logo" class="h-12 w-auto object-contain" />
          <span>C3K Engine</span>
        </a>
        <hr />
        <div class="px-3 py-4 overflow-y-auto rounded">
          <ul class="py-6 space-y-2">
            <li><a href="dashboard" class="block rounded-lg px-4 py-2 text-gray-800 hover:bg-violet-100">
                <span class="icon-[fluent-mdl2--recruitment-management] bg-violet-600 mx-3 w-5 h-5"></span>
                HRMS</a></li>
            <li><a href="dashboard" class="block rounded-lg px-4 py-2 text-gray-800 hover:bg-violet-100">
                <span class="icon-[vaadin--shop] bg-violet-600 mx-3 w-5 h-5"></span>
                Retail</a></li>
            <li><a href="dashboard" class="block rounded-lg px-4 py-2 text-gray-800 hover:bg-violet-100">
                <span class="icon-[mdi--office-building-settings-outline] bg-violet-600 mx-3 w-5 h-5"></span>
                Production</a></li>
          </ul>
        </div>
        <div class="mt-auto px-3 py-3">
          <hr />
          <ul class="space-y-2">
            <li>
              <a href="/profile" class="flex items-center rounded-lg px-4 py-2 text-gray-800 hover:bg-violet-100">
                <span class="icon-[ic--baseline-person] bg-violet-600 mx-3 w-5 h-5"></span>
                Profile
              </a>
            </li>
            <li>
              <a href="/settings" class="flex items-center rounded-lg px-4 py-2 text-gray-800 hover:bg-violet-100">
                <span class="icon-[ic--baseline-settings] bg-violet-600 mx-3 w-5 h-5"></span>
                Settings
              </a>
            </li>
            <li>
              <a href="/logout" class="flex items-center rounded-lg px-4 py-2 text-gray-800 hover:bg-violet-100">
                <span class="icon-[ic--baseline-logout] bg-violet-600 mx-3 w-5 h-5"></span>
                Logout
              </a>
            </li>
          </ul>
        </div>
      </div>
    </div>
  </transition>
  <transition name="fade">
      <div v-if="showModuleDropdown" class="fixed inset-0 bg-black opacity-50 z-40" @click="$emit('toggleModule')"></div>
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

</style>
