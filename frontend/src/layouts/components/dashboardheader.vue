<script setup lang="ts">
import { ref, computed } from 'vue';
import Card from '@/layouts/components/card.vue';
import Sidebar from './sidebar.vue';
import Notifications from './notification.vue';
import ProfileDropdown from './profile.vue';
import logo from "@/assets/images/vue.svg";
import HRMSmodule from '@/layouts/components/HRMSmodule.vue';
import Retailmodule from '@/layouts/components/Retailmodule.vue';
import Productionmodule from '@/layouts/components/Productionmodule.vue';

const isOpen = ref(false);
const showModuleDropdown = ref(false);
const showProfileDropdown = ref(false);
const collapsed = ref(false);

import { useRouter } from 'vue-router';

const router = useRouter()
const searchQuery = ref('');
const selectedCategory = ref('All');
const showFavorites = ref(false);
const showModulePage = ref(false);
const selectedCardTitle = ref('');
const currentModule = ref('');
const selectedFilter = ref('');
const currentPage = ref(1);

const cards = ref([
    {
        title: 'HRMS',
        description: 'Streamline HR with our software',
        status: 'Installed',
        buttonText: 'MODULE INFO',
        iconClass: 'icon-[fluent-mdl2--recruitment-management]',
        category: 'HRMS',
    },
    {
        title: 'Retail',
        description: 'Boost retail with streamlined software.',
        status: 'Activate',
        buttonText: 'LEARN MORE',
        iconClass: 'icon-[vaadin--shop]',
        category: 'Retail',
    },
    {
        title: 'Production',
        description: 'Looking forward to process improvements.',
        status: 'Installed',
        buttonText: 'LEARN MORE',
        iconClass: 'icon-[mdi--office-building-settings-outline]',
        category: 'Production',
    }
]);

function toggleModule() {
  showModuleDropdown.value = !showModuleDropdown.value;
  isOpen.value = false;
  showProfileDropdown.value = false;
}

function toggleSidebar() {
  collapsed.value = !collapsed.value;
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

const filteredCards = computed(() => {
    return cards.value.filter((card) => {
        const matchesCategory = selectedCategory.value === 'All' || card.category === selectedCategory.value;
        const matchesSearch = card.title.toLowerCase().includes(searchQuery.value.toLowerCase());
        return matchesCategory && matchesSearch;
    });
});

function filterCards() {
    currentPage.value = 1;
}

function filterByCategory(category: string) {
    selectedCategory.value = category;
}

function toggleFilters() {
    selectedFilter.value = selectedFilter.value === 'Favorites' ? 'All' : 'Favorites';
}

function groupByCategory() {
    alert('Group By button clicked');
}

function toggleFavorites() {
    showFavorites.value = !showFavorites.value;
    // alert('Favorites button clicked');
}


function handleCardClick(cardTitle: string) {
    showModulePage.value = true;
    selectedCardTitle.value = cardTitle;
    currentModule.value = cardTitle;
}

const moduleComponent = computed(() => {
    switch (currentModule.value) {
        case 'HRMS':
            return HRMSmodule;
        case 'Retail':
            return Retailmodule;
        case 'Production':
            return Productionmodule;
        default:
            return null;
    }
});

function goToMain() {
    router.push('/app/main');
}

</script>

<template>
  <div class="flex h-screen">
    <Sidebar :collapsed="collapsed" @toggleSidebar="toggleSidebar" />
    <div :class="collapsed ? 'ml-16' : 'ml-64'" class="flex-grow transition-all duration-300">
      <div class="fixed top-0 left-0 w-full px-4 sm:px-6 lg:px-8 flex h-16 items-center bg-gradient-to-r from-violet-600 to-blue-500 justify-between shadow-lg z-50">
        <div class="flex items-center">
          <button @click="toggleSidebar" class="flex items-center text-white hover:text-gray-200 focus:outline-none">
            <span class="icon-[fluent--navigation-unread-20-filled] h-8 w-8"></span>
          </button>
          <a href="/" class="text-2xl font-semibold text-white flex items-center space-x-2">
            <img :src="logo" alt="C3K Engine Logo" class="h-10 w-auto object-contain" />
            <span>C3K Engine</span>
          </a>
        </div>
        <div class="flex items-center">
          <button class="flex items-center text-white hover:text-gray-200 focus:outline-none">
            <span class="icon-[ic--baseline-search] h-6 w-6"></span>
          </button>
          <Notifications @toggleDropdown="toggleDropdown" :isOpen="isOpen" />
          <ProfileDropdown @toggleProfileDropdown="toggleProfileDropdown" :showProfileDropdown="showProfileDropdown" />
        </div>
      </div>
      <div class="mt-6 sm:px-6 lg:px-8">
        <!-- <div class="container mx-auto py-6 px-4 sm:px-6 lg:px-8"> -->
            <div class="flex flex-col md:flex-row justify-between items-center bg-white rounded-lg shadow p-4 mb-6">
                <div class="text-lg font-semibold">
                    <a href="#" @click.prevent="goToMain" class="text-violet-600">Apps</a>
                    <span class="mx-2 text-gray-400">/</span>
                    {{ selectedCardTitle }}
                </div>
                <div class="flex items-center space-x-4 mt-4 md:mt-0">
                    <div class="relative">
                        <input type="text" placeholder="Search..."
                            class="pl-10 pr-4 py-2 border ro
                            
                            
                            unded-lg focus:outline-none focus:ring-2 focus:ring-violet-500"
                            v-model="searchQuery" />
                        <i class="icon-[fluent--search-20-filled] h-6 w-6 absolute left-3 top-2 text-gray-400"></i>
                    </div>
                    <button @click="toggleFilters"
                        class="flex items-center bg-white border border-gray-300 text-gray-700 px-3 py-1 rounded-md hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-500">
                        <i class="icon-[fluent--filter-16-filled] mr-2"></i>
                        Filters
                    </button>

                    <button @click="groupByCategory"
                        class="flex items-center bg-white border border-gray-300 text-gray-700 px-3 py-1 rounded-md hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-500">
                        <i class="icon-[fluent--group-24-filled] mr-2"></i>
                        Group By
                    </button>

                    <button @click="toggleFavorites"
                        class="flex items-center bg-white border border-gray-300 text-gray-700 px-3 py-1 rounded-md hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-500">
                        <i class="icon-[mdi--star-outline] mr-2"></i>
                        Favorites
                    </button>
                </div>
            </div>

            <div v-if="!showModulePage" class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
                <Card v-for="card in filteredCards" :key="card.title" :title="card.title"
                    :description="card.description" :status="card.status" :buttonText="card.buttonText"
                    :iconClass="card.iconClass" @click="handleCardClick(card.title)" />
            </div>

            <div v-else>
                <component :is="moduleComponent" :cardTitle="selectedCardTitle" />
            </div>
        <!-- </div> -->
      </div>
    </div>
  </div>
</template>