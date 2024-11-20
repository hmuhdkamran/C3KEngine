<script setup lang="ts">
import { useTableStore } from '@/plugins/store';
import { Icon } from '@iconify/vue';
import { ref, computed } from 'vue';

const props = defineProps({
    data: {
        type: Array as () => Record<string, any>[],
        default: () => [],
    },
    columns: {
        type: Array as () => {
            key: string,
            label: string,
            sort?: boolean,
            width?: string,
            class?: string,
            check?: boolean
        }[],
        default: () => []
    },
    checkColumn: {
        type: Boolean,
        default: false,
    }
});

const tableStore = useTableStore();

const itemsPerPage = computed(() => tableStore.itemsPerPage);
const sortColumn = ref<string>('');
const sortOrder = ref<'asc' | 'desc'>('asc');
const selectedRecords = ref<Record<string, any>[]>([]);
const selectAll = ref(false);

const filteredRecords = computed(() => {
    const query = tableStore.searchQuery.toLowerCase();
    const filteredRecords = props.data.filter(record =>
        Object.values(record).some(value =>
            value.toString().toLowerCase().includes(query)
        )
    ).sort((a, b) => {
        const compareA = a[sortColumn.value]?.toString().toLowerCase() || '';
        const compareB = b[sortColumn.value]?.toString().toLowerCase() || '';
        if (compareA < compareB) return sortOrder.value === 'asc' ? -1 : 1;
        if (compareA > compareB) return sortOrder.value === 'asc' ? 1 : -1;
        return 0;
    });

    tableStore.updateTotalRecords(filteredRecords.length);
    return filteredRecords;
});

const paginatedRecords = computed(() => {
    const start = (tableStore.currentPage - 1) * itemsPerPage.value;
    const end = start + itemsPerPage.value;
    return filteredRecords.value.slice(start, end);
});

const changeSort = (column: string, sort: boolean = true) => {
    if (!sort) {
        return;
    }
    if (sortColumn.value === column) {
        sortOrder.value = sortOrder.value === 'asc' ? 'desc' : 'asc';
    } else {
        sortColumn.value = column;
        sortOrder.value = 'asc';
    }
};

const toggleSelectAll = () => {
    if (selectAll.value) {
        selectedRecords.value = [...paginatedRecords.value];
    } else {
        selectedRecords.value = [];
    }
};

const selectedFilter = ref('');
const data = ref<Record<string, any>[]>([]);

const refreshData = async () => {
    console.log('Data refreshed');
    try {
        const response = await fetch('api/data');
        const newData = await response.json();
        data.value = newData;
        console.log('New Data:', newData);
    } catch (error) {
        console.error('Error refreshing data:', error);
    }
};

const applyFilter = () => {
    console.log('Filter applied for:', selectedFilter.value);
    if (selectedFilter.value) {
        data.value = props.data.filter((item) =>
            item[selectedFilter.value] !== undefined
        );
        console.log('Filtered Data:', data.value);
    } else {
        data.value = [...props.data];
        console.log('No filter selected, showing all data.');
    }
};



const exportData = () => {
    if (!filteredRecords.value.length) {
        console.log('No data available for export');
        return;
    }
    console.log('Exporting data to CSV');
    const dataToExport = filteredRecords.value;
    const csvContent = dataToCSV(dataToExport);
    const blob = new Blob([csvContent], { type: 'text/csv' });
    const url = URL.createObjectURL(blob);
    const link = document.createElement('a');
    link.href = url;
    link.download = 'exported-data.csv';
    link.click();
    URL.revokeObjectURL(url);
};


function dataToCSV(data: Record<string, any>[]) {
    const header = Object.keys(data[0]).join(',') + '\n';
    const rows = data
        .map((item) =>
            Object.values(item)
                .map((value) => `"${value}"`)
                .join(',')
        )
        .join('\n');

    return header + rows;
}
</script>

<template>
    <div>
        <div class="mb-4 flex justify-end items-center">
            <div class="flex space-x-4 items-center">
                <div class="flex space-x-1">
                    <button class="flex items-center hover:bg-gray-100 rounded" @click="exportData"
                        aria-label="Export to csv">
                        <Icon icon="pajamas-import" class="text-lg text-gray-600 mr-2" />
                    </button>
                    <button class="flex items-center hover:bg-gray-100 rounded" @click="refreshData"
                        aria-label="Refresh Data">
                        <Icon icon="ci:arrows-reload-01" class="text-lg text-gray-600 mr-2" />
                    </button>
                </div>
                <select v-model="selectedFilter" class="p-1 input-complete" @change="applyFilter"
                    aria-label="Filter Data">
                    <option value="">All</option>
                    <option v-for="column in columns" :key="column.key" :value="column.key">
                        Filter by {{ column.label }}
                    </option>
                </select>
            </div>
        </div>
        <div class="overflow-x-auto border rounded-sm shadow-md">
            <table class="table-auto w-full border-collapse">
                <thead>
                <tr class="bg-gray-200 border-b border-gray-300">
                    <template v-for="column in props.columns" :key="column.key">
                        <th v-if="column.check ?? false"
                        class="cursor-pointer">
                        <input class="cursor-pointer" type="checkbox" v-model="selectAll" @change="toggleSelectAll" />
                    </th>
                        <th v-else @click="changeSort(column.key, column.sort ?? false)"
                            :class="['cursor-pointer', column.class || '']"
                            :style="{ width: column.width || 'auto' }">
                            {{ column.label }}
                            <span v-if="sortColumn === column.key && column.sort !== false" class="ml-1 text-md">
                                {{ sortOrder === 'asc' ? '↑' : '↓' }}
                            </span>
                        </th>
                    </template>
                </tr>
            </thead>
                <tbody>
                <tr v-for="record in paginatedRecords" :key="record[props.columns[0].key]">
                    <template v-for="column in props.columns">
                        <template v-if="column.check ?? false">
                            <td class="cursor-pointer text-center">
                                <input class="cursor-pointer" type="checkbox" v-model="selectedRecords" :value="record" />
                            </td>
                        </template>
                        <template v-else>
                        <td class="p-1" :class="column.class || ''" :style="{ width: column.width || 'auto' }" :key="column.key">
                            <slot :name="column.key" :field="column.key" :row="record" v-if="$slots[column.key]"></slot>
                            <span v-html="record[column.key]" v-else></span>
                        </td>
                    </template>
                    </template>
                </tr>
            </tbody>
            </table>
        </div>
        <div class="mt-4 text-sm flex justify-between items-center">
            <div>
                Showing {{ (tableStore.currentPage - 1) * itemsPerPage + 1 }} to
                {{ Math.min(tableStore.currentPage * itemsPerPage, filteredRecords.length) }} of
                {{ filteredRecords.length }} records
            </div>
        </div>

    </div>
</template>

<style scoped>
.overflow-x-auto {
    overflow-x: auto;
    -webkit-overflow-scrolling: touch;
}
</style>
