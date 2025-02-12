<script setup lang="ts">
import { ref, onMounted, computed, watch } from 'vue';
import { useThemePaletteStore } from 'c3-library';
import { DataTable, newGuid, Pagination, RepositoryService } from 'c3-library';
import type { IUser } from '@/models';
import { setFormOpen, formStatus } from '@/stores/edit-form';

import DialogBox from './DialogBox.vue';
import { useRoleUserStore } from '@/stores/roles/user-store';

const store = useRoleUserStore();
const color = useThemePaletteStore();

const service = new RepositoryService<IUser>('auth/role/users');
const entities = ref<IUser[]>([]);
const entity = ref<IUser>({ UserId: newGuid(), Username: '', DisplayName: '', Language: '', Password: '', Salt: '', StatusId: '' });

const isEditMode = ref(false);
const currentPage = ref(1);
const itemsPerPage = 10;

const columns = [
  { key: 'DisplayName', label: 'Name', sort: true },
  { key: 'Username', label: 'Email', sort: true },
  { key: 'Language', label: 'Language', sort: true },
  { key: 'actions', label: 'Actions', sort: false, class: 'text-center' },
];

const fetchUsers = () => {
  setFormOpen(false);
  service.getAll().then(result => entities.value = result);
};

watch(formStatus, (newValue) => {
  if (!newValue) {
    entity.value = { UserId: newGuid(), Username: '', DisplayName: '', Language: '', Password: '', Salt: '', StatusId: '' };
    isEditMode.value = false;
  }
})

const openForm = (user: IUser | null = null) => {
  if (user) {
    entity.value = { ...user };
    isEditMode.value = true;
    setFormOpen(true);
  }
};

const saveUser = () => {
  const saveOperation = isEditMode.value
    ? service.update(entity.value)
    : service.add(entity.value);

  saveOperation.finally(() => {
    setFormOpen(false);
    fetchUsers();
  });
};

const deleteUser = (user: IUser) => {
  if (confirm('Are you sure you want to delete this user?')) {
    store.deleteItem(user)
  }
};

const totalPages = computed(() => {
  return Math.ceil(entities.value.length / itemsPerPage);
});

const paginatedUsers = computed(() => {
  const start = (currentPage.value - 1) * itemsPerPage;
  const end = start + itemsPerPage;
  return entities.value.slice(start, end);
});

const setPage = (page: number) => {
  if (page > 0 && page <= totalPages.value) {
    currentPage.value = page;
  }
};

onMounted(() => {
  store.getItems();
});
</script>

