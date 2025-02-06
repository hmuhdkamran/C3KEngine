<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { selectColor } from '@/stores/colorPalette';
import { DataTable, newGuid, Pagination, RepositoryService } from 'c3-library';

interface IUser {
  UserId: string;
  Username: string;
  DisplayName: string;
  Language: string;
  Password: string;
  Salt: string;
  StatusId: string;
}

const service = new RepositoryService<IUser>('auth/role/users');
const entities = ref<IUser[]>([]);
const entity = ref<IUser>({ UserId: newGuid(), Username: '', DisplayName: '', Language: '', Password: '', Salt: '', StatusId: '' });

const isEditMode = ref(false);
const currentPage = ref(1);
const itemsPerPage = 10;

const columns = [
  { key: 'DisplayName', label: 'Name', sort: true },
  { key: 'Username', label: 'Email', sort: true },
  { key: 'Language', label: 'Role', sort: true },
  { key: 'actions', label: 'Actions', sort: false, class: 'text-center' },
];

const fetchUsers = () => {
  service.getAll().then(result => entities.value = result);
};

const openForm = (user: IUser | null = null) => {
  if (user) {
    entity.value = { ...user };
    isEditMode.value = true;
  } else {
    entity.value = { UserId: newGuid(), Username: '', DisplayName: '', Language: '', Password: '', Salt: '', StatusId: '' };
    isEditMode.value = false;
  }
};

const saveUser = () => {
  if (isEditMode.value) {    
    service.update(entity.value).then(() => fetchUsers());
  } else {
    service.add(entity.value).then(() =>fetchUsers());
  }

  openForm();
};

const deleteUser = (userId: string) => {
  if (confirm('Are you sure you want to delete this user?')) {
    entities.value = entities.value.filter((user) => user.UserId !== userId);
    alert('User deleted successfully');
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
  fetchUsers();
});
</script>

<template>
  <div class="relative">
    <div v-if="isEditMode" class="p-4 bg-white rounded-sm shadow-sm">
      <h2 class="text-lg font-semibold mb-6">{{ isEditMode ? 'Edit User' : 'Add User' }}</h2>
      <form @submit.prevent="saveUser" class="grid grid-cols-1 sm:grid-cols-2 gap-4">
        <div class="mb-3">
          <label for="name" class="text-sm font-medium text-gray-700">Name:</label>
          <input type="text" id="name" v-model="entity.DisplayName" required
            class="mt-1 block w-full p-1 rounded-sm shadow-sm focus:ring-[var(--ring-color)] focus:border-[var(--border-color)]"
            :style="{ '--ring-color': selectColor(), '--border-color': selectColor() }" />
        </div>
        <div class="mb-3">
          <label for="email" class="text-sm font-medium text-gray-700">Email:</label>
          <input type="email" id="email" v-model="entity.Username" required
            class="mt-1 block w-full p-1 rounded-sm shadow-sm focus:ring-[var(--ring-color)] focus:border-[var(--border-color)]"
            :style="{ '--ring-color': selectColor(), '--border-color': selectColor() }" />
        </div>
        <div class="mb-3">
          <label for="role" class="text-sm font-medium text-gray-700">Language:</label>
          <input type="text" id="role" v-model="entity.Language" required
            class="mt-1 block w-full p-1 rounded-sm shadow-sm focus:ring-[var(--ring-color)] focus:border-[var(--border-color)]"
            :style="{ '--ring-color': selectColor(), '--border-color': selectColor() }" />
        </div>
        <div class="flex justify-end gap-3 mt-4 col-span-2">
          <button type="button" class="px-3 py-1.5 bg-gray-200 rounded-sm hover:bg-gray-300 transition"
            @click="openForm()">Cancel</button>
          <button type="submit"
            class="px-3 py-1.5 text-white rounded-sm hover:bg-indigo-600 focus:ring-2 focus:ring-indigo-500"
            :style="{ backgroundColor: selectColor() }">Save</button>
        </div>
      </form>
    </div>

    <div v-else>
      <DataTable :data="entities" :columns="columns">
        <template #actions="{ row }">
          <div class="flex justify-center space-x-1">
            <button class="w-6 h-6 cursor-pointer shadow-md flex items-center justify-center bg-white rounded-full"
              @click="openForm(row as any)" :style="{ color: selectColor() }">
              <span class="fas fa-pen"></span>
            </button>
            <button class="w-6 h-6 cursor-pointer shadow-md flex items-center justify-center bg-white rounded-full"
              @click="deleteUser(row.id)" :style="{ color: selectColor() }">
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

<style scoped>

</style>

<route lang="yaml">
  meta:
    layout: auth
    redirectIfLoggedIn: true
  </route>