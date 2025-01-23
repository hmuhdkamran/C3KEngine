<script lang="ts" setup>
import { ref, computed } from 'vue'
import { Edit, Delete } from '@element-plus/icons-vue'
import type { PlusColumn, FieldValues } from 'plus-pro-components'
import { selectColor } from '@/stores/colorPalette';

interface RowData {
  id: number
  name: string
  email: string
  role: string
}

const users = ref<RowData[]>([
  { id: 1, name: 'Ahmed Khan', email: 'ahmed.khan@example.com', role: 'Admin' },
  { id: 2, name: 'Sarah Ali', email: 'sarah.ali@example.com', role: 'User' },
  { id: 3, name: 'Ali Raza', email: 'ali.raza@example.com', role: 'User' },
  { id: 4, name: 'Zara Ahmed', email: 'zara.ahmed@example.com', role: 'Admin' },
])

const drawerVisible = ref(false)
const drawerTitle = ref('Add User')
const formData = ref<FieldValues>({ name: '', email: '', role: '' })

const columns: PlusColumn[] = [
  { label: 'ID', prop: 'id' },
  { label: 'Name', prop: 'name' },
  { label: 'Email', prop: 'email' },
  { label: 'Role', prop: 'role' },
]

const formColumns: PlusColumn[] = [
  {
    label: 'Name',
    prop: 'name',
    valueType: 'input',
    fieldProps: { placeholder: 'Enter name' },
  },
  {
    label: 'Email',
    prop: 'email',
    valueType: 'input',
    fieldProps: { placeholder: 'Enter email' },
  },
  {
    label: 'Role',
    prop: 'role',
    valueType: 'select',
    options: [
      { label: 'Admin', value: 'admin' },
      { label: 'User', value: 'user' },
    ],
  },
]

const formRules = {
  name: [{ required: true, message: 'Name is required', trigger: 'blur' }],
  email: [{ required: true, message: 'Email is required', trigger: 'blur' }],
  role: [{ required: true, message: 'Role is required', trigger: 'change' }],
}

const openDrawer = (action: 'add' | 'edit', record?: RowData) => {
  drawerTitle.value = action === 'add' ? 'Add User' : 'Edit User'
  const formData = ref<RowData>({ id: 0, name: '', email: '', role: '' })
  drawerVisible.value = true
}

const closeDrawer = () => {
  drawerVisible.value = false
}

const submitForm = () => {
  if (drawerTitle.value === 'Add User') {
    const newUser = { id: Date.now(), name: formData.value.name, email: formData.value.email, role: formData.value.role }
    users.value.push(newUser)
  } else {
    const index = users.value.findIndex((user) => user.id === formData.value.id)
    if (index !== -1) {
      users.value[index] = { ...formData.value }
    }
  }
  closeDrawer()
}

const deleteUser = (id: number) => {
  users.value = users.value.filter((user) => user.id !== id)
}

const buttons = computed(() => [
  {
    text: 'Edit',
    code: 'edit',
    props: (row: RowData) => ({
      type: 'button',
      icon: Edit,
      style: { color: '#0284c7', marginRight: '8px' },
      onClick: () => openDrawer('edit', row),
    }),
    show: true,
  },
  {
    text: 'Delete',
    code: 'delete',
    props: (row: RowData) => ({
      type: 'button',
      icon: Delete,
      style: { color: '#d9534f' },
      onClick: () => deleteUser(row.id),
    }),
    confirm: { options: { draggable: true } },
  },
])

</script>

<template>
  <div class="p-6 container mx-auto">
    <div class="flex justify-end mb-4">
      <button @click="openDrawer('add')" class="px-4 py-1 text-sm bg-blue-900 text-white rounded-sm"
      :style="{ backgroundColor: selectColor() }">
        Add
      </button>
    </div>
    <div class="adaptive-table-wrapper">
      <PlusTable ref="plusTableInstance" 
      :columns="columns" 
      :table-data="users"
      :pagination="{total: users.length, pageSizeList: [5, 10, 20],}" 
      adaptive
      :stripe="true"
      :border="true"
      :action-bar="{ buttons: buttons, type: 'icon', width: 200 }"

      class="shadow-sm border border-gray-200">
      </PlusTable>
    </div>

    <PlusDrawerForm v-model:visible="drawerVisible" :form="{ columns: formColumns, rules: formRules }"
      @submit="submitForm" :title="drawerTitle" />
  </div>
</template>

<style scoped></style>

<route lang="yaml">
meta:
  layout: auth
  redirectIfLoggedIn: true
</route>
