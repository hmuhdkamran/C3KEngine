<script lang="ts" setup>
import { ref } from 'vue'
import type { PlusColumn, FieldValues, PlusTable, PlusDrawerForm } from 'plus-pro-components'

interface User {
    id: number
    name: string
    email: string
    role: string
}

const users = ref<User[]>([
    { id: 1, name: 'Ahmed Khan', email: 'ahmed.khan@example.com', role: 'Admin' },
    { id: 2, name: 'Sarah Ali', email: 'sarah.ali@example.com', role: 'User' },
    { id: 3, name: 'Ali Raza', email: 'ali.raza@example.com', role: 'User' },
    { id: 4, name: 'Zara Ahmed', email: 'zara.ahmed@example.com', role: 'Admin' },
])

const drawerVisible = ref(false)
const drawerTitle = ref('Add User')
const formData = ref<FieldValues>({})

const columns: PlusColumn[] = [
    { label: 'ID', prop: 'id', width: 60 },
    { label: 'Name', prop: 'name', width: 120 },
    { label: 'Email', prop: 'email', width: 200 },
    { label: 'Role', prop: 'role', width: 120 },
    {
        label: 'Actions',
        prop: 'actions',
        valueType: 'slot',
        fixed: 'right',
        width: 200
    }
]

const formColumns: PlusColumn[] = [
    {
        label: 'Name',
        prop: 'name',
        valueType: 'input',
        fieldProps: { placeholder: 'Enter name' }
    },
    {
        label: 'Email',
        prop: 'email',
        valueType: 'input',
        fieldProps: { placeholder: 'Enter email' }
    },
    {
        label: 'Role',
        prop: 'role',
        valueType: 'select',
        options: [
            { label: 'Admin', value: 'admin' },
            { label: 'User', value: 'user' }
        ]
    }
]

const formRules = {
    name: [
        { required: true, message: 'Name is required', trigger: 'blur' }
    ],
    email: [
        { required: true, message: 'Email is required', trigger: 'blur' }
    ],
    role: [
        { required: true, message: 'Role is required', trigger: 'change' }
    ]
}

const openDrawer = (action: 'add' | 'edit', record?: User) => {
    drawerTitle.value = action === 'add' ? 'Add User' : 'Edit User'
    formData.value = record || {}
    drawerVisible.value = true
}

const closeDrawer = () => {
    drawerVisible.value = false
}

const submitForm = () => {
    if (drawerTitle.value === 'Add User') {
        const newUser = { id: Date.now(), ...formData.value }
        users.value.push(newUser)
    } else {
        const index = users.value.findIndex(user => user.id === formData.value.id)
        if (index !== -1) {
            users.value[index] = { ...formData.value }
        }
    }
    closeDrawer()
}

const deleteUser = (id: number) => {
    users.value = users.value.filter(user => user.id !== id)
}
</script>

<template>
    <div class="container mx-auto p-6">
        <div class="flex justify-end mb-4">
            <button @click="openDrawer('add')" class="px-4 py-2 bg-blue-900 text-white rounded-md">
                Add User
            </button>
        </div>
        <div class="adaptive-table-wrapper">
            <PlusTable ref="plusTableInstance" :columns="columns" :table-data="users" :pagination="{
                total: users.length,
                pageSizeList: [5, 10, 20]
            }" adaptive>
                <template #actions="{ record }">
                    <button @click="openDrawer('edit', record)" class="px-2 py-1 bg-yellow-500 text-white rounded-md">
                        Edit
                    </button>
                    <button @click="deleteUser(record.id)" class="px-2 py-1 bg-red-500 text-white rounded-md">
                        Delete
                    </button>
                </template>
            </PlusTable>
        </div>

        <PlusDrawerForm v-model:visible="drawerVisible" :form="{ columns: formColumns, rules: formRules }"
            @submit="submitForm" :title="drawerTitle" />
    </div>
</template>

<style scoped>

</style>

<route lang="yaml">
    meta:
      layout: auth
      redirectIfLoggedIn: true
    </route>