<script setup lang="ts">
import { UsersService } from '@/services';
import { DataTable, ConfirmationDialog } from 'c3k-library';
import { onMounted, ref, watch, type Ref } from 'vue';
import UserOperations from './user-operations.vue';
import type { IUser } from '@/models';
import { useApplicationStore } from '@/stores/counter';

const repo = new UsersService();
const store = useApplicationStore();
const data: Ref<IUser[]> = ref([]);

const columns = [
  { key: 'action', label: 'Action', sort: false, width: '300px', class: 'text-center' },
  { key: 'Username', label: 'User Name', sort: true },
  { key: 'DisplayName', label: 'Name', sort: true },
  { key: 'Language', label: 'Language', sort: false, width: '300px', class: 'text-center' },
  { key: 'status', label: 'Status', sort: false, width: '300px', class: 'text-center' },
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
const currentEntry = ref<IUser | null>(null); 

const openModal = (action: string, row: IUser | any) => {
  currentEntry.value = row;
  isEditMode.value = action === 'edit';
  isDrawerVisible.value = true;
};

const closeDrawer = () => {
  isDrawerVisible.value = false;
  isEditMode.value = false;
  currentEntry.value = null;
};

watch(()=>store.toggleDrawer, ()=> {
  isDrawerVisible.value = store.toggleDrawer
})

const saveEntry = async () => {
  if (currentEntry.value && isEditMode.value) {
    try {
      await repo.Update(currentEntry.value);
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
  <div class="about">
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
      :currentEntry="currentEntry"
      @close="closeDrawer"
      @save="saveEntry"
    />
  </div>
</template>
<route lang="yaml">
  meta:
    layout: application
    action: read
</route>