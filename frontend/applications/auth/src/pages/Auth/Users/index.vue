<script setup lang="ts">
import { type DataTableColumns } from 'naive-ui/es/components'
import type { RowData } from 'naive-ui/es/data-table/src/interface'
import { storeToRefs } from 'pinia';
import { useRoleUserStore } from '~/store/role/user-store';

const { t } = useI18n()
const store = useRoleUserStore();
const { isLoading } = storeToRefs(store)

function getItems() {
    store.getItems();
}

onMounted(getItems);

const columns: DataTableColumns<RowData> = [
  {
    title: t('user.username'),
    key: 'Username',
  },
  {
    title: t('user.display_name'),
    key: 'DisplayName',
  },
  {
    title: t('user.language'),
    key: 'Language',
    width: 80,
  },
]

</script>

<template>
    <div>
    <n-data-table
      :bordered="false" :columns="columns" :data="store.items" :loading="isLoading"
      :scroll-x="500"
    />
  </div>
</template>

<route lang="yaml">
meta:
    name: users
    title: users
    layout: auth
    authRequired: true
    breadcrumb:
    - authentication
    - users
</route>