<script setup lang="ts">
import { onMounted } from 'vue';
import { useSystemStore, useTableStore } from 'c3-library';
import { DataTable, Pagination } from 'c3-library';
import type { IUser } from '@/models';
import { setFormOpen } from '@/stores/edit-form';

import AddEdit from './add-edit.vue';
import { useRoleUserStore, useRoleRolesStore, useRoleUserRoleMapStore, useSetupStatusStore, useRoleProductsStore, useRoleUserProductMapsStore } from '@/stores';

const store = useRoleUserStore();
const color = useSystemStore();
const tableStore = useTableStore();

const roleStore = useRoleRolesStore();
const userRoleStore = useRoleUserRoleMapStore();

const productStore = useRoleProductsStore();
const userProductStore = useRoleUserProductMapsStore();

const statusStore = useSetupStatusStore();

const columns = [
  { key: 'DisplayName', label: 'Name', sort: true },
  { key: 'Username', label: 'Email', sort: true },
  { key: 'Language', label: 'Language', sort: true },
  { key: 'actions', label: 'Actions', sort: false, class: 'text-center' },
];

const openForm = (user: IUser | null = null) => {
  if (user) {
    store.item = { ...user };
    userRoleStore.getItems(`"UserId"='${store.item.UserId}'`);
    userProductStore.getItems(`"UserId"='${store.item.UserId}'`);
    store.shouldUpdate = true;
    setFormOpen(true);
  }
};

const deleteUser = (user: IUser) => {
  if (confirm('Are you sure you want to delete this user?')) {
    store.deleteItem(user)
  }
};

onMounted(() => {
  store.getItems().then(() => {
    tableStore.totalRecords = store.items.length;
  });

  roleStore.getItems();
  statusStore.getItems();
  productStore.getItems();
});
</script>

<template>
  <div class="relative">
    <DataTable :data="store.items" :columns="columns">
      <template #actions="{ row }">
        <div class="flex justify-center space-x-1">
          <button class="w-6 h-6 cursor-pointer shadow-md flex items-center justify-center bg-white rounded-full"
            @click="openForm(row as any)" :style="{ color: color.application.primaryColor }">
            <span class="fas fa-pen"></span>
          </button>
          <button class="w-6 h-6 cursor-pointer shadow-md flex items-center justify-center bg-white rounded-full"
            @click="deleteUser(row as IUser)" :style="{ color: color.application.primaryColor }">
            <span class="fas fa-trash-can"></span>
          </button>
        </div>
      </template>
    </DataTable>

    <!-- Pagination positioned at the bottom right -->
    <div class="absolute bottom-0 right-1">
      <Pagination />
    </div>

    <AddEdit />
  </div>
</template>

<style scoped></style>

<route lang="yaml">
  meta:
    layout: auth
    authentication: true
    product: auth
    module: role-user
    breadcrumb:
      - role
      - user
</route>