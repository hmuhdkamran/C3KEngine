<script setup lang="ts">
import { ref, computed } from 'vue';
import Card from '@/layouts/components/card.vue';
import Dashboardheader from '@/layouts/components/dashboardheader.vue';
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

}

function filterByCategory(category: string) {
    selectedCategory.value = category;
}

function toggleFilters() {
    alert('Filters button clicked');
}

function groupByCategory() {
    alert('Group By button clicked');
}

function toggleFavorites() {
    showFavorites.value = !showFavorites.value;
    alert('Favorites button clicked');
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
    <div class="h-screen mt-12 flex flex-col">
        <div class="bg-gray-200 py-2 px-4 flex items-center justify-between">
            <div class="text-lg font-semibold">
                <a href="#" @click.prevent="goToMain">Apps</a>
                <span class="mx-2">/</span>
                {{ selectedCardTitle }}
            </div>
            <div class="flex items-center space-x-4">
                <input type="text" placeholder="Search..." class="px-3 py-1 border rounded" v-model="searchQuery"
                    @input="filterCards" />
                <button @click="toggleFilters" class="bg-violet-500 text-white px-3 py-1 rounded">Filters</button>
                <button @click="groupByCategory" class="bg-violet-500 text-white px-3 py-1 rounded">Group
                    By</button>
                <button @click="toggleFavorites" class="bg-violet-500 text-white px-3 py-1 rounded">Favorites</button>
            </div>
        </div>

        <div v-if="!showModulePage" class="flex flex-1">
            <div class="bg-gray-100 w-64 p-4">
                <div class="text-lg font-semibold mb-4">
                    <span class="icon-[ion--folder-sharp] text-violet-600"></span>
                    <a href="#" :class="{ 'text-violet-600': selectedCategory === 'All' }"
                        @click.prevent="filterByCategory('All')"> CATEGORIES</a>
                </div>
                <ul class="px-8">
                    <li class="mb-2">
                        <a href="#" :class="{ 'text-violet-600': selectedCategory === 'HRMS' }"
                            @click.prevent="filterByCategory('HRMS')"> HRMS</a>
                    </li>
                    <li class="mb-2">
                        <a href="#" :class="{ 'text-violet-600': selectedCategory === 'Retail' }"
                            @click.prevent="filterByCategory('Retail')"> Retail</a>
                    </li>
                    <li class="mb-2">
                        <a href="#" :class="{ 'text-violet-600': selectedCategory === 'Production' }"
                            @click.prevent="filterByCategory('Production')">Production</a>
                    </li>
                </ul>
            </div>
            <div class="flex-1 container mx-auto py-24">
                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6">
                    <Card v-for="card in filteredCards" :key="card.title" :title="card.title"
                        :description="card.description" :status="card.status" :buttonText="card.buttonText"
                        :iconClass="card.iconClass" @click="handleCardClick(card.title)">
                        <!-- <template #header>
                            <div class="flex-1">
                                <div class="font-bold text-xl mb-1 flex items-center justify-between">
                                    <span>{{ card.title }}</span>
                                    <span class="icon-[ph--dots-three-vertical-bold]"></span>
                                </div>
                                <p class="text-gray-500">{{ card.description }}</p>
                            </div>
                        </template> -->
                    </Card>
                </div>
            </div>
        </div>

        <div v-else>
            <component :is="moduleComponent" :cardTitle="selectedCardTitle" />
        </div>
    </div>
    <Dashboardheader />
</template>

<route lang="yaml">
    meta:
      layout: auth
      action: read
</route>