<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { DataTable, Pagination } from 'c3-library';
import { selectColor } from '@/stores/colorPalette';
import { UserService } from '@/services/user-service';
import type { User } from '@/models/user';
import { useSystemStore } from 'c3-library';

const users = ref<User[]>([]);
const userService = new UserService();
const store = useSystemStore();
const currentUser = ref<User>({ UserId: '', Username: '', DisplayName: '', Language: '', Password: '', Salt: '', StatusId: '' });
const isEditMode = ref(false);
const currentPage = ref(1);
const itemsPerPage = 10;

const columns = [
  { key: 'DisplayName', label: 'Name', sort: true },
  { key: 'Username', label: 'Email', sort: true },
  { key: 'Language', label: 'Language', sort: true },
  { key: 'actions', label: 'Actions', sort: false, class: 'text-center' },
];

const fetchUsersData = async () => {
  try {
    users.value = await userService.getAll();
  } catch (error: any) {
    console.error('Error fetching users:', error);
    if (error.message === 'Authentication token not found' || error.message === 'Invalid token, please login again') {
      alert('Your session has expired. Please login again.');
      store.updateUser(null);
      window.location.href = '/login';
    }
  }
};

onMounted(() => {
  fetchUsersData();
});

const openForm = (user: User | null = null) => {
  if (user) {
    currentUser.value = { ...user };
    isEditMode.value = true;
  } else {
    currentUser.value = { UserId: '', Username: '', DisplayName: '', Language: '', Password: '', Salt: '', StatusId: '' };
    isEditMode.value = false;
  }
};

const saveUser = async () => {
  if (isEditMode.value) {
    const result = await userService.update(currentUser.value);
    if (result) {
      alert('User updated successfully');
    }
  } else {
    const result = await userService.add(currentUser.value);
    if (result) {
      alert('User created successfully');
    }
  }
  openForm();
};

const deleteUser = async (id: string) => {
  if (confirm('Are you sure you want to delete this user?')) {
    const result = await userService.delete(id);
    if (result) {
      alert('User deleted successfully');
      fetchUsersData();
    }
  }
};

const totalPages = computed(() => {
  return Math.ceil(users.value.length / itemsPerPage);
});

const paginatedUsers = computed(() => {
  const start = (currentPage.value - 1) * itemsPerPage;
  const end = start + itemsPerPage;
  return users.value.slice(start, end);
});

const setPage = (page: number) => {
  if (page > 0 && page <= totalPages.value) {
    currentPage.value = page;
  }
};
</script>

<template>
  <div class="user-management relative">
    <h1 class="text-lg mb-5">User Management</h1>

    <div v-if="currentUser.UserId !== ''" class="p-4 bg-white rounded-sm shadow-sm">
      <h2 class="text-lg font-semibold mb-6">{{ isEditMode ? 'Edit User' : 'Add User' }}</h2>
      <form @submit.prevent="saveUser" class="grid grid-cols-1 sm:grid-cols-2 gap-4">
        <div class="mb-3">
          <label for="DisplayName" class="text-sm font-medium text-gray-700">Name:</label>
          <input type="text" id="DisplayName" v-model="currentUser.DisplayName" required
            class="mt-1 block w-full p-1 rounded-sm shadow-sm focus:ring-[var(--ring-color)] focus:border-[var(--border-color)]"
            :style="{ '--ring-color': selectColor(), '--border-color': selectColor() }" />
        </div>
        <div class="mb-3">
          <label for="Username" class="text-sm font-medium text-gray-700">Email:</label>
          <input type="email" id="Username" v-model="currentUser.Username" required
            class="mt-1 block w-full p-1 rounded-sm shadow-sm focus:ring-[var(--ring-color)] focus:border-[var(--border-color)]"
            :style="{ '--ring-color': selectColor(), '--border-color': selectColor() }" />
        </div>
        <div class="mb-3">
          <label for="Language" class="text-sm font-medium text-gray-700">Language:</label>
          <input type="text" id="Language" v-model="currentUser.Language" required
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
      <DataTable :data="paginatedUsers" :columns="columns">
        <template #actions="{ row }">
          <div class="flex justify-center space-x-1">
            <button class="w-6 h-6 cursor-pointer shadow-md flex items-center justify-center bg-white rounded-full"
              @click="openForm(row)" :style="{ color: selectColor() }">
              <span class="fas fa-pen"></span>
            </button>
            <button class="w-6 h-6 cursor-pointer shadow-md flex items-center justify-center bg-white rounded-full"
              @click="deleteUser(row.UserId)" :style="{ color: selectColor() }">
              <span class="fas fa-trash-can"></span>
            </button>
          </div>
        </template>
      </DataTable>
      <div class="absolute bottom-4 right-4">
        <Pagination :totalPages="totalPages" :currentPage="currentPage" @page-changed="setPage" />
      </div>
    </div>
  </div>
</template>

<style scoped>
.user-management {
  padding: 20px;
  font-family: 'Arial', sans-serif;
  margin: 0 auto;
}
.user-management {
  padding: 20px;
  font-family: 'Arial', sans-serif;
  margin: 0 auto;
}
</style>

<route lang="yaml">
  meta:
    layout: auth
    redirectIfLoggedIn: true
</route>