<script setup lang="ts">
import { useTableStore } from '@/plugins';
import { Pagination } from '@/index'

const tableStore = useTableStore();
</script>

<template>
    <div class="flex flex-col space-y-2">
        <div class="relative">
            <i class="icon-[mdi--magnify] absolute left-2 top-3 text-gray-400"></i>
            <input type="text" placeholder="Search..." class="input-bottom pl-6 w-full"
                v-model="tableStore.searchQuery" />
        </div>
        <div class="flex flex-wrap items-center justify-between lg:space-x-96 sm:space-x-80 md:space-x-12">
            <div class="flex justify-start space-x-1 items-center text-sm ">
                <slot name="action" />
            </div>
            <Pagination />
        </div>
    </div>
</template>