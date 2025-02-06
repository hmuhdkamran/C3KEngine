<script setup lang="ts">
import { useTableStore } from '@/index';

const tableStore = useTableStore();

</script>

<template>
    <div class="mt-6 flex flex-col md:flex-row justify-between items-center space-y-4 md:space-y-0">
        <div class="flex items-center space-x-2">
            <button @click="tableStore.setPage(1)" :disabled="tableStore.currentPage === 1"
                class="text-gray-800 hover:text-gray-600 transition-colors disabled:opacity-50 disabled:cursor-not-allowed">
                <span icon="fa-solid fa-angles-left" class="w-4 h-4"></span>
            </button>
            <button @click="tableStore.setPage(tableStore.currentPage - 1)" :disabled="tableStore.currentPage === 1"
                class="text-gray-800  hover:text-gray-600 transition-colors disabled:opacity-50 disabled:cursor-not-allowed">
                <span icon="fa-solid fa-angle-left" class="w-4 h-4"></span>
            </button>
            <div class="text-gray-800  text-sm">
                <span class="text-bold">Page</span> {{ tableStore.currentPage }} of {{ tableStore.totalPages }}
            </div>
            <button @click="tableStore.setPage(tableStore.currentPage + 1)"
                :disabled="tableStore.currentPage === tableStore.totalPages"
                class="text-gray-800  hover:text-gray-600 transition-colors disabled:opacity-50 disabled:cursor-not-allowed">
                <span icon="fa-solid fa-angle-right" class="w-4 h-4"></span>
            </button>
            <button @click="tableStore.setPage(tableStore.totalPages)"
                :disabled="tableStore.currentPage === tableStore.totalPages"
                class="text-gray-800  hover:text-gray-600 transition-colors disabled:opacity-50 disabled:cursor-not-allowed">
                <span icon="fa-solid fa-angles-right" class="w-4 h-4"></span>
            </button>
        </div>
    </div>
</template>
