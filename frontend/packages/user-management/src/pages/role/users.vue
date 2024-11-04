<script setup lang="ts">
import { DataTable } from "c3k-library";
import { newGuid } from "c3k-library";
import { ref, onMounted, type Ref } from "vue";
import { UsersService } from "@/services/roles/users-service";
import type { IUser } from '@/models';

const columns = [
    { key: 'check', label: 'Check', sort: false, check: true },
    { key: 'username', label: 'Username', sort: true },
    { key: 'language', label: 'Language', sort: true },
    { key: 'status', label: 'Status', sort: false, width: '100px', class: 'text-center' },
    { key: 'action', label: 'Action', sort: false, width: '100px', class: 'text-center' },
];

const currentEntry: Ref<IUser> = ref({
    UserId: newGuid(),
    Username: '',
    DisplayName: '',
    Language: '',
    Password: '',
    Salt: '',
    StatusId: '',
    Status: false,
});

const itemList = ref<IUser[]>([]);
const repository: UsersService = new UsersService();

const load = () => {
    console.log("Reached load");
    repository.GetAll().then((response: any) => {
        console.log("Got Response");
        if (response) itemList.value = response.data
    })
}

onMounted(() => {
    console.log("Reached Mounted");
    load();
});


const openModal = (action: string, row: IUser) => {
    currentEntry.value = row;
    console.log("Data currentEntry", currentEntry.value, action);
};


</script>

<template>
    <div v-if="itemList">
        <button class="btn-gradient px-4 py-2" @click="load()">Load</button>
        <DataTable :data="itemList" :columns="columns" :check-column="false">
            <template #status="{ row }">
                <span :class="row.Status ? 'bg-green-100 text-green-700' : 'bg-yellow-100 text-yellow-700'"
                    class="px-1 py-0.5 rounded-full">
                    {{ row.Status ? 'Active' : 'Inactive' }}
                </span>
            </template>
            <template #action="{ row }">
                <div class="flex justify-center space-x-2">
                    <button class="grid-action-btn hover-btn-warning" @click="openModal('view', row)">
                        <span class="icon-[ep--view]"></span>
                    </button>
                    <button class="grid-action-btn hover-btn-primary" @click="openModal('edit', row)">
                        <span class="icon-[akar-icons--edit]"></span>
                    </button>
                    <button class="grid-action-btn hover-btn-danger" @click="openModal('delete', row)">
                        <span class="icon-[hugeicons--delete-02]"></span>
                    </button>
                </div>
            </template>
        </DataTable>
    </div>
</template>
