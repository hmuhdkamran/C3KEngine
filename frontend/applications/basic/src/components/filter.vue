<script lang="ts" setup>
import { ref } from 'vue';
import { useThemePalleteStore } from 'c3-library';
import { setFormOpen } from '@/stores/edit-form';

const store = useThemePalleteStore();
const showFilter = ref(false);

const dropdownOptions = [
    { label: 'Category', options: ['Category 1', 'Category 2', 'Category 3'] },
    { label: 'Status', options: ['Active', 'Inactive', 'Pending'] },
    { label: 'Date', options: ['Today', 'Last Week', 'Last Month'] },
    { label: 'Priority', options: ['High', 'Medium', 'Low'] },
];

</script>

<template>
    <div class="flex gap-1 px-4">
        <button @click="showFilter = !showFilter"
            class="w-8 h-8 p-3 cursor-pointer border-gray-200 border shadow-lg flex items-center justify-center text-white rounded-full"
            :style="{ backgroundColor: store.selectedColor }">
            <span class="fas fa-filter fa-sm"></span>
        </button>

        <div v-if="showFilter"
            class="absolute right-0 mt-12 w-[98%] bg-white shadow-sm rounded-sm border border-gray-200 p-4 z-50">
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
                <div v-for="(dropdown, index) in dropdownOptions" :key="index" class="flex flex-col gap-1">
                    <label class="text-sm font-medium text-gray-600">{{ dropdown.label }}</label>
                    <select class="p-1 border border-gray-300 rounded-sm focus:outline-none transition-all"
                        :style="{ '--ring-color': store.selectedColor, '--border-color': store.selectedColor, }"
                        :class="['focus:ring-1', 'focus:ring-[var(--ring-color)]', 'focus:border-[var(--border-color)]']">
                        <option v-for="(option, i) in dropdown.options" :key="i" :value="option">
                            {{ option }}
                        </option>
                    </select>
                </div>
            </div>
        </div>
        <button @click="setFormOpen(true)"
            class="w-8 h-8 p-3 cursor-pointer border-gray-200 border shadow-lg flex items-center justify-center text-white rounded-full"
            :style="{ backgroundColor: store.selectedColor }">
            <span class="fas fa-plus fa-sm"></span>
        </button>
    </div>
</template>