<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue';
import { useRouter } from 'vue-router';
import logo from '@/assets/logo.svg';
import { avatar } from '@/assets/images/images';
import { useSidebar } from '@/stores/useSidebar';
import { useSystemStore, Drawer } from 'c3-library';
import { menuItems } from '@/stores/menuData'
import { sidebarStore } from '@/stores/menuStore';

const { isSidebarOpen, toggleSidebar } = useSidebar();
const openDropdown = ref<string | null>(null);
const userName = ref('Admin');
const isLogoutDialogOpen = ref(false);
const isProcessingLogout = ref(false);
const router = useRouter();
const store = useSystemStore();

const setActive = (menu: { name: string; link: string }) => {
  sidebarStore.setActiveParent(menu);
};

const handleClick = (parent: string, child: { name: string; link: string }) => {
  setActive(child);
  openDropdown.value = openDropdown.value === parent ? null : parent;
};

const openLogoutDialog = () => (isLogoutDialogOpen.value = true);
const cancelLogout = () => (isLogoutDialogOpen.value = false);

const confirmLogout = () => {
  isProcessingLogout.value = true;
  setTimeout(() => {
    isProcessingLogout.value = false;
    isLogoutDialogOpen.value = false;
    router.push('/authentication/login');
  }, 1000);
};

const toggleDropdown = (itemName: string) => {
  openDropdown.value = openDropdown.value === itemName ? null : itemName;
};

const isSmallScreen = ref(false);

const checkScreenSize = () => {
  isSmallScreen.value = window.innerWidth <= 768;
};

onMounted(() => {
  checkScreenSize();
  window.addEventListener('resize', checkScreenSize);
});

onUnmounted(() => {
  window.removeEventListener('resize', checkScreenSize);
});

</script>

