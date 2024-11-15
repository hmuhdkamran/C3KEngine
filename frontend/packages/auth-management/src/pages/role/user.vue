<script setup lang="ts">
import { UsersService } from '@/services';
import { DataTable, ConfirmationDialog } from 'c3k-library';
import { onMounted, ref, type Ref } from 'vue';
import UserOperations from './user-operations.vue';
import type { IUser } from '@/models';

const repo = new UsersService();
const data: Ref<IUser[]> = ref([]);

const columns = [
  { key: 'Username', label: 'User Name', sort: true },
  { key: 'DisplayName', label: 'Name', sort: true },
  { key: 'Language', label: 'Language', sort: false, width: '300px', class: 'text-center' },
  { key: 'StatusId', label: 'Status', sort: false, width: '300px', class: 'text-center' },
  { key: 'action', label: 'Action', sort: false, width: '300px', class: 'text-center' },
];

const load = () => {
  repo.GetAll().then((res: any) => {
    if (res)
    data.value = res.data
  })
}

onMounted(() => {
  load();
});

const isDeleteDialogVisible = ref(false);
const isDrawerVisible = ref(false);
const isEditMode = ref(false);
const isViewMode = ref(false);
const currentEntry = ref<IUser | null>(null);

const openModal = (action: string, row: IUser | null = null) => {
  if (action === 'add') {
    currentEntry.value = { UserId: '', Username: '', DisplayName: '', Language: '', Password: '', Salt: '', StatusId: 'Active' } as IUser;
    isEditMode.value = true;
    isViewMode.value = false;
  } else if (action === 'edit') {
    currentEntry.value = row;
    isEditMode.value = true;
    isViewMode.value = false;
  } else if (action === 'view') {
    currentEntry.value = row;
    isEditMode.value = false;
    isViewMode.value = true;
  }
  isDrawerVisible.value = true;
};

const closeDrawer = () => {
  isDrawerVisible.value = false;
  isEditMode.value = false;
  isViewMode.value = false;
  currentEntry.value = null;
};

const saveEntry = async () => {
  if (currentEntry.value) {
    try {
      if (isEditMode.value) {
        await repo.Update(currentEntry.value);
      } else {
        await repo.AddOne(currentEntry.value);
      }
      load();
      closeDrawer();
    } catch (error) {
      console.error('Failed to save entry:', error);
    }
  }
};

const onDelete = () => {
  isDeleteDialogVisible.value = true;
};

const onConfirmDelete = () => {
  isDeleteDialogVisible.value = false;
};

const onCancelDelete = () => {
  isDeleteDialogVisible.value = false;
};
</script>

<template>
  <DataTable :data="data" :columns="columns" :check-column="false">
    <template #status="{ row }">
      <span :class="row.StatusId === 'Active' ? 'bg-green-100 text-green-700' : 'bg-yellow-100 text-yellow-700'" class="px-1 py-0.5 rounded-full">
        {{ row.StatusId === 'Active' ? 'Active' : 'Inactive' }}
      </span>
    </template>
    <template #action="{ row }">
      <div class="flex justify-center space-x-2">
        <button class="grid-action-btn hover-btn-warning" @click="openModal('view', row)">
          <span class="icon-[ep--view]"></span>
        </button>
        <button class="grid-action-btn hover-btn-primary" @click="openModal('edit', row)">
          <span class="icon-[akar-icons--edit]"></span>
        </button>
        <button class="grid-action-btn hover-btn-danger" @click="onDelete">
          <span class="icon-[hugeicons--delete-02]"></span>
        </button>
      </div>
    </template>
  </DataTable>

  <ConfirmationDialog
    v-if="isDeleteDialogVisible"
    title="Confirm Delete"
    message="Are you sure you want to delete this entry?"
    :isVisible="isDeleteDialogVisible"
    @confirm="onConfirmDelete"
    @cancel="onCancelDelete"
  />

  <UserOperations
    :isDrawerVisible="isDrawerVisible"
    :isEditMode="isEditMode"
    :isViewMode="isViewMode"
    :currentEntry="currentEntry"
    @close="closeDrawer"
    @save="saveEntry"
  />
</template>
<route lang="yaml">
  meta:
    layout: application
    action: read
</route>