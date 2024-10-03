<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { DataTable, useTableStore } from 'c3k-library';
import { UsersService } from '@/service/auth/role/user-service';
import { IUsers } from '@/models/user';

// const users = ref([
//   { id: 1, name: "Ahmed Ali", email: "ahmed.ali@example.com", role: "Admin" },
//   { id: 2, name: "Sana Khan", email: "sana.khan@example.com", role: "User" },
//   { id: 3, name: "Raza Siddiqui", email: "raza.siddiqui@example.com", role: "Manager" },
//   { id: 4, name: "Aisha Noor", email: "aisha.noor@example.com", role: "User" },
//   { id: 5, name: "Bilal Ahmed", email: "bilal.ahmed@example.com", role: "Admin" },
//   { id: 6, name: "Hina Aslam", email: "hina.aslam@example.com", role: "User" },
//   { id: 7, name: "Faizan Shah", email: "faizan.shah@example.com", role: "User" },
//   { id: 8, name: "Zainab Malik", email: "zainab.malik@example.com", role: "Admin" },
//   { id: 9, name: "Usman Tariq", email: "usman.tariq@example.com", role: "User" },
//   { id: 10, name: "Ayesha Tariq", email: "ayesha.tariq@example.com", role: "Manager" },
// ]);

const usersService = new UsersService();
const tableStore = useTableStore();

const users = ref<IUsers[]>([]); 
const showAddUserModal = ref(false);
const newUser = ref<IUsers>({ UserId: '', name: '', email: '', role: 'User' }); 

const fetchUsers = async () => {
  try {
    const response = await usersService.GetAll();
    users.value = response.data || []; 
    tableStore.updateTotalRecords(users.value.length);
  } catch (error) {
    console.error('Failed to fetch users:', error);
  }
};

const addUser = async () => {
  try {
    await usersService.AddOne({ ...newUser.value });
    await fetchUsers();
    showAddUserModal.value = false; 
  } catch (error) {
    console.error('Failed to add user:', error);
  }
};

const openModal = (row: IUsers) => {
  console.log('Open modal for:', row);
};

const reload = async () => {
  await fetchUsers();
};

onMounted(fetchUsers);

const columns = [
  { key: 'check', label: '', sort: false, check: true },
  { key: 'id', label: 'ID', sort: true },
  { key: 'name', label: 'Name', sort: true },
  { key: 'email', label: 'Email', sort: true },
  { key: 'role', label: 'Role', sort: true },
  { key: 'actions', label: 'Actions', sort: false, class: 'text-center' },
];
</script>

<template>
  <div class="p-4 bg-gray-50 min-h-screen">
    <div class="flex justify-between items-center mb-6">
      <h1 class="text-2xl font-bold text-gray-800">User Management</h1>
      <div>
        <button @click="showAddUserModal = true" class="px-3 py-1 rounded-full hover:bg-gray-100">
          <span class="icon-[ic--baseline-plus]"></span> Add User
        </button>
        <button @click="reload" class="px-3 py-1 rounded-full hover:bg-gray-100">
          <span class="icon-[mdi--reload]"></span> Reload
        </button>
      </div>
    </div>
    <DataTable :data="users" :columns="columns" :check-column="true">
      <template #action="{ row }">
        <button @click="openModal(row)" class="text-gray-500 focus:outline-none hover:bg-violet-50 rounded-full p-2">
          <span class="icon-[mdi--edit-outline] w-4 h-4"></span>
        </button>
        <button @click="openModal(row)" class="ml-2 text-gray-500 focus:outline-none hover:bg-violet-50 rounded-full p-2">
          <span class="icon-[material-symbols--delete-outline] w-4 h-4"></span>
        </button>
      </template>
    </DataTable>
    <div v-if="showAddUserModal" class="fixed inset-0 z-50 flex items-center justify-center bg-gray-800 bg-opacity-50">
      <div class="bg-white p-6 rounded-sm shadow-lg w-full max-w-lg">
        <h2 class="text-md font-bold mb-4 text-gray-800">Add New User</h2>
        <form @submit.prevent="addUser" class="space-y-4">
          <div>
            <label class="block text-sm text-gray-600 mb-1" for="name">Name</label>
            <input v-model="newUser.name" class="w-full input-complete" type="text" id="name" required />
          </div>
          <div>
            <label class="block text-sm text-gray-600 mb-1" for="email">Email</label>
            <input v-model="newUser.email" class="w-full input-complete" type="email" id="email" required />
          </div>
          <div>
            <label class="block text-sm text-gray-600 mb-1" for="role">Role</label>
            <select v-model="newUser.role" class="w-full input-complete text-sm" id="role" required>
              <option value="User">User</option>
              <option value="Admin">Admin</option>
              <option value="Manager">Manager</option>
            </select>
          </div>
          <div class="flex justify-end space-x-2">
            <button @click="showAddUserModal = false" type="button" class="text-sm px-2 py-1 btn-secondary">Cancel</button>
            <button type="submit" class="text-sm px-2 py-1 btn-primary">Add</button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>
<style scoped>
</style>

<route lang="yaml">
  meta:
    layout: blank
    action: read
</route>