<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { avatar } from '@/assets/images/images'
import { useSidebar } from '../../../stores/useSidebar'
import { selectColor } from '@/stores/colorPalette';

const { isSidebarOpen } = useSidebar()
const openDropdown = ref<string | null>(null)
const userName = ref('Admin')
const isLogoutDialogOpen = ref(false)
const isProcessingLogout = ref(false)
const router = useRouter()

const menuItems = [
  {
    name: 'Home',
    icon: 'icon-[material-symbols--dashboard-outline]',
    children: [
      { name: 'Dashboard', link: '/dashboard' }
    ],
  },
  {
    name: 'HRM',
    icon: 'icon-[mdi--briefcase-outline]',
    children: [
      { name: 'Employee Management', link: '#' },
      { name: 'Payroll', link: '#' },
      { name: 'Attendance', link: '#' },
    ],
  },
  {
    name: 'User Management',
    icon: 'icon-[mdi--account-key-outline]',
    children: [
      { name: 'Users', link: '/users' },
      { name: 'User Roles', link: '#' },
      { name: 'Routes', link: '#' },
      { name: 'Roles Routes Map', link: '#' },
      { name: 'Users Roles Map', link: '#' },
      { name: 'Queries', link: '#' },
    ],
  },
  {
    name: 'Production',
    icon: 'icon-[material-symbols--factory-outline]',
    children: [
      { name: 'Inventory', link: '#' },
      { name: 'Manufacturing', link: '#' },
      { name: 'Quality Control', link: '#' },
    ],
  },
  {
    name: 'Retail',
    icon: 'icon-[mdi--cart-outline]',
    children: [
      { name: 'Point of Sale', link: '#' },
      { name: 'Sales Reports', link: '#' },
      { name: 'Customer Feedback', link: '#' },
    ],
  }
]

const openLogoutDialog = () => {
  isLogoutDialogOpen.value = true
}

const cancelLogout = () => {
  isLogoutDialogOpen.value = false
}

const confirmLogout = () => {
  isProcessingLogout.value = true
  setTimeout(() => {
    isProcessingLogout.value = false
    isLogoutDialogOpen.value = false
    router.push('/authentication/login')
  }, 1000)
}

const toggleDropdown = (itemName: string) => {
  openDropdown.value = openDropdown.value === itemName ? null : itemName
}
</script>

<template>
  <aside :class="{
    'w-64': isSidebarOpen,
    'w-16': !isSidebarOpen,
  }" class="bg-white shadow-md flex flex-col relative transition-all duration-300 min-h-screen">
    <div class="relative bg-white border-b-2 border-gray-200 p-4">
      <div class="flex flex-col items-center">
        <div class="relative">
          <img :src="avatar" alt="User"
            class="rounded-full border-2 border-sky-400 transform hover:scale-105 transition duration-300"
            :class="{ 'h-16 w-16': isSidebarOpen, 'h-8 w-8': !isSidebarOpen }" />
          <div class="absolute bottom-0 right-1 bg-green-500 rounded-full"
            :class="{ 'h-4 w-4': isSidebarOpen, 'h-2 w-2': !isSidebarOpen }"></div>
        </div>
        <div v-if="isSidebarOpen" class="text-gray-800 text-center mt-2">
          <h3 class="text-lg font-bold">{{ userName }}</h3>
          <p class="text-sm text-gray-700 font-light">Administrator</p>
        </div>
      </div>
    </div>

    <nav class="flex-1 mt-4">
      <ul class="space-y-2 text-sm">
        <li v-for="item in menuItems" :key="item.name" class="px-2 text-gray-700 relative group">
          <div class="flex items-center px-4 py-2 cursor-pointer hover:bg-gray-100 rounded-sm"
            @click="toggleDropdown(item.name)">
            <span :class="item.icon" class="h-5 w-5"></span>
            <span v-if="isSidebarOpen" class="ml-4 font-medium">{{ item.name }}</span>
            <span v-if="isSidebarOpen" :class="openDropdown === item.name ? 'icon-[mdi--chevron-up]' : 'icon-[mdi--chevron-down]'
              " class="h-5 w-5 ml-auto"></span>
          </div>

          <ul v-if="isSidebarOpen && openDropdown === item.name" class="ml-8 border-l-2 border-gray-300">
            <li v-for="child in item.children" :key="child.name"
              class="px-4 py-2 text-gray-600 hover:bg-gray-100 rounded-md">
              <router-link :to="child.link" class="block">{{ child.name }}</router-link>
            </li>
          </ul>

          <div v-if="!isSidebarOpen"
            class="tooltip hidden absolute left-full top-0 transform -translate-y-1/2 bg-white border border-gray-300 p-1 z-50 group-hover:block w-48">
            <ul>
              <li v-for="child in item.children" :key="child.name"
                class="px-4 py-2 text-sm text-gray-600 hover:bg-gray-100 hover:text-gray-700 rounded-sm">
                <router-link :to="child.link" class="block">{{ child.name }}</router-link>
              </li>
            </ul>
          </div>
        </li>
      </ul>
    </nav>

    <div class="fixed bottom-0 left-0 p-4 bg-white border-t border-gray-200"
      :class="{ 'w-64': isSidebarOpen, 'w-16': !isSidebarOpen }">
      <button class="w-full text-white py-2 rounded-sm flex items-center justify-center transition relative"
        :style="{ backgroundColor: selectColor() }" @click="openLogoutDialog">
        <span class="icon-[mdi--logout] h-5 w-5"></span>
        <span v-if="isSidebarOpen" class="ml-2">Logout</span>
      </button>
    </div>

    <div v-if="isLogoutDialogOpen" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div class="bg-white rounded-md shadow-md w-1/4 p-6 text-center">
        <h3 class="text-lg font-bold mb-4">Confirm Logout</h3>
        <p class="text-gray-600 mb-6">Are you sure you want to log out?</p>
        <div class="flex justify-center space-x-4">
          <button class="px-4 py-2 rounded-sm text-white transition" @click="confirmLogout"
          :style="{ backgroundColor: selectColor() }" :disabled="isProcessingLogout">
            <span v-if="!isProcessingLogout">Yes, Logout</span>
            <span v-else class="flex items-center">
              <span class="icon-[uiw--loading] animate-spin mr-2"></span> Logging Out...
            </span>
          </button>
          <button class="px-4 py-2 rounded-sm text-gray-800 bg-gray-200 hover:bg-gray-300 transition"
            @click="cancelLogout" :disabled="isProcessingLogout">
            Cancel
          </button>
        </div>
      </div>
    </div>
  </aside>
</template>

<style lang="css" scoped>
.tooltip {
  display: none;
  transform: scale(0.9);
  opacity: 0;
  transition: all 0.3s ease-in-out;
}

.group:hover .tooltip {
  display: block;
  transform: scale(1);
  opacity: 1;
}
</style>
