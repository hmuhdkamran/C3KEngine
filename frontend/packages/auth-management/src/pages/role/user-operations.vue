<script setup lang="ts">
import { ref, defineProps, defineEmits, watch, computed, onMounted, onBeforeUnmount } from 'vue';
import { Drawer } from 'c3k-library';
import type { IUser } from '@/models';
import { useApplicationEventStore } from '@/stores/application';

const store = useApplicationEventStore();

interface Props {
    title: string;
    currentEntry: IUser | null;
}

interface Emits {
    (e: 'close'): void;
    (e: 'save'): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

const currentEntry = ref<IUser | null>({
    UserId: '',
    Username: '',
    DisplayName: '',
    Language: '',
    Password: '',
    Salt: '',
    StatusId: '',
});

const isAdding = ref<boolean | null>(true);
const isDropdownOpen = ref(false);

const title = computed(() => props.title);

const isStatusDisabled = computed(() => {
    return title.value?.toLowerCase().startsWith('view');
});

watch(
    () => store.ToggleDrawer,
    () => {
        if (props.currentEntry === null) {
            isAdding.value = true;
            currentEntry.value = { UserId: '', Username: '', DisplayName: '', Language: '', Password: '', Salt: '', StatusId: '' };
        } else {
            isAdding.value = false;
            currentEntry.value = props.currentEntry;
        }

        if (title.value?.toLowerCase().startsWith('view')) {
            isAdding.value = null;
        }
    }
);

const drawerSize = ref("w-full");

const updateDrawerSize = () => {
    const width = window.innerWidth;
    if (width >= 1024) {
        drawerSize.value = "w-1/3";
    } else if (width >= 768) {
        drawerSize.value = "w-1/2";
    } else {
        drawerSize.value = "w-full";
    }
};

onMounted(() => {
    updateDrawerSize();
    window.addEventListener("resize", updateDrawerSize);
});

onBeforeUnmount(() => {
    window.removeEventListener("resize", updateDrawerSize);
});

const closeDrawer = () => emit('close');
const onSave = () => emit('save');
</script>

<template>
    <Drawer :isOpen="store.ToggleDrawer" :title="title" position="right" :size="drawerSize"
        class="bg-black bg-opacity-50">
        <template #header>
            <div
                class="w-full flex justify-between items-center p-4 bg-gradient-to-r from-indigo-500 to-violet-600 border-b">
                <h2 class="text-lg font-semibold text-gray-100">{{ title || "Add Record" }}</h2>
            </div>
        </template>
        <div class="flex flex-col h-full">
            <div v-if="currentEntry" class="p-6 overflow-y-auto flex-grow">
                <div>
                    <span class="font-semibold text-gray-700">User Name:</span>
                    <input v-model="currentEntry.Username" placeholder="Enter username"
                        class="w-full px-3 py-1 mb-4 input-complete" required />

                    <span class="font-semibold text-gray-700">Display Name:</span>
                    <input v-model="currentEntry.DisplayName" placeholder="Enter display name"
                        class="w-full px-3 py-1 mb-4 input-complete" required />
                </div>
                <div class="mb-4 relative">
                    <label class="block text-gray-700 font-semibold" for="status">Status:</label>
                    <div class="relative">
                        <select v-model="currentEntry.StatusId" id="status" class="w-full input-complete px-3 py-1"
                            @focus="isDropdownOpen = true" @blur="isDropdownOpen = false"
                            :disabled="isStatusDisabled && !isAdding">
                            <option value="Active">Active</option>
                            <option value="Inactive">Inactive</option>
                        </select>
                        <span v-if="isDropdownOpen"
                            class="icon-[iconamoon--arrow-up-2] absolute right-3 top-3 text-gray-600"></span>
                        <span v-else class="icon-[iconamoon--arrow-down-2] absolute right-3 top-3 text-gray-600"></span>
                    </div>
                </div>
            </div>
            <div class="flex justify-end p-4 border-t">
                <button class="btn-secondary px-3 py-1 mr-2" @click="closeDrawer">Close</button>
                <button class="btn-primary px-3 py-1" @click="onSave" v-if="isAdding != null">
                    <span v-if="isAdding">Add Record</span>
                    <span v-else>Update Record</span>
                </button>
            </div>
        </div>
    </Drawer>
</template>