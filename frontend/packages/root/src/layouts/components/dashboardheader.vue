<script setup lang="ts">
import { ref, computed } from 'vue';
import Sidebar from './sidebar.vue';
import Notifications from './notification.vue';
import ProfileDropdown from './profile.vue';
import logo from "@/assets/images/vue.svg";

const isOpen = ref(false);
const showModuleDropdown = ref(false);
const showProfileDropdown = ref(false);

const props = defineProps<{ selectedCardTitle: string }>();

function toggleModule() {
  showModuleDropdown.value = !showModuleDropdown.value;
  isOpen.value = false;
  showProfileDropdown.value = false;
}

function toggleDropdown() {
  isOpen.value = !isOpen.value;
  showProfileDropdown.value = false;
  showModuleDropdown.value = false;
}

function toggleProfileDropdown() {
  showProfileDropdown.value = !showProfileDropdown.value;
  isOpen.value = false;
  showModuleDropdown.value = false;
}
const showDropdown = ref(false);

function Dropdown() {
  showDropdown.value = !showDropdown.value;
}

const searchQuery = ref("");

const services = ref([
  { name: 'Authentication Service', icon: 'icon-[mdi--account-check]', description: 'Manage user authentication' },
  { name: 'Business Setup Service', icon: 'icon-[mdi--office-building]', description: 'Setup business essentials' },
  { name: 'HRMS Service', icon: 'icon-[clarity--employee-group-solid]', description: 'Human resource management' },
  { name: 'Retail Service', icon: 'icon-[mdi--store]', description: 'Manage retail operations' },
  { name: 'Point of Sale Service', icon: 'icon-[mdi--cash-register]', description: 'Point of Sale functionality' },
  { name: 'Supply Chain Service', icon: 'icon-[mdi--truck-delivery]', description: 'Manage supply chain' },
  { name: 'Finance Service', icon: 'icon-[mdi--currency-usd]', description: 'Financial services' }
]);

const filteredServices = computed(() =>
  services.value.filter(service =>
    service.name.toLowerCase().includes(searchQuery.value.toLowerCase())
  )
);

</script>

<template>
  <div class="fixed top-0 left-0 w-full px-4 sm:px-6 lg:px-8 flex h-12 items-center bg-violet-500 justify-between">
    <div class="md:flex md:items-center">
      <div class="relative">
        <button @click="Dropdown" class="flex items-center space-x-2 text-white hover:text-gray-300 focus:outline-none">
          <span class="icon-[fluent--navigation-unread-20-filled] text-white h-8 w-8"></span>
          <h1 class="text-white text-2xl font-bold py-4 px-4">{{ props.selectedCardTitle }}</h1>
        </button>
        <transition name="fade">
          <div v-if="showDropdown" class="absolute left-0 mt-4 w-[90vw] sm:w-[600px] bg-white rounded-lg shadow-lg z-50 p-6 grid grid-cols-2 gap-6 transform 
          origin-top scale-95 transition-transform duration-300">
            <div class="col-span-full mb-4 relative">
              <input v-model="searchQuery" type="text" placeholder="Search Modules..."
                class="c3k-input pl-5" />
              <span
                class="icon-[ic--baseline-search] -pl-5 h-6 w-6 text-gray-400 absolute top-1/2 transform -translate-y-1/2 left-3"></span>
            </div>

            <div v-if="filteredServices.length" v-for="(service, index) in filteredServices" :key="index"
              class="flex items-center cursor-pointer bg-gray-50 border border-gray-200 rounded-sm border-l-4 p-4 hover:bg-gray-100 hover:shadow-md transition-all duration-300 relative">

              <span :class="service.icon + ' text-violet-600 h-8 w-8 mr-4'"></span>
              <div>
                <h4 class="text-md font-semibold text-gray-700">{{ service.name }}</h4>
                <p class="text-sm text-gray-500">{{ service.description }}</p>
              </div>

              <span v-if="service.name === 'HRMS Service'"
                class="absolute top-2 right-2 text-xs font-semibold text-white bg-red-500 rounded-full px-2 py-1">New</span>
            </div>

            <div v-if="!filteredServices.length" class="col-span-full text-center text-gray-500">
              <p>No modules found.</p>
            </div>
          </div>
        </transition>
      </div>
      <div class="flex space-x-8 items-center">
        <nav class="flex space-x-6">
          <a href="#" class="text-white font-medium hover:text-gray-200 transition duration-200">CRM Dashboard</a>
          <a href="#" class="text-white font-medium hover:text-gray-200 transition duration-200">Sales Overview</a>
          <a href="#" class="text-white font-medium hover:text-gray-200 transition duration-200">Customer Engagement</a>
          <a href="#" class="text-white font-medium hover:text-gray-200 transition duration-200">Pipeline Management</a>
        </nav>
      </div>
      <!-- <Sidebar @toggleModule="toggleModule" :showModuleDropdown="showModuleDropdown" /> -->
    </div>
    <div class="flex items-center">
      <button>
        <div class="flex items-center p-2 text-gray-600 hover:text-gray-800 focus:outline-none">
          <span class="icon-[ic--baseline-chat] text-white h-6 w-6"></span>
        </div>
      </button>
      <Notifications @toggleDropdown="toggleDropdown" :isOpen="isOpen" />
      <button>
        <div class="flex items-center p-2 text-gray-600 hover:text-gray-800 focus:outline-none">
          <span class="icon-[ic--baseline-help] text-white h-6 w-6"></span>
        </div>
      </button>
      <button>
        <div class="flex items-center p-2 text-gray-600 hover:text-gray-800 focus:outline-none">
          <span class="icon-[ic--baseline-settings] text-white h-6 w-6"></span>
        </div>
      </button>
      <ProfileDropdown @toggleProfileDropdown="toggleProfileDropdown" :showProfileDropdown="showProfileDropdown" />
    </div>
  </div>
</template>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.5s ease, transform 0.5s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
  transform: scale(0.95);
}

@media (min-width: 640px) {

  .fade-enter-active,
  .fade-leave-active {
    transition: opacity 0.3s ease, transform 0.3s ease;
  }
}

@media (min-width: 768px) {
  .grid-cols-1 {
    grid-template-columns: repeat(1, minmax(0, 1fr));
  }
}
</style>