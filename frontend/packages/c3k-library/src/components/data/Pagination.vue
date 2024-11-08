<script setup lang="ts">
import { useTableStore } from '@/plugins';

const tableStore = useTableStore();

</script>

<template>
    <div class="mt-6 flex flex-col md:flex-row justify-between items-center space-y-4 md:space-y-0">
        <div class="flex items-center space-x-2">
            <button @click="tableStore.setPage(1)" :disabled="tableStore.currentPage === 1"
                class="text-gray-800 hover:text-gray-600 transition-colors disabled:opacity-50 disabled:cursor-not-allowed">
                <span class="icon-[material-symbols--first-page] w-5 h-5"></span>
            </button>
            <button @click="tableStore.setPage(tableStore.currentPage - 1)" :disabled="tableStore.currentPage === 1"
                class="text-gray-800  hover:text-gray-600 transition-colors disabled:opacity-50 disabled:cursor-not-allowed">
                <span class="icon-[material-symbols--keyboard-arrow-left] w-5 h-5"></span>
            </button>
            <div class="text-gray-800  text-sm mb-2">
                <span class="text-bold">Page</span> {{ tableStore.currentPage }} of {{ tableStore.totalPages }}
            </div>
            <button @click="tableStore.setPage(tableStore.currentPage + 1)"
                :disabled="tableStore.currentPage === tableStore.totalPages"
                class="text-gray-800  hover:text-gray-600 transition-colors disabled:opacity-50 disabled:cursor-not-allowed">
                <span class="icon-[material-symbols--keyboard-arrow-right] w-5 h-5"></span>
            </button>
            <button @click="tableStore.setPage(tableStore.totalPages)"
                :disabled="tableStore.currentPage === tableStore.totalPages"
                class="text-gray-800  hover:text-gray-600 transition-colors disabled:opacity-50 disabled:cursor-not-allowed">
                <span class="icon-[material-symbols--last-page] w-5 h-5"></span>
            </button>
        </div>
    </div>
</template>