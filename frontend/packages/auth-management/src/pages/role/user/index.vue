<script setup lang="ts">
import { DataTable, ConfirmationDialog } from 'c3k-library';
import type { IUser, RecordPubSub } from '@/models';
import { computed, onMounted } from 'vue';

import EntityOperation from './operation.vue';
import { useRoleUserStore } from '@/stores';

const store = useRoleUserStore();
const deleteDialog = computed(() => store.store.Open && store.store.OperationType === 'delete');
const columns = [
    { key: 'Username', label: 'User Name', sort: true },
    { key: 'DisplayName', label: 'Name', sort: true },
    { key: 'Language', label: 'Language', sort: false, width: '300px', class: 'text-center' },
    { key: 'StatusId', label: 'Status', sort: false, width: '300px', class: 'text-center' },
    { key: 'action', label: 'Action', sort: false, width: '300px', class: 'text-center' },
];

onMounted(() => store.execute('get'));

</script>

<template>
    <div>
        <DataTable :data="store.items" :columns="columns" :check-column="false">
            <template #action="{ row }">
                <div class="flex justify-center ">
                    <button class="actionbtn rounded-full bg-white hover:bg-gray-50 shadow-sm hover:shadow-md transition duration-300 
                        transform hover:scale-110 p-1.5 flex items-center justify-center"
                        @click="store.selectItem((row as IUser), 'View Record')" title="View Record">
                        <span class="icon-[ep--view] text-yellow-700 text-sm"></span>
                    </button>
                    <button class="actionbtn rounded-full bg-white hover:bg-gray-50 shadow-sm hover:shadow-md transition duration-300 
                        transform hover:scale-110  p-1.5 flex items-center justify-center"
                        @click="store.selectItem((row as IUser), 'Edit Record')" title="Edit Record">
                        <span class="icon-[akar-icons--edit] text-blue-700 text-sm"></span>
                    </button>
                    <button class="actionbtn rounded-full bg-white hover:bg-gray-50 shadow-sm hover:shadow-md transition duration-300 
                        transform hover:scale-110 p-1.5 flex items-center justify-center"
                        @click="store.selectItem((row as IUser), 'Delete Record')" title="Delete Record">
                        <span class="icon-[hugeicons--delete-02] text-red-700 text-sm"></span>
                    </button>
                </div>
            </template>
        </DataTable>

        <EntityOperation />
        <ConfirmationDialog :is-visible="deleteDialog" title="Delete Record"
            message="Are you sure you want to remove this record" @cancel="store.store.resetOperation()"
            @confirm="store.execute('delete')" />
    </div>
</template>
<route lang="yaml">
    meta:
      layout: application
      action: read
  </route>