<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue';
import { useRouter } from 'vue-router';
import logo from '@/assets/logo.svg';
import { avatar } from '@/assets/images/images';
import { useSidebar } from '@/stores/useSidebar';
import { useSystemStore, Drawer } from 'c3-library';
import { menuItems } from '@/stores/menuData';
import { sidebarStore } from '@/stores/menuStore';
import { AuthenticationService } from '@/services/authentication-service';

const { isSidebarOpen, toggleSidebar } = useSidebar();
const openDropdown = ref<string | null>(null);
const isHovered = ref<Record<string, boolean>>({});
const userName = ref('Admin');
const userEmail = ref('admin@example.com');
const isLogoutDialogOpen = ref(false);
const isProcessingLogout = ref(false);
const isTooltipListOpen = ref(false);
const isSmallScreen = ref(false);
const router = useRouter();
const store = useSystemStore();

interface Item {
  label: string;
  icon: string;
  link: string;
}

const setActive = (menu: { name: string; link: string }) => {
  sidebarStore.setActiveParent(menu);
};

const handleMouseOver = (itemName: string) => {
  isHovered.value[itemName] = true;
};

const handleMouseLeave = (itemName: string) => {
  isHovered.value[itemName] = false;
};

const handleClick = (parent: string, child: { name: string; link: string }) => {
  setActive(child);
  openDropdown.value = openDropdown.value === parent ? null : parent;
};

const openLogoutDialog = () => (isLogoutDialogOpen.value = true);
const cancelLogout = () => (isLogoutDialogOpen.value = false);

const auth_repo = new AuthenticationService();

const confirmLogout = () => {
  isProcessingLogout.value = true;
  let response = auth_repo.logout();

  if (response) {
    isProcessingLogout.value = false;
    isLogoutDialogOpen.value = false;
    router.push('/authentication/login');
  }
};

const toggleDropdown = (itemName: string) => {
  openDropdown.value = openDropdown.value === itemName ? null : itemName;
};

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

const toggleTooltipList = () => {
  if (!isSidebarOpen.value) {
    isTooltipListOpen.value = !isTooltipListOpen.value;
  }
};

const items: Item[] = [
  { label: 'Profile', icon: 'fa-solid fa-user', link: '/profile' },
  { label: 'Password', icon: 'fa-solid fa-lock', link: '/password' },
  { label: 'Feeds', icon: 'fa-solid fa-bell', link: '/feeds' },
  { label: 'Settings', icon: 'fa-solid fa-gear', link: '/settings' },
  { label: 'Help', icon: 'fa-solid fa-square-question', link: '/help' },
  { label: 'Chat', icon: 'fa-solid fa-comment', link: '/chat' },
];

</script>

