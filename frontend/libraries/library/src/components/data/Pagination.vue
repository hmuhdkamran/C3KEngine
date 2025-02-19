<script setup lang="ts">
import { ref, watch } from 'vue';
import { useTableStore } from '@/index';

const tableStore = useTableStore();

const perPageOptions = [10, 20, 30, 40];
const perPage = ref(tableStore.itemsPerPage);

watch(perPage, (newPerPage) => {
    tableStore.updateItemsPerPage(newPerPage);
});
</script>

<template>
    <div class="flex flex-col md:flex-row justify-between items-center space-y-4 md:space-y-0">
        <div class="flex items-center space-x-2">
            <button @click="tableStore.setPage(1)" :disabled="tableStore.currentPage === 1"
                class="px-1 text-gray-800 bg-white border border-gray-300 rounded hover:bg-gray-100 transition-colors disabled:opacity-50 disabled:cursor-not-allowed">
                <span class="fa-regular fa-angles-left fa-xs"></span>
            </button>
            <button @click="tableStore.setPage(tableStore.currentPage - 1)" :disabled="tableStore.currentPage === 1"
                class="px-2 text-gray-800 bg-white border border-gray-300 rounded hover:bg-gray-100 transition-colors disabled:opacity-50 disabled:cursor-not-allowed">
                <span class="fa-regular fa-angle-left fa-xs"></span>
            </button>
            <div class="text-gray-800 text-sm">
                <span class="font-bold">Page</span> {{ tableStore.currentPage }} of {{ tableStore.totalPages }}
            </div>
            <button @click="tableStore.setPage(tableStore.currentPage + 1)"
                :disabled="tableStore.currentPage === tableStore.totalPages"
                class="px-2 text-gray-800 bg-white border border-gray-300 rounded hover:bg-gray-100 transition-colors disabled:opacity-50 disabled:cursor-not-allowed">
                <span class="fa-regular fa-angle-right fa-xs"></span>
            </button>
            <button @click="tableStore.setPage(tableStore.totalPages)"
                :disabled="tableStore.currentPage === tableStore.totalPages"
                class="px-1 text-gray-800 bg-white border border-gray-300 rounded hover:bg-gray-100 transition-colors disabled:opacity-50 disabled:cursor-not-allowed">
                <span class="fa-regular fa-angles-right fa-xs"></span>
            </button>
            <div class="ml-1">
                <select v-model="perPage" id="perPage"
                    class="p-1 text-gray-800 text-sm bg-white border border-gray-300 rounded hover:bg-gray-100 transition-colors cursor-pointer">
                    <option v-for="option in perPageOptions" :key="option" :value="option">{{ option }}/page</option>
                </select>
            </div>
        </div>
    </div>
</template>
