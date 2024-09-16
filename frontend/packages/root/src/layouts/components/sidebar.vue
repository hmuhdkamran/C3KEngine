<script setup lang="ts">
import logo from "@/assets/images/vue.svg";
import { ref } from 'vue';

defineProps<{ collapsed: boolean }>();
defineEmits(['toggleSidebar']);
</script>

<template>
  <div :class="[collapsed ? 'w-16' : 'w-64']" class="fixed inset-y-0 left-0 bg-white shadow-lg z-50 transition-all duration-300 ease-in-out">
    <div class="flex flex-col h-full">
      <div class="flex justify-between p-4">
        <button @click="$emit('toggleSidebar')" class="text-gray-600 hover:text-gray-800 focus:outline-none">
          <span class="icon-[fluent--chevron-left-20-filled] h-5 w-5" :class="collapsed ? 'rotate-180' : ''"></span>
        </button>
      </div>

      <a href="/" class="flex items-center space-x-2 p-4">
        <img :src="logo" alt="Logo" class="h-12 w-auto object-contain" />
        <span v-if="!collapsed" class="text-xl font-semibold text-gray-800">C3K Engine</span>
      </a>

      <hr />

      <div class="px-3 py-4 overflow-y-auto">
        <ul class="space-y-2">
          <li>
            <a href="dashboard" class="flex items-center rounded-lg px-4 py-2 text-gray-800 hover:bg-violet-100">
              <span class="icon-[fluent-mdl2--recruitment-management] bg-violet-600 mx-3 w-5 h-5"></span>
              <span v-if="!collapsed">HRMS</span>
            </a>
          </li>
          <li>
            <a href="dashboard" class="flex items-center rounded-lg px-4 py-2 text-gray-800 hover:bg-violet-100">
              <span class="icon-[vaadin--shop] bg-violet-600 mx-3 w-5 h-5"></span>
              <span v-if="!collapsed">Retail</span>
            </a>
          </li>
          <li>
            <a href="dashboard" class="flex items-center rounded-lg px-4 py-2 text-gray-800 hover:bg-violet-100">
              <span class="icon-[mdi--office-building-settings-outline] bg-violet-600 mx-3 w-5 h-5"></span>
              <span v-if="!collapsed">Production</span>
            </a>
          </li>
        </ul>
      </div>
      <div class="mt-auto px-3 py-3">
        <hr />
        <ul class="space-y-2">
          <li>
            <a href="/profile" class="flex items-center rounded-lg px-4 py-2 text-gray-800 hover:bg-violet-100">
              <span class="icon-[ic--baseline-person] bg-violet-600 mx-3 w-5 h-5"></span>
              <span v-if="!collapsed">Profile</span>
            </a>
          </li>
          <li>
            <a href="/settings" class="flex items-center rounded-lg px-4 py-2 text-gray-800 hover:bg-violet-100">
              <span class="icon-[ic--baseline-settings] bg-violet-600 mx-3 w-5 h-5"></span>
              <span v-if="!collapsed">Settings</span>
            </a>
          </li>
          <li>
            <a href="/logout" class="flex items-center rounded-lg px-4 py-2 text-gray-800 hover:bg-violet-100">
              <span class="icon-[ic--baseline-logout] bg-violet-600 mx-3 w-5 h-5"></span>
              <span v-if="!collapsed">Logout</span>
            </a>
          </li>
        </ul>
      </div>
    </div>
  </div>
</template>

