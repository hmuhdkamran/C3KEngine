<script setup lang="ts">
import { useTableStore } from '@/plugins/store';
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
            sort: boolean,
            width: string,
            class: string,
            check: boolean
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
</script>

<template>
    <table>
        <thead>
            <tr class="bg-gray-100 border-b border-gray-300">
                <template v-for="column in props.columns" :key="column.key">
                    <th v-if="column.check"
                        class="cursor-pointer">
                        <input class="cursor-pointer" type="checkbox" v-model="selectAll" @change="toggleSelectAll" />
                    </th>
                    <th v-else @click="changeSort(column.key, column.sort)"
                        class="cursor-pointer ">
                        {{ column.label }}
                        <span v-if="sortColumn === column.key && column.sort" class="ml-1 text-md">
                            {{ sortOrder === 'asc' ? '↑' : '↓' }}
                        </span>
                    </th>
                </template>
            </tr>
        </thead>
        <tbody>
            <tr v-for="record in paginatedRecords" :key="record[props.columns[0].key]">
                <template v-for="column in props.columns">
                    <template v-if="column.check">
                        <td class="cursor-pointer text-center">
                            <input class="cursor-pointer" type="checkbox" v-model="selectedRecords" :value="record" />
                        </td>
                    </template>
                    <template v-else>
                        <td class="p-1" :class="column.class" :key="column.key"
                            v-if="typeof $slots[column.key] !== 'undefined'">
                            <slot :name="column.key" :field="column.key" :row="record"></slot>
                        </td>
                        <td class="p-1" :class="column.class" :key="column.key + `-els`" v-else>
                            <span v-html="record[column.key]"></span>
                        </td>
                    </template>
                </template>
            </tr>
        </tbody>
    </table>
</template>

<style scoped></style>