<template>
  <aside v-if="!isSmallScreen" :class="{ 'w-64': isSidebarOpen, 'w-16': !isSidebarOpen }"
    class="shadow-md flex flex-col relative transition-all duration-500 ease-[cubic-bezier(0.4, 0.0, 0.2, 1)] z-50"
    :style="{ backgroundColor: store.application.sidebarColor }">
    <div class="flex items-center p-3 space-x-2">
      <img :src="logo" alt="Logo" class="h-8 w-8" />
      <h2 v-if="isSidebarOpen"
        class="hidden md:block text-md font-semibold text-gray-800 whitespace-nowrap overflow-hidden text-ellipsis">
        {{ store.application.name }}
      </h2>
    </div>
    <nav class="flex-1 mt-4">
      <ul class="space-y-2 text-sm">
        <li v-for="item in menuItems" :key="item.name" class="px-2 text-gray-700 relative group">
          <div class="flex items-center px-4 py-2 cursor-pointer rounded-sm"
            :style="{ backgroundColor: isHovered[item.name] ? `${store.application.primaryColor}20` : 'transparent' }"
            @click="toggleDropdown(item.name)" @mouseover="handleMouseOver(item.name)"
            @mouseleave="handleMouseLeave(item.name)">
            <span :class="item.icon" class="h-5 w-5" :style="{ color: `${store.application.primaryColor}` }"></span>
            <span v-if="isSidebarOpen" class="ml-4 font-medium">{{ item.name }}</span>
            <span v-if="isSidebarOpen" :class="openDropdown === item.name
              ? 'fa-solid fa-angle-up'
              : 'fa-solid fa-angle-down'
              " class="ml-auto"></span>
          </div>

          <transition name="dropdown">
            <ul v-if="isSidebarOpen && openDropdown === item.name" class="ml-5">
              <li v-for="child in item.children" :key="child.name" @click="handleClick(item.name, child)"
                class="text-gray-600 rounded-md"
                :style="{ backgroundColor: isHovered[child.name] ? `${store.application.primaryColor}20` : 'transparent' }"
                @mouseover="handleMouseOver(child.name)" @mouseleave="handleMouseLeave(child.name)">
                <router-link :to="child.link"
                  class="px-4 py-2 flex items-center gap-2 hover:text-[${store.application.primaryColor}]">
                  <span :class="child.icon" class="h-4 w-4 flex-shrink-0"
                    :style="{ color: `${store.application.primaryColor}` }"></span>
                  <span class="text-center">{{ child.name }}</span>
                </router-link>
              </li>
            </ul>
          </transition>

          <div v-if="!isSidebarOpen"
            class="tooltip hidden absolute left-full top-1/2 -translate-y-1/2 bg-white border border-gray-300 p-1 z-50 group-hover:block w-54 shadow-md">
            <ul>
              <li v-for="child in item.children" :key="child.name" class="px-4 py-2 text-sm text-gray-700 rounded-sm"
                :style="{ backgroundColor: isHovered[child.name] ? `${store.application.primaryColor}20` : 'transparent' }"
                @mouseover="handleMouseOver(child.name)" @mouseleave="handleMouseLeave(child.name)">
                <router-link :to="child.link"
                  class="flex items-center gap-2 hover:text-[${store.application.primaryColor}]">
                  <span :class="child.icon" class="h-4 w-4 flex-shrink-0"
                    :style="{ color: `${store.application.primaryColor}` }"></span>
                  {{ child.name }}
                </router-link>
              </li>
            </ul>
          </div>
        </li>
      </ul>
    </nav>

    <div
      class="fixed bottom-0 left-0 p-4 bg-white border-t border-gray-200 transition-all duration-500 ease-[cubic-bezier(0.4, 0.0, 0.2, 1)]"
      :class="{ 'w-64': isSidebarOpen, 'w-16': !isSidebarOpen }"
      :style="{ backgroundColor: store.application.sidebarColor }">
      <div class="flex items-center mb-4 transition-all duration-500 ease-[cubic-bezier(0.4, 0.0, 0.2, 1)]">
        <img :src="avatar" alt="User" class="rounded-full mr-2"
          :class="{ 'w-10 h-10': isSidebarOpen, 'w-8 h-8 cursor-pointer hover:shadow-md hover:translate-y-[-2px] transition-transform duration-300': !isSidebarOpen }"
          @click="toggleTooltipList" />
        <div v-if="!isSidebarOpen && isTooltipListOpen"
          class="absolute left-full bottom-16 w-48 p-1 bg-white rounded-md shadow-lg overflow-hidden z-50">
          <ul class="py-1" role="menu" aria-orientation="vertical" aria-labelledby="options-menu">
            <li v-for="(item, index) in items" :key="index"
              class="px-4 py-2 text-sm text-gray-700 transition-all duration-500 ease-[cubic-bezier(0.4, 0.0, 0.2, 1)]"
              :style="{ backgroundColor: isHovered[index] ? `${store.application.primaryColor}20` : 'transparent' }"
              @mouseover="isHovered[index] = true" @mouseout="isHovered[index] = false">
              <router-link :to="item.link" class="flex items-center w-full">
                <span :class="item.icon" class="mr-3 fa-md" :style="{ color: store.application.primaryColor }"></span>
                {{ item.label }}
              </router-link>
            </li>
          </ul>
        </div>
        <div class="flex-1 hidden md:block whitespace-nowrap overflow-hidden text-ellipsis">
          <h3 class="text-sm font-bold">{{ userName }}</h3>
          <p class="text-sm">{{ userEmail }}</p>
        </div>
      </div>

      <div class="flex flex-col space-y-2 mt-4 transition-all duration-500 ease-[cubic-bezier(0.4, 0.0, 0.2, 1)]">
        <div class="flex" :class="{ 'space-x-2': isSidebarOpen, 'space-y-2': !isSidebarOpen }">
          <div class="group relative transition-all duration-500 ease-[cubic-bezier(0.4, 0.0, 0.2, 1)]">
            <button
              class="cursor-pointer w-7 h-7 text-white rounded-full flex items-center justify-center hover:translate-y-[-2px] transition-transform duration-300 shadow-md border"
              :style="{ borderColor: store.application.primaryColor, backgroundColor: `${store.application.primaryColor}CC` }"
              @click="openLogoutDialog">
              <span class="fa-solid fa-right-from-bracket fa-sm"></span>
            </button>
            <span
              class="tooltip text-xs absolute bottom-full left-1/2 transform -translate-x-1/2 whitespace-nowrap bg-gray-700 text-white px-2 py-1 rounded-md">Logout</span>
          </div>
          <div v-show="isSidebarOpen"
            class="group relative transition-all duration-500 ease-[cubic-bezier(0.4, 0.0, 0.2, 1)]">
            <button
              class="cursor-pointer w-7 h-7 text-white rounded-full flex items-center justify-center hover:translate-y-[-2px] transition-transform duration-300 shadow-md border"
              :style="{ borderColor: store.application.primaryColor, backgroundColor: `${store.application.primaryColor}CC` }">
              <span class="fa-solid fa-gear fa-sm"></span>
            </button>
            <span
              class="tooltip text-xs absolute bottom-full left-1/2 transform -translate-x-1/2 whitespace-nowrap bg-gray-700 text-white px-2 py-1 rounded-md">Settings</span>
          </div>
          <div v-show="isSidebarOpen"
            class="group relative transition-all duration-500 ease-[cubic-bezier(0.4, 0.0, 0.2, 1)]">
            <button
              class="cursor-pointer w-7 h-7 text-white rounded-full flex items-center justify-center hover:translate-y-[-2px] transition-transform duration-300 shadow-md border"
              :style="{ borderColor: store.application.primaryColor, backgroundColor: `${store.application.primaryColor}CC` }">
              <span class="fa-solid fa-square-question fa-sm"></span>
            </button>
            <span
              class="tooltip text-xs absolute bottom-full left-1/2 transform -translate-x-1/2 whitespace-nowrap bg-gray-700 text-white px-2 py-1 rounded-md">Help</span>
          </div>
          <div v-show="isSidebarOpen"
            class="group relative transition-all duration-500 ease-[cubic-bezier(0.4, 0.0, 0.2, 1)]">
            <button
              class="cursor-pointer w-7 h-7 text-white rounded-full flex items-center justify-center hover:translate-y-[-2px] transition-transform duration-300 shadow-md border"
              :style="{ borderColor: store.application.primaryColor, backgroundColor: `${store.application.primaryColor}CC` }">
              <span class="fa-solid fa-comment fa-sm"></span>
            </button>
            <span
              class="tooltip text-xs absolute bottom-full left-1/2 transform -translate-x-1/2 whitespace-nowrap bg-gray-700 text-white px-2 py-1 rounded-md">Chat</span>
          </div>
          <div v-show="isSidebarOpen"
            class="group relative transition-all duration-500 ease-[cubic-bezier(0.4, 0.0, 0.2, 1)]">
            <button
              class="cursor-pointer w-7 h-7 text-white rounded-full flex items-center justify-center hover:translate-y-[-2px] transition-transform duration-300 shadow-md border"
              :style="{ borderColor: store.application.primaryColor, backgroundColor: `${store.application.primaryColor}CC` }">
              <span class="fa-solid fa-lock fa-sm"></span>
            </button>
            <span
              class="tooltip text-xs absolute bottom-full left-1/2 transform -translate-x-1/2 whitespace-nowrap bg-gray-700 text-white px-2 py-1 rounded-md">Password</span>
          </div>
          <div v-show="isSidebarOpen"
            class="group relative transition-all duration-500 ease-[cubic-bezier(0.4, 0.0, 0.2, 1)]">
            <button
              class="cursor-pointer w-7 h-7 text-white rounded-full flex items-center justify-center hover:translate-y-[-2px] transition-transform duration-300 shadow-md border"
              :style="{ borderColor: store.application.primaryColor, backgroundColor: `${store.application.primaryColor}CC` }">
              <span class="fa-solid fa-bell fa-sm"></span>
            </button>
            <span
              class="tooltip text-xs absolute bottom-full left-1/2 transform -translate-x-1/2 whitespace-nowrap bg-gray-700 text-white px-2 py-1 rounded-md">Feeds</span>
          </div>
        </div>
      </div>
    </div>

    <div v-if="isLogoutDialogOpen" class="fixed inset-0 bg-gray-900/50 flex items-center justify-center z-50">
      <div class="bg-white rounded-sm shadow-md w-1/4 p-6 text-center">
        <h3 class="text-lg font-bold mb-4">Confirm Logout</h3>
        <p class="text-gray-600 mb-6">Are you sure you want to log out?</p>
        <div class="flex justify-center space-x-4">
          <button class="px-4 py-2 rounded-sm text-white transition cursor-pointer" @click="confirmLogout"
            :style="{ backgroundColor: store.application.primaryColor }" :disabled="isProcessingLogout">
            <span v-if="!isProcessingLogout">Yes, Logout</span>
            <span v-else class="flex items-center">
              <span class="fa-duotone fa-light fa-spinner-scale animate-spin mr-2"></span>Logging Out...
            </span>
          </button>
          <button class="px-4 py-2 cursor-pointer rounded-sm text-gray-800 bg-gray-200 hover:bg-gray-300 transition"
            @click="cancelLogout" :disabled="isProcessingLogout">
            Cancel
          </button>
        </div>
      </div>
    </div>
  </aside>

  <Drawer v-else :is-open="isSidebarOpen" title=" " position="left" size="w-full" @click="toggleSidebar">
    <template #header>
      <div class="flex items-center justify-between w-full px-4 py-3">
        <div class="flex items-center w-full space-x-2 mr-10">
          <img :src="logo" alt="Logo" class="h-8 w-8" />
          <h2 v-if="isSidebarOpen" class="text-md font-semibold text-gray-800">
            Ultimate ERP Solution
          </h2>
        </div>
        <button @click="toggleSidebar" class="text-gray-800 hover:text-gray-500 transition cursor-pointer duration-200">
          <span class="fa-solid fa-xmark"></span>
        </button>
      </div>
    </template>

    <div class="p-4">
      <div class="flex items-center mb-4">
        <img :src="avatar" alt="User" class="rounded-full h-12 w-12 mr-2" />
        <div>
          <h3 class="text-sm font-bold">{{ userName }}</h3>
          <p class="text-sm text-gray-700">{{ userEmail }}</p>
        </div>
      </div>
      <nav class="flex-1">
        <ul class="space-y-2 text-sm">
          <li v-for="item in menuItems" :key="item.name" class="px-2 text-gray-700 relative group">
            <div class="flex items-center px-4 py-2 cursor-pointer rounded-sm"
              :style="{ backgroundColor: isHovered[item.name] ? `${store.application.primaryColor}20` : 'transparent' }"
              @click="toggleDropdown(item.name)" @mouseover="handleMouseOver(item.name)"
              @mouseleave="handleMouseLeave(item.name)">
              <span :class="item.icon" class="h-5 w-5" :style="{ color: `${store.application.primaryColor}` }"></span>
              <span class="ml-4 font-medium">{{ item.name }}</span>
              <span :class="openDropdown === item.name
                ? 'fa-solid fa-angle-up'
                : 'fa-solid fa-angle-down'
                " class="ml-auto"></span>
            </div>
            <transition name="dropdown">
              <ul v-if="openDropdown === item.name" class="ml-8">
                <li v-for="child in item.children" :key="child.name" class="px-4 py-2 text-sm text-gray-700 rounded-sm"
                  :style="{ backgroundColor: isHovered[child.name] ? `${store.application.primaryColor}20` : 'transparent' }"
                  @mouseover="handleMouseOver(child.name)" @mouseleave="handleMouseLeave(child.name)">
                  <router-link :to="child.link"
                    class="flex items-center gap-2 hover:text-[${store.application.primaryColor}]">
                    <span :class="child.icon" class="h-4 w-4 flex-shrink-0"
                      :style="{ color: `${store.application.primaryColor}` }"></span>
                    {{ child.name }}
                  </router-link>
                </li>
              </ul>
            </transition>
          </li>
        </ul>
      </nav>
      <div class="fixed bottom-0 w-full left-0 p-4 bg-white border-t border-gray-200"
        :class="{ 'w-64': isSidebarOpen, 'w-16': !isSidebarOpen }">
        <div class="flex flex-col space-y-2">
          <button
            class="w-full cursor-pointer text-white py-2 rounded-sm flex items-center justify-center transition relative"
            :style="{ backgroundColor: store.application.primaryColor }" @click="openLogoutDialog">
            <span class="fa-solid fa-right-from-bracket"></span>
            <span v-if="isSidebarOpen" class="ml-2">Logout</span>
          </button>
        </div>
      </div>

      <div v-if="isLogoutDialogOpen" class="fixed inset-0 bg-gray-900/50 flex items-center justify-center z-50">
        <div class="bg-white rounded-sm shadow-md w-1/4 p-6 text-center">
          <h3 class="text-lg font-bold mb-4">Confirm Logout</h3>
          <p class="text-gray-600 mb-6">Are you sure you want to log out?</p>
          <div class="flex justify-center space-x-4">
            <button class="px-4 py-2 cursor-pointer rounded-sm text-white transition" @click="confirmLogout"
              :style="{ backgroundColor: store.application.primaryColor }" :disabled="isProcessingLogout">
              <span v-if="!isProcessingLogout">Yes, Logout</span>
              <span v-else class="flex items-center">
                <span class="fa-duotone fa-light fa-spinner-scale animate-spin mr-2"></span>Logging Out...
              </span>
            </button>
            <button class="px-4 py-2 cursor-pointer rounded-sm text-gray-800 bg-gray-200 hover:bg-gray-300 transition"
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

.dropdown-enter-active,
.dropdown-leave-active {
  transition: all 0.3s ease-in-out;
}

.dropdown-enter-from,
.dropdown-leave-to {
  opacity: 0;
  transform: translateY(-10px);
}

@media (max-width: 768px) {
  aside {
    display: none;
  }
}
</style>
