<script setup lang="ts">
import { DataTable, ConfirmationDialog, Drawer } from "c3k-library";
import { newGuid } from "c3k-library";
import { ref, onMounted, watch, type Ref } from "vue";
import { UsersService } from "@/services/roles/users-service";
import type { IUsers } from '@/models';

const columns = [
    { key: 'check', label: 'Check', sort: false, check: true },
    { key: 'username', label: 'Username', sort: true },
    { key: 'language', label: 'Language', sort: true },
    { key: 'status', label: 'Status', sort: false, width: '100px', class: 'text-center' },
    { key: 'action', label: 'Action', sort: false, width: '100px', class: 'text-center' },
];

const item: Ref<IUsers> = ref({
    UserId: newGuid(),
    Username: '',
    DisplayName: '',
    Language: '',
    Password: '',
    Salt: '',
    StatusId: '',
    Status: false,
});

const itemList = ref<IUsers[]>([]);
const repository = new UsersService();
const isDeleteDialogVisible = ref(false);
const isDrawerVisible = ref(false);
const isEditMode = ref(false);
const currentEntry = ref<IUsers>({
    UserId: '',
    Username: '',
    DisplayName: '',
    Language: '',
    Password: '',
    Salt: '',
    StatusId: '',
});


const loadData = async () => {
    try {
        const response = await repository.GetAll();
        console.log("API Response:", response);
        itemList.value = response.data;
        console.log("Fetched user data:", itemList.value);
    } catch (error) {
        console.error("Failed to load user data:", error);
    }
};

onMounted(() => {
    loadData();
});


const openModal = (action: string, row: IUsers) => {
    currentEntry.value = row;
    console.log("Data currentEntry", currentEntry.value)
    isEditMode.value = action === 'edit';
    isDrawerVisible.value = true;
};

watch(currentEntry, (newVal) => {
    console.log("Current Entry updated:", newVal);
});

const closeDrawer = () => {
    isDrawerVisible.value = false;
    isEditMode.value = false;
};

const onDelete = () => {
    isDeleteDialogVisible.value = true;
};

const onConfirmDelete = () => {
    isDeleteDialogVisible.value = false;
};

const onCancelDelete = () => {
    isDeleteDialogVisible.value = false;
};
</script>

<template>
    <div>
        <DataTable :data="itemList" :columns="columns" :check-column="true">
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
                    <button class="grid-action-btn hover-btn-danger" @click="onDelete">
                        <span class="icon-[hugeicons--delete-02]"></span>
                    </button>
                </div>
            </template>
        </DataTable>

        <ConfirmationDialog v-if="isDeleteDialogVisible" title="Confirm Delete"
            message="Are you sure you want to delete this entry?" :isVisible="isDeleteDialogVisible"
            @confirm="onConfirmDelete" @cancel="onCancelDelete" />

        <Drawer :isOpen="isDrawerVisible" :title="isEditMode ? 'Edit Entry' : 'View Entry'" position="right"
            size="w-1/4">
            <template #header>
                <div class="w-full flex justify-between items-center p-4 bg-gray-100 border-b">
                    <h2 class="text-lg font-semibold text-gray-800">{{ isEditMode ? "Edit" : "View" }}</h2>
                    <button @click="closeDrawer" class="text-gray-400 hover:text-gray-600 focus:outline-none">
                        <span class="icon-[fluent--dismiss-20-filled] h-4 w-4"></span>
                    </button>
                </div>
            </template>
            <div v-if="currentEntry" class="flex flex-col h-full">
                <div class="p-4 overflow-y-auto flex-grow">
                    <div v-if="isEditMode">
                        <label class="font-semibold text-gray-700">Username:</label>
                        <input v-model="currentEntry.username" class="w-full px-3 py-1 mb-4 input-complete" />

                        <label class="font-semibold text-gray-700">Display Name:</label>
                        <input v-model="currentEntry.display_name" class="w-full px-3 py-1 mb-4 input-complete" />

                        <label class="font-semibold text-gray-700">Language:</label>
                        <input v-model="currentEntry.language" class="w-full px-3 py-1 mb-4 input-complete" />
                    </div>
                    <div v-else>
                        <label class="font-semibold text-gray-700">Username:</label>
                        <input v-model="currentEntry.username" class="w-full px-3 py-1 mb-4 input-complete" disabled/>

                        <label class="font-semibold text-gray-700">Display Name:</label>
                        <input v-model="currentEntry.display_name" class="w-full px-3 py-1 mb-4 input-complete" disabled/>

                        <label class="font-semibold text-gray-700">Language:</label>
                        <input v-model="currentEntry.language" class="w-full px-3 py-1 mb-4 input-complete" disabled/>
                    </div>
                    <div class="mb-4 relative">
                        <label class="block text-gray-700 font-semibold">Status:</label>
                        <div v-if="isEditMode">
                            <select v-model="currentEntry.status_id" class="w-full input-complete px-3 py-1">
                                <option :value="true">Activate</option>
                                <option :value="false">Installed</option>
                            </select>
                        </div>
                        <div v-else>
                            <span class="text-gray-600">{{ currentEntry.status_id ? "Active" : "Inactive" }}</span>
                        </div>
                    </div>
                </div>
                <div class="flex justify-end p-4 border-t">
                    <button class="btn-secondary px-3 py-1 mr-2" @click="closeDrawer">Close</button>
                    <button class="btn-primary px-3 py-1" v-if="isEditMode">Save</button>
                </div>
            </div>
        </Drawer>
    </div>
</template>
