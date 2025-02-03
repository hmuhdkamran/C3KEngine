<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { selectColor } from '@/stores/colorPalette';
import { DataTable, Pagination } from 'c3-library';

interface User {
  id: number;
  name: string;
  email: string;
  role: string;
}

const users = ref<User[]>([]);
const currentUser = ref<User>({ id: 0, name: '', email: '', role: '' });
const isEditMode = ref(false);
const currentPage = ref(1);
const itemsPerPage = 10;

const columns = [
  { key: 'name', label: 'Name', sort: true },
  { key: 'email', label: 'Email', sort: true },
  { key: 'role', label: 'Role', sort: true },
  { key: 'actions', label: 'Actions', sort: false, class: 'text-center' },
];

const generateDummyUsers = (count: number): User[] => {
  const firstNames = ['Ali', 'Ahmed', 'Fatima', 'Zainab', 'Usman', 'Hassan', 'Ayesha', 'Bilal', 'Sana', 'Amal'];
  const lastNames = ['Khan', 'Ali', 'Ahmed', 'Raza', 'Malik', 'Hussain', 'Shah', 'Akhtar', 'Qureshi', 'Baig'];
  const roles = ['Admin', 'Editor', 'Viewer', 'Manager'];

  return Array.from({ length: count }, (_, index) => ({
    id: index + 1,
    name: `${firstNames[Math.floor(Math.random() * firstNames.length)]} ${lastNames[Math.floor(Math.random() * lastNames.length)]}`,
    email: `user${index + 1}@example.com`,
    role: roles[Math.floor(Math.random() * roles.length)],
  }));
};

const fetchUsers = () => {
  users.value = generateDummyUsers(10);
};

const openForm = (user: User | null = null) => {
  if (user) {
    currentUser.value = { ...user };
    isEditMode.value = true;
  } else {
    currentUser.value = { id: 0, name: '', email: '', role: '' };
    isEditMode.value = false;
  }
};

const saveUser = () => {
  if (isEditMode.value) {
    const index = users.value.findIndex((u) => u.id === currentUser.value.id);
    if (index !== -1) {
      users.value[index] = { ...currentUser.value };
    }
    alert('User updated successfully');
  } else {
    currentUser.value.id = users.value.length + 1;
    users.value.push({ ...currentUser.value });
    alert('User created successfully');
  }

  openForm();
};

const deleteUser = (id: number) => {
  if (confirm('Are you sure you want to delete this user?')) {
    users.value = users.value.filter((user) => user.id !== id);
    alert('User deleted successfully');
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

onMounted(() => {
  fetchUsers();
});
</script>

<template>
  <div class="user-management relative">
    <div v-if="currentUser.id !== 0" class="p-4 bg-white rounded-sm shadow-sm">
      <h2 class="text-lg font-semibold mb-6">{{ isEditMode ? 'Edit User' : 'Add User' }}</h2>
      <form @submit.prevent="saveUser" class="grid grid-cols-1 sm:grid-cols-2 gap-4">
        <div class="mb-3">
          <label for="name" class="text-sm font-medium text-gray-700">Name:</label>
          <input type="text" id="name" v-model="currentUser.name" required
            class="mt-1 block w-full p-1 rounded-sm shadow-sm focus:ring-[var(--ring-color)] focus:border-[var(--border-color)]"
            :style="{ '--ring-color': selectColor(), '--border-color': selectColor() }" />
        </div>
        <div class="mb-3">
          <label for="email" class="text-sm font-medium text-gray-700">Email:</label>
          <input type="email" id="email" v-model="currentUser.email" required
            class="mt-1 block w-full p-1 rounded-sm shadow-sm focus:ring-[var(--ring-color)] focus:border-[var(--border-color)]"
            :style="{ '--ring-color': selectColor(), '--border-color': selectColor() }" />
        </div>
        <div class="mb-3">
          <label for="role" class="text-sm font-medium text-gray-700">Role:</label>
          <input type="text" id="role" v-model="currentUser.role" required
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
      <DataTable :data="users" :columns="columns">
        <template #actions="{ row }">
          <div class="flex justify-center space-x-1">
            <button class="w-6 h-6 cursor-pointer shadow-md flex items-center justify-center bg-white rounded-full"
              @click="openForm(row)" :style="{ color: selectColor() }">
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