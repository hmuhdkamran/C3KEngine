<script setup lang="ts">
import { DataTable, ConfirmationDialog, PubSub } from 'c3k-library';
import { UsersService } from '@/services';
import type { IUser, RecordPubSub } from '@/models';
import { onMounted, onUnmounted, ref, type Ref } from 'vue';

import EntityOperation from './operation.vue';

const repo = new UsersService();

const data: Ref<IUser[]> = ref([]);
const entity: Ref<IUser> = ref({ UserId: '', Username: '', DisplayName: '', Language: '', Password: '', Salt: '', StatusId: '' });

const deleteDialog: Ref<boolean> = ref(false);
const entityDrawer: Ref<boolean> = ref(false);

const entityTitle: Ref<string> = ref('');

const columns = [
    { key: 'Username', label: 'User Name', sort: true },
    { key: 'DisplayName', label: 'Name', sort: true },
    { key: 'Language', label: 'Language', sort: false, width: '300px', class: 'text-center' },
    { key: 'StatusId', label: 'Status', sort: false, width: '300px', class: 'text-center' },
    { key: 'action', label: 'Action', sort: false, width: '300px', class: 'text-center' },
];

const loadRecords = () => {
    repo.GetAll().then((res: any) => {
        if (res)
            data.value = res.data
    })
}

onMounted(() => loadRecords());

const operation = (action: string, record: IUser | any) => {
    entityTitle.value = action;

    if (action.toLowerCase().startsWith('add')) {
        entity.value = { UserId: '', Username: '', DisplayName: '', Language: '', Password: '', Salt: '', StatusId: '' } as IUser;
    } else if (action.toLowerCase().startsWith('edit')) {
        entity.value = record;
    } else if (action.toLowerCase().startsWith('view')) {
        entity.value = record;
    }

    entityTitle.value.toLowerCase().startsWith('delete') ? deleteDialog.value = true : PubSub.publish<RecordPubSub>("ToggleDrawer", { open: true, title: entityTitle.value, entity: record });
}

const execution = (action: RecordPubSub) => {
    if (action.entity !== null) {
        if (action.title.toLowerCase().startsWith('add')) {
            repo.AddOne(action.entity as IUser).then(() => loadRecords());
        } else if (action.title.toLowerCase().startsWith('edit')) {
            repo.Update(action.entity as IUser).then(() => loadRecords());
        } else if (action.title.toLowerCase().startsWith('delete')) {
            repo.delete((action.entity as IUser).UserId).then(() => loadRecords());
        }
    }

    entityTitle.value.toLowerCase().startsWith('delete') ? deleteDialog.value = false : PubSub.publish<RecordPubSub>("ToggleDrawer", { open: false, title: '', entity: null });
}
</script>

<template>
    <div>
        <DataTable :data="data" :columns="columns" :check-column="false">
            <template #action="{ row }">
                <div class="flex justify-center ">
                    <button
                        class="actionbtn rounded-full bg-white hover:bg-gray-50 shadow-sm hover:shadow-md transition duration-300 
                        transform hover:scale-110 p-1.5 flex items-center justify-center"
                        @click="operation('View Record', row)" title="View Record">
                        <span class="icon-[ep--view] text-yellow-700 text-sm"></span>
                    </button>
                    <button
                        class="actionbtn rounded-full bg-white hover:bg-gray-50 shadow-sm hover:shadow-md transition duration-300 
                        transform hover:scale-110  p-1.5 flex items-center justify-center"
                        @click="operation('Edit Record', row)" title="Edit Record">
                        <span class="icon-[akar-icons--edit] text-blue-700 text-sm"></span>
                    </button>
                    <button
                        class="actionbtn rounded-full bg-white hover:bg-gray-50 shadow-sm hover:shadow-md transition duration-300 
                        transform hover:scale-110 p-1.5 flex items-center justify-center"
                        @click="operation('Delete Record', row)" title="Delete Record">
                        <span class="icon-[hugeicons--delete-02] text-red-700 text-sm"></span>
                    </button>
                </div>
            </template>
        </DataTable>

        <EntityOperation @execute="execution" />
    </div>
</template>
<route lang="yaml">
    meta:
      layout: application
      action: read
  </route>