<script setup lang="ts">
import { setFormOpen, formStatus } from '@/stores/edit-form';
import type { IUser } from '@/models';
import { DialogBox, newGuid, useSystemStore, type ISignupUsers } from 'c3-library';
import { useRoleUserStore, useRoleRolesStore, useRoleUserRoleMapStore, useSetupStatusStore } from '@/stores';
import { computed, ref, watch, type Ref } from 'vue';
import { AuthenticationService } from '@/services/authentication-service';

const color = useSystemStore();
const store = useRoleUserStore();
const roleStore = useRoleRolesStore();
const userRoleStore = useRoleUserRoleMapStore();
const statusStore = useSetupStatusStore();
const aut_repo = new AuthenticationService();

const userItem = computed(() => store.item || ({} as IUser));
const selectedRoles = ref<string[]>([]);

watch(() => userRoleStore.items, () => selectedRoles.value = userRoleStore.items.map(i => i.RoleId), { deep: true });

const toggleRole = (roleId: string) => {
    const index = selectedRoles.value.indexOf(roleId);
    if (index === -1) {
        selectedRoles.value.push(roleId);
    } else {
        selectedRoles.value.splice(index, 1);
    }
};

function saveUser() {
    if (!store.shouldUpdate) {
        let entity: ISignupUsers = {
            user_id: newGuid(),
            username: userItem.value.Username as string,
            display_name: userItem.value.DisplayName as string,
            language: userItem.value.Language as string,
            password: userItem.value.Password as string,
            status_id: statusStore.items[0].StatusId as string,
            roles: selectedRoles.value,
        };
        console.log(JSON.stringify(entity));
        aut_repo.signup(entity).then(() => {
            store.getItems();
            console.log('User saved successfully');
            store.shouldUpdate = false;
            setFormOpen(false);
        });
    } else {
        store.createOrUpdateItem(store.item as IUser)
            .then(() => {
                console.log('User saved successfully');
                store.shouldUpdate = false;
                setFormOpen(false);
            })
            .catch((e) => {
                console.error('Error saving user:', e);
            });
    }
}

function close() {
    store.shouldUpdate = false;
    setFormOpen(false)
}

</script>

<template>
    <DialogBox :show="formStatus" :showClose="false" @close="close()">
        <template #header>
            <h2 class="text-lg font-semibold">{{ store.shouldUpdate ? 'User Edit Form' : 'Add User Form' }}</h2>
            <p class="text-sm text-gray-500">{{ store.shouldUpdate ? 'Edit Record' : 'Add Record' }}</p>
        </template>

        <form class="grid grid-cols-1 sm:grid-cols-2 gap-4">
            <div class="mb-3">
                <label for="DisplayName" class="text-sm font-medium text-gray-700">Name:</label>
                <input type="text" id="DisplayName" v-model="userItem.DisplayName" required
                    class="mt-1 block w-full p-1 rounded-sm shadow-sm focus:ring-[var(--ring-color)] focus:border-[var(--border-color)]"
                    :style="{ '--ring-color': color.application.primaryColor, '--border-color': color.application.primaryColor }" />
            </div>
            <div class="mb-3">
                <label for="Username" class="text-sm font-medium text-gray-700">Email:</label>
                <input type="email" id="Username" v-model="userItem.Username" required
                    class="mt-1 block w-full p-1 rounded-sm shadow-sm focus:ring-[var(--ring-color)] focus:border-[var(--border-color)]"
                    :style="{ '--ring-color': color.application.primaryColor, '--border-color': color.application.primaryColor }" />
            </div>
            <div class="mb-3">
                <label for="Language" class="text-sm font-medium text-gray-700">Language:</label>
                <select id="Language" v-model="userItem.Language" required
                    class="mt-1 block w-full p-1 rounded-sm shadow-sm focus:ring-[var(--ring-color)] focus:border-[var(--border-color)]"
                    :style="{ '--ring-color': color.application.primaryColor, '--border-color': color.application.primaryColor }">
                    <option value="English">English</option>
                    <option value="Urdu">Urdu</option>
                </select>
            </div>
            <div class="mb-3" v-if="!store.shouldUpdate">
                <label for="Password" class="text-sm font-medium text-gray-700">Password:</label>
                <input type="password" id="Password" v-model="userItem.Password" required
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
                <select id="UserRole" v-model="roleStore.getItems" required
                    class="mt-1 block w-full p-1 rounded-sm shadow-sm focus:ring-[var(--ring-color)] focus:border-[var(--border-color)]"
                    :style="{ '--ring-color': color.application.primaryColor, '--border-color': color.application.primaryColor }">
                    <option v-for="item in roleStore.items" :value="item.RoleId">{{ item.FullName }}</option>
                </select>
            </div>
            <div class="mb-3" v-if="store.shouldUpdate">
                <label for="Status" class="text-sm font-medium text-gray-700">Status:</label>
                <select id="Language" v-model="userItem.StatusId" required
                    class="mt-1 block w-full p-1 rounded-sm shadow-sm focus:ring-[var(--ring-color)] focus:border-[var(--border-color)]"
                    :style="{ '--ring-color': color.application.primaryColor, '--border-color': color.application.primaryColor }">
                    <option v-for="item in statusStore.items" :value="item.StatusId">{{ item.FullName }}</option>
                </select>
            </div>

            <div class="">
                <ul role="listbox" aria-label="role lists">
                    <li v-for="item in roleStore.items" :key="item.RoleId" tabindex="-1" role="option">
                        <label :for="item.RoleId" class="flex items-center cursor-pointer my-1">
                            <div class="relative">
                                <input type="checkbox" :id="item.RoleId" :checked="selectedRoles.includes(item.RoleId)"
                                    @change="toggleRole(item.RoleId)" class="sr-only">
                                <div class="block bg-gray-600 w-14 h-6 rounded-sm"></div>
                                <div class="dot absolute left-1 top-1 bg-white w-6 h-4 rounded-sm transition"></div>
                            </div>
                            <div class="ml-3 text-gray-700 font-medium">
                                {{ item.FullName }}
                            </div>
                        </label>
                    </li>
                </ul>
            </div>
        </form>
        <template #footer>
            <div class="flex justify-end gap-3 col-span-2">
                <button type="button"
                    class="px-3 py-1.5 bg-gray-200 rounded-sm hover:bg-gray-300 transition cursor-pointer"
                    @click="close()">Cancel</button>
                <button type="submit" @click="saveUser" class="px-3 py-1.5 text-white rounded-sm cursor-pointer"
                    :style="{ backgroundColor: color.application.primaryColor }">Save</button>
            </div>
        </template>
    </DialogBox>
</template>

<style scoped>
/* Toggle A */
input:checked~.dot {
    transform: translateX(100%);
    background-color: #48bb78;
}

/* Toggle B */
input:checked~.dot {
    transform: translateX(100%);
    background-color: #48bb78;
}
</style>