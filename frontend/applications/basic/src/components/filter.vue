<script lang="ts" setup>
import { ref } from 'vue';
import { useSystemStore, useTableStore, DialogBox } from 'c3-library';
import { setFormOpen } from '@/stores/edit-form';

const store = useSystemStore();
const table = useTableStore();
const color = useSystemStore();

const showFilter = ref(false);
const showDialog = ref(false);

const dropdownOptions = [
    { label: 'Category', options: ['Category 1', 'Category 2', 'Category 3'] },
    { label: 'Status', options: ['Active', 'Inactive', 'Pending'] },
    { label: 'Date', options: ['Today', 'Last Week', 'Last Month'] },
    { label: 'Priority', options: ['High', 'Medium', 'Low'] },
];

const reloadTableData = () => {
};

const openDialog = () => {
    showDialog.value = true;
};

const applyFilters = () => {
    showDialog.value = false;
};

</script>

<template>
    <div class="flex gap-1 px-4">
        <button @click="openDialog"
            class="w-8 h-8 p-3 cursor-pointer border-gray-200 border shadow-lg flex items-center justify-center text-white rounded-full"
            :style="{ backgroundColor: store.application.primaryColor }">
            <span class="fas fa-filter fa-sm"></span>
        </button>

        <button @click="setFormOpen(true)"
            class="w-8 h-8 p-3 cursor-pointer border-gray-200 border shadow-lg flex items-center justify-center text-white rounded-full"
            :style="{ backgroundColor: store.application.primaryColor }">
            <span class="fas fa-plus fa-sm"></span>
        </button>

        <button @click="reloadTableData"
            class="w-8 h-8 p-3 cursor-pointer border-gray-200 border shadow-lg flex items-center justify-center text-white rounded-full"
            :style="{ backgroundColor: store.application.primaryColor }">
            <span class="fas fa-sync-alt fa-sm"></span>
        </button>
    </div>

    <DialogBox :show="showDialog" @close="showDialog = false" :showClose="false">
        <template #default>
            <div class="grid grid-cols-1 gap-4">
                <div class="input-with-icon">
                    <span class="icon fas fa-search"></span>
                    <input type="text" class="input-with-icon" v-model="table.searchQuery" placeholder="Search..." />
                </div>
            </div>
        </template>
        <template #footer>
            <div class="flex justify-end gap-3 col-span-2">
                <button @click="showDialog = false"
                    class="px-3 py-1.5 bg-gray-200 rounded-sm hover:bg-gray-300 transition cursor-pointer">
                    Cancel
                </button>
                <button @click="applyFilters" class="px-3 py-1.5 text-white rounded-sm cursor-pointer"
                    :style="{ backgroundColor: color.application.primaryColor }">
                    Apply
                </button>
            </div>
        </template>
    </DialogBox>
</template>