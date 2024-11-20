<script setup lang="ts">
import { UsersService } from '@/services';
import { DataTable, ConfirmationDialog } from 'c3k-library';
import { onMounted, ref, watch, type Ref } from 'vue';
import UserOperations from './user-operations.vue';
import type { IUser } from '@/models';
import { useApplicationEventStore } from '@/stores/application';

const repo = new UsersService();
const store = useApplicationEventStore();

const data: Ref<IUser[]> = ref([]);

const columns = [
  { key: 'Username', label: 'User Name', sort: true },
  { key: 'DisplayName', label: 'Name', sort: true },
  { key: 'Language', label: 'Language', sort: false, width: '300px', class: 'text-center' },
  { key: 'StatusId', label: 'Status', sort: false, width: '300px', class: 'text-center' },
  { key: 'action', label: 'Action', sort: false, width: '300px', class: 'text-center' },
];

const Operation = ref()

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

const openModal = (action: string, row: IUser | any) => {
  Operation.value = action;

  if (action.toLowerCase().startsWith('add')) {
    currentEntry.value = { UserId: '', Username: '', DisplayName: '', Language: '', Password: '', Salt: '', StatusId: 'Active' } as IUser;
  } else if (action.toLowerCase().startsWith('edit')) {
    currentEntry.value = row;
  } else if (action.toLowerCase().startsWith('view')) {
    currentEntry.value = row;
  }
  store.ToggleDrawer = true;
};

const closeDrawer = () => {
  isDrawerVisible.value = false;
  isEditMode.value = false;
  isViewMode.value = false;
  currentEntry.value = null;
  store.ToggleDrawer = false;
};

watch(()=>store.toggleDrawer, ()=> {
  isDrawerVisible.value = store.toggleDrawer
})

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
      <span :class="row.StatusId === 'Active' ? 'bg-green-100 text-green-700' : 'bg-yellow-100 text-yellow-700'"
        class="px-1 py-0.5 rounded-full">
        {{ row.StatusId === 'Active' ? 'Active' : 'Inactive' }}
      </span>
    </template>
    <template #action="{ row }">
      <div class="flex justify-center space-x-3">
        <button class="actionbtn rounded-md bg-yellow-100 hover:bg-yellow-200 hover:shadow-lg transition duration-300 transform hover:scale-110"
          @click="openModal('View Record', row)" title="View Record">
          <span class="icon-[ep--view] text-yellow-600"></span>
        </button>
        <button class="actionbtn rounded-md bg-blue-100 hover:bg-blue-200 hover:shadow-lg transition duration-300 transform hover:scale-110"
          @click="openModal('Edit Record', row)" title="Edit Record">
          <span class="icon-[akar-icons--edit] text-blue-600"></span>
        </button>
        <button class="actionbtn rounded-md bg-red-100 hover:bg-red-200 hover:shadow-lg transition duration-300 transform hover:scale-110"
          @click="onDelete" title="Delete Record">
          <span class="icon-[hugeicons--delete-02] text-red-600"></span>
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

  <UserOperations :isDrawerVisible="isDrawerVisible" :title="Operation" :currentEntry="currentEntry"
    @close="closeDrawer" @save="saveEntry" />
</template>
<style scoped>
.actionbtn {
  width: 1.5rem;
  height: 1.5rem;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border: none;
  cursor: pointer;
  outline: none;
  font-size: 1.rem;
}
</style>
<route lang="yaml">
  meta:
    layout: application
    action: read
</route>