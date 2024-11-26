<script setup lang="ts">
import { computed, onMounted, ref, watch, type Ref } from 'vue';
import { Drawer, newGuid, requiredValidator, emailValidator, passwordValidator, useValidation } from 'c3k-library';
import type { IUser, IStatus } from '@/models';
import { StatusService } from '@/services';
import { useRoleUserStore } from '@/stores';

const repo: StatusService = new StatusService();
const statusList: Ref<Array<IStatus>> = ref([]);

const store = useRoleUserStore();
const open = computed(() => store.store.Open && store.store.OperationType !== 'delete');
const title = computed(() => store.store.Title);
const addUpdate = computed(() => store.store.OperationType === 'add' || store.store.OperationType === 'edit');
const entity: Ref<IUser | null> = ref(null);

watch(() => store.store.OperationType,
    () => entity.value = store.store.OperationType === 'add' ?
        { UserId: newGuid(), Username: '', DisplayName: '', Language: 'en-US', Password: '', Salt: '', StatusId: '' } :
        store.entity);

const { validationErrors, validateForm } = useValidation();

const execute = (action: boolean) => {
    if (action && entity.value && !validateForm(entity.value, [
        { field: 'Username', rules: [requiredValidator, emailValidator] },
        { field: 'DisplayName', rules: [requiredValidator] },
        ...(store.store.OperationType === 'add' ? [{ field: 'Password', rules: [requiredValidator, passwordValidator] }] : []),
    ])) {
        return;
    }

    action === false ? store.execute('get') : store.execute(store.store.OperationType as 'add' | 'delete' | 'get' | 'edit', entity.value as IUser);
}
onMounted(() => repo.GetAll().then(res => statusList.value = (res as any).data))
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
                <div class="space-y-2">
                    <div class="flex flex-col">
                        <label for="username" class="font-semibold text-gray-700 mb-1">User Name:</label>
                        <input id="username" v-model="entity.Username" class="input-bottom pl-1 mb-2 "
                            placeholder="Enter username" required />
                        <span v-if="validationErrors.Username" class="text-red-500 text-sm">{{ validationErrors.Username
                            }}</span>
                    </div>

                    <div class="flex flex-col">
                        <label for="displayname" class="font-semibold text-gray-700 mb-1">Display Name:</label>
                        <input id="displayname" v-model="entity.DisplayName" class="input-bottom pl-1 mb-2 "
                            placeholder="Enter display name" required />
                        <span v-if="validationErrors.DisplayName" class="text-red-500 text-sm">{{
                            validationErrors.DisplayName
                            }}</span>
                    </div>
                    <div v-if="title.toLowerCase().startsWith('add')" class="flex flex-col">
                        <label for="password" class="font-semibold text-gray-700 mb-1">Password:</label>
                        <input id="password" v-model="entity.Password" class="input-bottom pl-1 mb-2 "
                            placeholder="Enter password" :rules="[requiredValidator, passwordValidator]" required />
                        <span v-if="validationErrors.Password" class="text-red-500 text-sm">{{ validationErrors.Password
                            }}</span>
                    </div>
                </div>
                <div class="py-2 relative">
                    <label class="block text-gray-700 font-semibold" for="status">Status:</label>
                    <div class="relative">
                        <select v-model="entity.StatusId" id="status"
                            class="input-bottom pl-2 w-full bg-white appearance-none pr-10">
                            <option v-for="item in statusList" :key="item.StatusId" :value="item.StatusId">{{
                                item.FullName }}
                            </option>
                        </select>
                        <span
                            class="icon-[iconamoon--arrow-down-2] absolute right-3 top-3 text-gray-600 pointer-events-none"></span>
                    </div>
                </div>
            </div>

            <div class="flex justify-end p-4 border-t">
                <button class="btn-secondary px-3 py-1 mr-2" @click="execute(false)">Close</button>
                <button class="btn-primary px-3 py-1" @click="execute(true)" v-if="addUpdate">
                    <span v-if="store.store.OperationType === 'add'">Add Record</span>
                    <span v-else>Update Record</span>
                </button>
            </div>
        </div>
    </Drawer>
</template>