<template>
  <aside v-if="!isSmallScreen" :class="{ 'w-64': isSidebarOpen, 'w-16': !isSidebarOpen }"
    class="shadow-md flex flex-col relative transition-all duration-300"
    :style="{ backgroundColor: store.application.sidebarColor }">
    <div class="flex items-center border-b border-gray-200 p-3 space-x-2">
      <img :src="logo" alt="Logo" class="h-8 w-8" />
      <h2 v-if="isSidebarOpen"
        class="hidden md:block text-md font-semibold text-gray-800 whitespace-nowrap overflow-hidden text-ellipsis">
        {{ store.application.name }}
      </h2>
    </div>
    <nav class="flex-1 mt-4">
      <ul class="space-y-2 text-sm">
        <li v-for="item in menuItems" :key="item.name" class="px-2 text-gray-700 relative group">
          <div class="flex items-center px-4 py-2 cursor-pointer hover:bg-gray-100 rounded-sm"
            @click="toggleDropdown(item.name)">
            <span :class="item.icon" class="h-5 w-5"></span>
            <span v-if="isSidebarOpen" class="ml-4 font-medium">{{ item.name }}</span>
            <span v-if="isSidebarOpen" :class="openDropdown === item.name
              ? 'fa-solid fa-angle-up'
              : 'fa-solid fa-angle-down'
              " class="ml-auto"></span>
          </div>

          <ul v-if="isSidebarOpen && openDropdown === item.name" class="ml-8 border-l-2 border-gray-300">
            <li v-for="child in item.children" :key="child.name" @click="handleClick(item.name, child)"
              class="px-4 py-2 text-gray-600 hover:bg-gray-100 rounded-md">
              <router-link :to="child.link" class="block">{{ child.name }}</router-link>
            </li>
          </ul>

          <div v-if="!isSidebarOpen"
            class="tooltip hidden absolute left-full top-1/2 -translate-y-1/2 bg-white border border-gray-300 p-1 z-50 group-hover:block w-48 shadow-md">
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
        :style="{ backgroundColor: store.application.primaryColor }" @click="openLogoutDialog">
        <span class="fa-solid fa-right-from-bracket"></span>
        <span v-if="isSidebarOpen" class="ml-2">Logout</span>
      </button>
    </div>

    <div v-if="isLogoutDialogOpen" class="fixed inset-0 bg-gray-900/50 flex items-center justify-center z-50">
      <div class="bg-white rounded-sm shadow-md w-1/4 p-6 text-center">
        <h3 class="text-lg font-bold mb-4">Confirm Logout</h3>
        <p class="text-gray-600 mb-6">Are you sure you want to log out?</p>
        <div class="flex justify-center space-x-4">
          <button class="px-4 py-2 rounded-sm text-white transition" @click="confirmLogout"
            :style="{ backgroundColor: store.application.primaryColor }" :disabled="isProcessingLogout">
            <span v-if="!isProcessingLogout">Yes, Logout</span>
            <span v-else class="flex items-center">
              <span class="fa-duotone fa-light fa-spinner-scale animate-spin mr-2"></span>Logging Out...
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

  <Drawer v-else :is-open="isSidebarOpen" title=" " position="left" size="w-full" @click="toggleSidebar">
    <template #header>
      <div class="flex items-center justify-between w-full px-4 py-3"
        :style="{ backgroundColor: store.application.sidebarColor, color: 'white' }">


        <div class="flex items-center w-full space-x-2 mr-10">
          <img :src="logo" alt="Logo" class="h-8 w-8" />
          <h2 v-if="isSidebarOpen" class="text-md font-semibold text-gray-50">
            Ultimate ERP Solution
          </h2>
        </div>

        <button @click="toggleSidebar" class="text-white hover:text-gray-300 transition duration-200">
          <span class="fa-solid fa-xmark"></span>
        </button>
      </div>
    </template>

    <div class="p-4">
      <div class="flex items-center mb-4">
        <img :src="avatar" alt="User" class="rounded-full h-12 w-12 mr-2" />
        <div>
          <h3 class="text-lg font-bold">{{ userName }}</h3>
          <p class="text-sm text-gray-700">Administrator</p>
        </div>
      </div>
      <nav class="flex-1">
        <ul class="space-y-2 text-sm">
          <li v-for="item in menuItems" :key="item.name" class="px-2 text-gray-700 relative group">
            <div class="flex items-center px-4 py-2 cursor-pointer hover:bg-gray-100 rounded-sm"
              @click="toggleDropdown(item.name)">
              <span :class="item.icon" class="h-5 w-5"></span>
              <span class="ml-4 font-medium">{{ item.name }}</span>
              <span :class="openDropdown === item.name
                ? 'fa-solid fa-angle-up'
                : 'fa-solid fa-angle-down'
                " class="ml-auto"></span>
            </div>
            <ul v-if="openDropdown === item.name" class="ml-8 border-l-2 border-gray-300">
              <li v-for="child in item.children" :key="child.name" @click="handleClick(item.name, child)"
                class="px-4 py-2 text-gray-600 hover:bg-gray-100 rounded-md">
                <router-link :to="child.link" class="block">{{ child.name }}</router-link>
              </li>
            </ul>
          </li>
        </ul>
      </nav>
      <div class="fixed bottom-0 w-full left-0 p-4 bg-white border-t border-gray-200"
        :class="{ 'w-64': isSidebarOpen, 'w-16': !isSidebarOpen }">
        <button class="w-full text-white py-2 rounded-sm flex items-center justify-center transition relative"
          :style="{ backgroundColor: store.application.primaryColor }" @click="openLogoutDialog">
          <span class="fa-solid fa-right-from-bracket"></span>
          <span v-if="isSidebarOpen" class="ml-2">Logout</span>
        </button>
      </div>

      <div v-if="isLogoutDialogOpen" class="fixed inset-0 bg-gray-900/50 flex items-center justify-center z-50">
        <div class="bg-white rounded-sm shadow-md w-1/4 p-6 text-center">
          <h3 class="text-lg font-bold mb-4">Confirm Logout</h3>
          <p class="text-gray-600 mb-6">Are you sure you want to log out?</p>
          <div class="flex justify-center space-x-4">
            <button class="px-4 py-2 rounded-sm text-white transition" @click="confirmLogout"
              :style="{ backgroundColor: store.application.primaryColor }" :disabled="isProcessingLogout">
              <span v-if="!isProcessingLogout">Yes, Logout</span>
              <span v-else class="flex items-center">
                <span class="fa-duotone fa-light fa-spinner-scale animate-spin mr-2"></span>Logging Out...
              </span>
            </button>
            <button class="px-4 py-2 rounded-sm text-gray-800 bg-gray-200 hover:bg-gray-300 transition"
              @click="cancelLogout" :disabled="isProcessingLogout">
              Cancel
            </button>
          </div>
        </div>
      </div>
    </div>
  </Drawer>

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

@media (max-width: 768px) {
  aside {
    display: none;
  }
}
</style>
