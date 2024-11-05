<script setup lang="ts">
import { ref, defineProps, defineEmits } from 'vue';
import { Drawer } from 'c3k-library';
import type { IUser } from '@/models';

interface Props {
    isDrawerVisible: boolean;
    isEditMode: boolean;
    currentEntry: IUser | null;
}

interface Emits {
  (e: 'close'): void;
  (e: 'save'): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

const isDropdownOpen = ref(false);

const closeDrawer = () => {
    emit('close');
};

const onSave = () => {
    emit('save');
};
</script>

<template>
    <Drawer :isOpen="props.isDrawerVisible" :title="props.isEditMode ? 'Edit Entry' : 'View Entry'" position="right" size="w-1/4">
        <template #header>
            <div class="w-full flex justify-between items-center p-4 bg-gray-100 border-b">
                <h2 class="text-lg font-semibold text-gray-800">{{ props.isEditMode ? "Edit" : "View" }}</h2>
                <button @click="closeDrawer" class="text-gray-400 hover:text-gray-600 focus:outline-none">
                    <span class="icon-[fluent--dismiss-20-filled] h-4 w-4"></span>
                </button>
            </div>
        </template>
        <div class="flex flex-col h-full">
            <div v-if="props.currentEntry" class="p-4 overflow-y-auto flex-grow">
                <div v-if="props.isEditMode">
                    <span class="font-semibold text-gray-700">User Name:</span>
                    <input v-model="props.currentEntry.Username" class="w-full px-3 py-1 mb-4 input-complete" required />

                    <span class="font-semibold text-gray-700">Display Name:</span>
                    <input v-model="props.currentEntry.DisplayName" class="w-full px-3 py-1 mb-4 input-complete" required />
                </div>
                <div v-else>
                    <span class="font-semibold text-gray-700">User Name:</span>
                    <input v-model="props.currentEntry.Username" class="w-full px-3 py-1 mb-4 input-complete" disabled />

                    <span class="font-semibold text-gray-700">Display Name:</span>
                    <input v-model="props.currentEntry.DisplayName" class="w-full px-3 py-1 mb-4 input-complete" disabled />
                </div>
                <div class="mb-4 relative">
                    <label class="block text-gray-700 font-semibold" for="status">Status:</label>
                    <div v-if="isEditMode" class="relative">
                        <select v-model="props.currentEntry.StatusId" id="status" class="w-full input-complete px-3 py-1"
                            @focus="isDropdownOpen = true" @blur="isDropdownOpen = false">
                            <option value="Activate">Active</option>
                            <option value="Installed">Inactive</option>
                        </select>
                        <span v-if="isDropdownOpen"
                            class="icon-[iconamoon--arrow-up-2] absolute right-3 top-3 text-gray-600"></span>
                        <span v-else class="icon-[iconamoon--arrow-down-2] absolute right-3 top-3 text-gray-600"></span>
                    </div>
                    <div v-else>
                        <span class="text-gray-600">{{ props.currentEntry.StatusId === "Active" ? "Active" : "Inactive"
                            }}</span>
                    </div>
                </div>
            </div>
            <div class="flex justify-end p-4 border-t">
                <button class="btn-secondary px-3 py-1 mr-2" @click="closeDrawer">Close</button>
                <button class="btn-primary px-3 py-1" v-if="props.isEditMode" @click="onSave">Save</button>
            </div>
        </div>
    </Drawer>
</template>