<template>
  <div class="relative">
    <DialogBox :show="formStatus" @close="setFormOpen(false)">
      <template #header>
        <h2 class="text-lg font-semibold">{{ isEditMode ? 'Edit User' : 'Add User' }}</h2>
      </template>

      <form @submit.prevent="saveUser" class="grid grid-cols-1 sm:grid-cols-2 gap-4">
        <div class="mb-3">
          <label for="DisplayName" class="text-sm font-medium text-gray-700">Name:</label>
          <input type="text" id="DisplayName" v-model="entity.DisplayName" required
            class="mt-1 block w-full p-1 rounded-sm shadow-sm focus:ring-[var(--ring-color)] focus:border-[var(--border-color)]"
            :style="{ '--ring-color': color.selectedColor, '--border-color': color.selectedColor }" />
        </div>
        <div class="mb-3">
          <label for="Username" class="text-sm font-medium text-gray-700">Email:</label>
          <input type="email" id="Username" v-model="entity.Username" required
            class="mt-1 block w-full p-1 rounded-sm shadow-sm focus:ring-[var(--ring-color)] focus:border-[var(--border-color)]"
            :style="{ '--ring-color': color.selectedColor, '--border-color': color.selectedColor }" />
        </div>
        <div class="mb-3">
          <label for="Language" class="text-sm font-medium text-gray-700">Language:</label>
          <select id="Language" v-model="entity.Language" required
            class="mt-1 block w-full p-1 rounded-sm shadow-sm focus:ring-[var(--ring-color)] focus:border-[var(--border-color)]"
            :style="{ '--ring-color': color.selectedColor, '--border-color': color.selectedColor }">
            <option value="English">English</option>
            <option value="Urdu">Urdu</option>
          </select>
        </div>
        <div class="mb-3" v-if="!isEditMode">
          <label for="Password" class="text-sm font-medium text-gray-700">Password:</label>
          <input type="password" id="Password" v-model="entity.Password" required
            class="mt-1 block w-full p-1 rounded-sm shadow-sm focus:ring-[var(--ring-color)] focus:border-[var(--border-color)]"
            :style="{ '--ring-color': color.selectedColor, '--border-color': color.selectedColor }" />
        </div>
        <div class="mb-3" v-if="!isEditMode">
          <label for="ConfirmPassword" class="text-sm font-medium text-gray-700">Confirm Password:</label>
          <input type="password" id="ConfirmPassword" required
            class="mt-1 block w-full p-1 rounded-sm shadow-sm focus:ring-[var(--ring-color)] focus:border-[var(--border-color)]"
            :style="{ '--ring-color': color.selectedColor, '--border-color': color.selectedColor }" />
        </div>
        <div class="mb-3" v-if="!isEditMode">
          <label for="UserRole" class="text-sm font-medium text-gray-700">User Role:</label>
          <select id="UserRole" v-model="entity.StatusId" required
            class="mt-1 block w-full p-1 rounded-sm shadow-sm focus:ring-[var(--ring-color)] focus:border-[var(--border-color)]"
            :style="{ '--ring-color': color.selectedColor, '--border-color': color.selectedColor }">
            <option value="Admin">Admin</option>
            <option value="User">User</option>
            <option value="Guest">Guest</option>
          </select>
        </div>
        <div class="mb-3" v-if="isEditMode">
          <label for="Status" class="text-sm font-medium text-gray-700">Status:</label>
          <div class="mt-1">
            <button class="inline-flex items-center" :class="{
              'text-green-700': entity.StatusId === 'Active',
              'text-red-600': entity.StatusId !== 'Active'
            }">
              <span v-if="entity.StatusId === 'Active'" class="fa-solid fa-toggle-on fa-xl"></span>
              <span v-else class="fa-solid fa-toggle-off fa-xl"></span>
              <span class="ml-2">{{ entity.StatusId === 'Active' ? 'Active' : 'Inactive' }}</span>
            </button>
          </div>
        </div>
      </form>
      <template #footer>
        <div class="flex justify-end gap-3 col-span-2">
          <button type="button" class="px-3 py-1.5 bg-gray-200 rounded-sm hover:bg-gray-300 transition"
            @click="setFormOpen(false)">Cancel</button>
          <button type="submit"
            class="px-3 py-1.5 text-white rounded-sm hover:bg-indigo-600 focus:ring-2 focus:ring-indigo-500"
            :style="{ backgroundColor: color.selectedColor }">Save</button>
        </div>
      </template>
    </DialogBox>

    <div>
      <DataTable :data="store.items" :columns="columns">
        <template #actions="{ row }">
          <div class="flex justify-center space-x-1">
            <button class="w-6 h-6 cursor-pointer shadow-md flex items-center justify-center bg-white rounded-full"
              @click="openForm(row as any)" :style="{ color: color.selectedColor }">
              <span class="fas fa-pen"></span>
            </button>
            <button class="w-6 h-6 cursor-pointer shadow-md flex items-center justify-center bg-white rounded-full"
              @click="deleteUser(row as IUser)" :style="{ color: color.selectedColor }">
              <span class="fas fa-trash-can"></span>
            </button>
          </div>
        </template>
      </DataTable>

      <!-- Pagination positioned at the bottom right -->
      <div class="absolute bottom-4 right-4">
        <Pagination :totalPages="totalPages" :currentPage="currentPage" @page-changed="setPage" />
      </div>
    </div>
  </div>
</template>

<style scoped></style>

<route lang="yaml">
  meta:
    layout: auth
    redirectIfLoggedIn: true
  </route>