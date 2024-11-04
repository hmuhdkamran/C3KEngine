<script setup lang="ts">
import { UsersService } from '@/services'
import { type IUser, DataTable } from 'c3k-library';
import { onMounted, ref, type Ref } from 'vue';

const repo: UsersService = new UsersService();
const data: Ref<Array<IUser>> = ref([]);

const columns = [
  // { key: 'check', label: 'check', sort: false, check: true },
  { key: 'action', label: 'Action', sort: false, width: '100px', class: 'text-center' },
  { key: 'Username', label: 'User Name', sort: true },
  { key: 'DisplayName', label: 'Name', sort: true },
  { key: 'Language', label: 'Language', sort: false, width: '100px', class: 'text-center' },
];

onMounted(() => {
  repo.GetAll().then((res: any) => data.value = res.data as IUser[]);
});

</script>

<template>
  <div class="about">
    <h1>This is an about page</h1>

    <DataTable :data="data" :columns="columns" :check-column="false">
      <template #action="{ row }">
        <div class="flex justify-center space-x-2">
          <button class="grid-action-btn hover-btn-warning">
            <span class="icon-[ep--view]"></span>
          </button>
          <button class="grid-action-btn hover-btn-primary">
            <span class="icon-[akar-icons--edit]"></span>
          </button>
          <button class="grid-action-btn hover-btn-danger">
            <span class="icon-[hugeicons--delete-02]"></span>
          </button>
        </div>
      </template>
    </DataTable>
  </div>
</template>

<route lang="yaml">
  meta:
    layout: application
    action: read
</route>