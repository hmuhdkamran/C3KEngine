<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref, type Ref } from 'vue';

import { Drawer, newGuid, PubSub } from 'c3k-library';
import type { IUser, IStatus, RecordPubSub } from '@/models';
import { StatusService } from '@/services';

const repo: StatusService = new StatusService();
const statusList: Ref<Array<IStatus>> = ref([]);

interface Emits {
    (e: 'execute', action: RecordPubSub): void;
};

const emit = defineEmits<Emits>();

const entity: Ref<IUser | null> = ref(null);
const title: Ref<string> = ref('');
const open: Ref<boolean> = ref(false);

const drawerAction = (action: RecordPubSub) => {
    open.value = action.open;
    entity.value = open.value && action.entity === null ? { UserId: newGuid(), Username: '', DisplayName: '', Language: 'en-US', Password: '', Salt: '', StatusId: '' } : action.entity;
    title.value = action.title;
}
PubSub.subscribe<RecordPubSub>("ToggleDrawer", drawerAction);

const execute = (action: boolean) => {
    const record: RecordPubSub = { open: open.value, entity: action ? entity.value : null, title: title.value };
    emit('execute', record);
}

onMounted(() => repo.GetAll().then((response: any) => statusList.value = response.data));
onUnmounted(() => PubSub.unsubscribe<RecordPubSub>("ToggleDrawer", drawerAction));

</script>
<template>
    <Drawer :isOpen="open" :title="title" position="right" size="w-1/3" class="bg-black bg-opacity-50">
        <template #header>
            <div
                class="w-full flex justify-between items-center p-4 bg-gradient-to-r from-indigo-500 to-violet-600 border-b">
                <h2 class="text-lg font-semibold text-gray-100">{{ title }}</h2>
            </div>
        </template>
        <div class="flex flex-col h-full">
            <div v-if="entity" class="p-6 overflow-y-auto flex-grow">
                <div>
                    <span class="font-semibold text-gray-700">User Name:</span>
                    <input v-model="entity.Username" placeholder="Enter username"
                        class="input-bottom pl-2 mb-4 w-full bg-white" required />

                    <span class="font-semibold text-gray-700">Display Name:</span>
                    <input v-model="entity.DisplayName" placeholder="Enter display name"
                        class="input-bottom pl-2 mb-4 w-full bg-white" required />

                    <span class="font-semibold text-gray-700" v-if="title.toLowerCase().startsWith('add')">Password:</span>
                    <input v-model="entity.Password" placeholder="Enter password"
                        class="input-bottom pl-2 mb-4 w-full bg-white" required  v-if="title.toLowerCase().startsWith('add')" />
                </div>
                <div class="mb-4 relative">
                    <label class="block text-gray-700 font-semibold" for="status">Status:</label>
                    <div class="relative">
                        <select v-model="entity.StatusId" id="status" class="input-bottom pl-2 w-full bg-white">
                            <option v-for="item in statusList" :value="item.StatusId">{{ item.FullName }}</option>
                        </select>
                        <span v-if="open"
                            class="icon-[iconamoon--arrow-up-2] absolute right-3 top-3 text-gray-600"></span>
                        <span v-else class="icon-[iconamoon--arrow-down-2] absolute right-3 top-3 text-gray-600"></span>
                    </div>
                </div>
            </div>

            <div class="flex justify-end p-4 border-t">
                <button class="btn-secondary px-3 py-1 mr-2" @click="execute(false)">Close</button>
                <button class="btn-primary px-3 py-1" @click="execute(true)"
                    v-if="!title.toLowerCase().startsWith('view')">
                    <span v-if="title.toLowerCase().startsWith('add')">Add Record</span>
                    <span v-else>Update Record</span>
                </button>
            </div>
        </div>
    </Drawer>
</template>