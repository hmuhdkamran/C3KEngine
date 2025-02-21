<script setup lang="ts">
import { defineProps, defineEmits, ref } from 'vue';
import { setFormOpen, formStatus } from '@/stores/edit-form';
import type { IUser } from '@/models';
import { newGuid, useSystemStore } from 'c3-library';
import { useRoleUserStore, useRoleRolesStore, useRoleUserRoleMapStore } from '@/stores';

import DialogBox from './DialogBox.vue';

const color = useSystemStore();
const store = useRoleUserStore();
const roleStore = useRoleRolesStore();
const userRoleStore = useRoleUserRoleMapStore();

function saveUser () {
    store.createOrUpdateItem(store.item)
    .then(()=> {
        store.shouldUpdate = false;
        setFormOpen(false);
    })
    .catch((e) => console.log(`Error Raised: ${e}`));
}

function close() {
    store.shouldUpdate = false;
    setFormOpen(false)
}

function role(value: string) {
    const find = userRoleStore.items.find(f => f.RoleId === value);
    if (find) {
        return true;
    } else {
        return false;
    }
}

</script>

<template>
    <DialogBox :show="formStatus" :showClose="false" @close="close()">
        <template #header>
            <h2 class="text-lg font-semibold">{{ store.shouldUpdate ? 'User Edit Form' : 'Add User Form' }}</h2>
            <p class="text-sm text-gray-500">{{ store.shouldUpdate ? 'Edit Record' : 'Add Record' }}</p>
        </template>

        <form @submit.prevent="saveUser" class="grid grid-cols-1 sm:grid-cols-2 gap-4">
            <div class="mb-3">
                <label for="DisplayName" class="text-sm font-medium text-gray-700">Name:</label>
                <input type="text" id="DisplayName" v-model="store.item.DisplayName" required
                    class="mt-1 block w-full p-1 rounded-sm shadow-sm focus:ring-[var(--ring-color)] focus:border-[var(--border-color)]"
                    :style="{ '--ring-color': color.application.primaryColor, '--border-color': color.application.primaryColor }" />
            </div>
            <div class="mb-3">
                <label for="Username" class="text-sm font-medium text-gray-700">Email:</label>
                <input type="email" id="Username" v-model="store.item.Username" required
                    class="mt-1 block w-full p-1 rounded-sm shadow-sm focus:ring-[var(--ring-color)] focus:border-[var(--border-color)]"
                    :style="{ '--ring-color': color.application.primaryColor, '--border-color': color.application.primaryColor }" />
            </div>
            <div class="mb-3">
                <label for="Language" class="text-sm font-medium text-gray-700">Language:</label>
                <select id="Language" v-model="store.item.Language" required
                    class="mt-1 block w-full p-1 rounded-sm shadow-sm focus:ring-[var(--ring-color)] focus:border-[var(--border-color)]"
                    :style="{ '--ring-color': color.application.primaryColor, '--border-color': color.application.primaryColor }">
                    <option value="English">English</option>
                    <option value="Urdu">Urdu</option>
                </select>
            </div>
            <div class="mb-3" v-if="!store.shouldUpdate">
                <label for="Password" class="text-sm font-medium text-gray-700">Password:</label>
                <input type="password" id="Password" v-model="store.item.Password" required
                    class="mt-1 block w-full p-1 rounded-sm shadow-sm focus:ring-[var(--ring-color)] focus:border-[var(--border-color)]"
                    :style="{ '--ring-color': color.application.primaryColor, '--border-color': color.application.primaryColor }" />
            </div>
            <div class="mb-3" v-if="!store.shouldUpdate">
                <label for="ConfirmPassword" class="text-sm font-medium text-gray-700">Confirm Password:</label>
                <input type="password" id="ConfirmPassword" required
                    class="mt-1 block w-full p-1 rounded-sm shadow-sm focus:ring-[var(--ring-color)] focus:border-[var(--border-color)]"
                    :style="{ '--ring-color': color.application.primaryColor, '--border-color': color.application.primaryColor }" />
            </div>
            <div class="mb-3" v-if="!store.shouldUpdate">
                <label for="UserRole" class="text-sm font-medium text-gray-700">User Role:</label>
                <select id="UserRole" v-model="store.item.StatusId" required
                    class="mt-1 block w-full p-1 rounded-sm shadow-sm focus:ring-[var(--ring-color)] focus:border-[var(--border-color)]"
                    :style="{ '--ring-color': color.application.primaryColor, '--border-color': color.application.primaryColor }">
                    <option value="Admin">Admin</option>
                    <option value="User">User</option>
                    <option value="Guest">Guest</option>
                </select>
            </div>
            <div class="mb-3" v-if="store.shouldUpdate">
                <label for="Status" class="text-sm font-medium text-gray-700">Status:</label>
                <div class="mt-1">
                    <button class="inline-flex items-center" :class="{
                        'text-green-700': store.item.StatusId === 'Active',
                        'text-red-600': store.item.StatusId !== 'Active'
                    }">
                        <span v-if="store.item.StatusId === 'Active'" class="fa-solid fa-toggle-on fa-xl"></span>
                        <span v-else class="fa-solid fa-toggle-off fa-xl"></span>
                        <span class="ml-2">{{ store.item.StatusId === 'Active' ? 'Active' : 'Inactive' }}</span>
                    </button>
                </div>
            </div>

            <div class="">
                <ul role="listbox" aria-label="role lists">
                    <li tabindex="-1" role="option" aria-checked="false" v-for="item in roleStore.items">
                        <input tabindex="-1" type="checkbox" :checked="role(item.RoleId)" /> {{ item.FullName }}
                    </li>
                </ul>
            </div>
        </form>
        <template #footer>
            <div class="flex justify-end gap-3 col-span-2">
                <button type="button" class="px-3 py-1.5 bg-gray-200 rounded-sm hover:bg-gray-300 transition"
                    @click="close()">Cancel</button>
                <button type="submit"
                    class="px-3 py-1.5 text-white rounded-sm hover:bg-indigo-600 focus:ring-2 focus:ring-indigo-500"
                    :style="{ backgroundColor: color.application.primaryColor }">Save</button>
            </div>
        </template>
    </DialogBox>
</template>