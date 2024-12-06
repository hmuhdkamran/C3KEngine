<script setup lang="ts">
import { ref, computed } from 'vue';
import { useRouter } from 'vue-router';
import { useTableStore, Filter, LocalStorageHelper, Card } from 'c3k-library';
import { store } from '@/stores';
import type { UserProduct } from '@/models/user-products';

const tableStore = useTableStore();

const router = useRouter()
const showModulePage = ref(false);
const selectedCardTitle = ref('');
const currentModule = ref('');

const cards = store.userModules as UserProduct[];

const filteredCards = computed(() => {
    if(cards && cards.length > 0) {
        console.log(cards)
        return cards.filter(card => card.FullName.toLowerCase().includes((tableStore.searchQuery.toLowerCase())))
    } else return [];
});

tableStore.updateTotalRecords(filteredCards.value.length);

const pageHeading = computed(() => {
    return selectedCardTitle.value !== '' ? selectedCardTitle.value : 'Apps';
});

function handleCardClick(card: UserProduct) {
    LocalStorageHelper.set("application", `c3k-${card.Abbreviation.replace('/', '-')}`)
    router.replace(`/c3k-${card.Abbreviation.replace('/', '-')}`);

    // showModulePage.value = true;
    selectedCardTitle.value = card.FullName;
    currentModule.value = card.FullName;
}

function goToMain() {
    router.replace('/dashboard');
}
</script>
<template>
    <div>
        <div
            class="border-b border-gray-300 mt-12 py-2 px-4 flex flex-col md:flex-row justify-between shadow-md w-full space-y-4 md:space-y-0 md:space-x-8">
            <div class="w-full md:w-1/2 flex flex-col justify-center space-y-6">
                <div class="px-3">
                    <h1 class="text-2xl font-bold text-gray-800">{{ pageHeading }}</h1>
                </div>
                <nav class="sm:text-md px-2 text-sm flex space-x-2">
                    <div @click.prevent="goToMain"
                        class="hover:underline cursor-pointer text-gray-600 flex items-center">
                        <i class="icon-[mdi--home-outline] mr-1 text-gray-500"></i> Apps
                    </div>
                </nav>
            </div>
            <div class="w-full md:w-1/2 flex flex-col space-y-2">
                <Filter />
            </div>
        </div>
        <div v-if="!showModulePage">
            <div class="flex-1 mx-auto py-6 px-6">
                <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-6 gap-6">
                    <Card v-for="(card, index) in store.userModules" :key="index" :title="card.FullName"
                        :description="card.Description" :status="'Active'" :buttonText="'Explore...'"
                        :iconClass="card.Icon" @click="handleCardClick(card)" :showHeader="true" :showFooter="false">
                    </Card>
                </div>
            </div>
        </div>
    </div>
</template>
<route lang="yaml">
    meta:
      layout: authentication
      action: read
  </route>