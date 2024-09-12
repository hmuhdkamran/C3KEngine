<script setup lang="ts">
import { ref, computed } from 'vue';
import Card from '@/layouts/components/card.vue';
import DashboardHeader from '@/layouts/components/dashboardheader.vue';
import HRMSmodule from '@/layouts/components/HRMSmodule.vue';
import Retailmodule from '@/layouts/components/Retailmodule.vue';
import Productionmodule from '@/layouts/components/Productionmodule.vue';
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
   <div class="min-h-screen bg-gray-100 pt-12">
        <DashboardHeader />

        <!-- <div class="container mx-auto py-6 px-4 sm:px-6 lg:px-8">
            <div class="flex flex-col md:flex-row justify-between items-center bg-white rounded-lg shadow p-4 mb-6">
                <div class="text-lg font-semibold">
                    <a href="#" @click.prevent="goToMain" class="text-violet-600">Apps</a>
                    <span class="mx-2 text-gray-400">/</span>
                    {{ selectedCardTitle }}
                </div>
                <div class="flex items-center space-x-4 mt-4 md:mt-0">
                    <div class="relative">
                        <input type="text" placeholder="Search..."
                            class="pl-10 pr-4 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-violet-500"
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
        </div> -->
    </div>  
</template>

<route lang="yaml">
    meta:
      layout: auth
      action: read
</route>
