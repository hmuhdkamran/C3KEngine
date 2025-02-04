<script setup lang="ts">
import { ref, onMounted, computed, onUnmounted } from 'vue';
import { DataTable, Pagination } from 'c3-library';
import { selectColor } from '@/stores/colorPalette';
import { UserService } from '@/services/user-service';
import type { User } from '@/models/user';
import { useSystemStore } from 'c3-library';
import { useRoute, useRouter } from 'vue-router';

const users = ref<User[]>([]);
const userService = new UserService();
const store = useSystemStore();
const router = useRouter();
const confirmPassword = ref('');

const isFormOpen = ref(false);
const currentUser = ref<User>({
  UserId: '',
  Username: '',
  DisplayName: '',
  Language: '',
  Password: '',
  Salt: '',
  StatusId: ''
});
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

const openForm = (user?: User) => {
  if (user) {
    currentUser.value = { ...user };
    isEditMode.value = true;
  } else {
    currentUser.value = {
      UserId: '',
      Username: '',
      DisplayName: '',
      Language: 'English',
      Password: '',
      Salt: '',
      StatusId: 'Active',
    };
    confirmPassword.value = ''; 
    isEditMode.value = false;
  }
  isFormOpen.value = true;
};

const closeForm = () => {
  isFormOpen.value = false;
};

const saveUser = async () => {
  if (isEditMode.value) {
    if (!currentUser.value.UserId || !currentUser.value.Username || !currentUser.value.DisplayName || !currentUser.value.Language || !currentUser.value.Password || !currentUser.value.Salt || !currentUser.value.StatusId) {
      alert('User data is incomplete. Please fill all required fields.');
      return;
    }
    const result = await userService.update(currentUser.value);
    if (result) {
      alert('User updated successfully');
      fetchUsersData();
      closeForm();
    }
  } else {
    if (!currentUser.value.Username || !currentUser.value.DisplayName || !currentUser.value.Language || !currentUser.value.Password || !currentUser.value.StatusId) {
      alert('User data is incomplete. Please fill all required fields.');
      return;
    }

    if (currentUser.value.Password !== confirmPassword.value) {
      alert('Passwords do not match.');
      return;
    }

    const result = await userService.add(currentUser.value);
    if (result) {
      alert('User created successfully');
      fetchUsersData();
      closeForm();
    }
  }
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

const handleOpenFormEvent = (event: CustomEvent) => {
  if (event.detail && event.detail.user) {
    openForm(event.detail.user);
  } else {
    openForm();
  }
};

onMounted(() => {
  const userManagementComponent = document.querySelector('.user-management');
  if (userManagementComponent) {
    userManagementComponent.addEventListener('openForm', handleOpenFormEvent);
  }
});

onUnmounted(() => {
  const userManagementComponent = document.querySelector('.user-management');
  if (userManagementComponent) {
    userManagementComponent.removeEventListener('openForm', handleOpenFormEvent);
  }
});

const toggleStatus = () => {
  currentUser.value.StatusId = currentUser.value.StatusId === 'Active' ? 'Inactive' : 'Active';
};
</script>

<template>
  <div class="user-management relative">
    <div v-if="isFormOpen" class="p-4 bg-white rounded-sm shadow-sm">
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
          <select id="Language" v-model="currentUser.Language" required
            class="mt-1 block w-full p-1 rounded-sm shadow-sm focus:ring-[var(--ring-color)] focus:border-[var(--border-color)]"
            :style="{ '--ring-color': selectColor(), '--border-color': selectColor() }">
            <option value="English">English</option>
            <option value="Urdu">Urdu</option>
          </select>
        </div>
        <div class="mb-3" v-if="!isEditMode">
          <label for="Password" class="text-sm font-medium text-gray-700">Password:</label>
          <input type="password" id="Password" v-model="currentUser.Password" required
            class="mt-1 block w-full p-1 rounded-sm shadow-sm focus:ring-[var(--ring-color)] focus:border-[var(--border-color)]"
            :style="{ '--ring-color': selectColor(), '--border-color': selectColor() }" />
        </div>
        <div class="mb-3" v-if="!isEditMode">
          <label for="ConfirmPassword" class="text-sm font-medium text-gray-700">Confirm Password:</label>
          <input type="password" id="ConfirmPassword" v-model="confirmPassword" required
            class="mt-1 block w-full p-1 rounded-sm shadow-sm focus:ring-[var(--ring-color)] focus:border-[var(--border-color)]"
            :style="{ '--ring-color': selectColor(), '--border-color': selectColor() }" />
        </div>
        <div class="mb-3" v-if="!isEditMode">
          <label for="UserRole" class="text-sm font-medium text-gray-700">User Role:</label>
          <select id="UserRole" v-model="currentUser.UserRole" required
            class="mt-1 block w-full p-1 rounded-sm shadow-sm focus:ring-[var(--ring-color)] focus:border-[var(--border-color)]"
            :style="{ '--ring-color': selectColor(), '--border-color': selectColor() }">
            <option value="Admin">Admin</option>
            <option value="User">User</option>
            <option value="Guest">Guest</option>
          </select>
        </div>
        <div class="mb-3" v-if="isEditMode">
          <label for="Status" class="text-sm font-medium text-gray-700">Status:</label>
          <div class="mt-1">
            <button @click="toggleStatus" class="inline-flex items-center" :class="{
              'text-green-700': currentUser.StatusId === 'Active',
              'text-red-600': currentUser.StatusId !== 'Active'
            }">
              <span v-if="currentUser.StatusId === 'Active'" class="fa-solid fa-toggle-on fa-xl"></span>
              <span v-else class="fa-solid fa-toggle-off fa-xl"></span>
              <span class="ml-2">{{ currentUser.StatusId === 'Active' ? 'Active' : 'Inactive' }}</span>
            </button>
          </div>
        </div>
        <div class="flex justify-end gap-3 mt-4 col-span-2">
          <button type="button" class="px-3 py-1.5 bg-gray-200 rounded-sm hover:bg-gray-300 transition"
            @click="closeForm()">Cancel</button>
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
</style>

<route lang="yaml">
  meta:
    layout: auth
    redirectIfLoggedIn: true
</route>