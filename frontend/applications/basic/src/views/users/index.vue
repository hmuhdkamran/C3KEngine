<script setup lang="ts">
import { ref } from 'vue'

const users = ref<any[]>([
    { id: 1, name: 'Ahmed Khan', email: 'ahmed.khan@example.com', role: 'Admin', phone: '03001234567', address: 'Lahore, Punjab' },
    { id: 2, name: 'Sarah Ali', email: 'sarah.ali@example.com', role: 'User', phone: '03007654321', address: 'Karachi, Sindh' },
    { id: 3, name: 'Ali Raza', email: 'ali.raza@example.com', role: 'User', phone: '03001239876', address: 'Islamabad, Capital Territory' },
    { id: 4, name: 'Zara Ahmed', email: 'zara.ahmed@example.com', role: 'Admin', phone: '03005432345', address: 'Peshawar, KPK' },
])

const editUser = ref<any | null>(null)
const userName = ref('')
const userEmail = ref('')
const userRole = ref('')
const userPhone = ref('')
const userAddress = ref('')

const createUser = () => {
    const newUser = {
        id: Date.now(),
        name: userName.value,
        email: userEmail.value,
        role: userRole.value,
        phone: userPhone.value,
        address: userAddress.value,
    }
    users.value.push(newUser)
    clearForm()
}

const editUserDetails = (user: any) => {
    editUser.value = { ...user }
    userName.value = user.name
    userEmail.value = user.email
    userRole.value = user.role
    userPhone.value = user.phone
    userAddress.value = user.address
}

const updateUser = () => {
    if (editUser.value) {
        editUser.value.name = userName.value
        editUser.value.email = userEmail.value
        editUser.value.role = userRole.value
        editUser.value.phone = userPhone.value
        editUser.value.address = userAddress.value
    }
    clearForm()
    editUser.value = null
}

const deleteUser = (userId: number) => {
    users.value = users.value.filter(user => user.id !== userId)
}

const clearForm = () => {
    userName.value = ''
    userEmail.value = ''
    userRole.value = ''
    userPhone.value = ''
    userAddress.value = ''
}
</script>

<template>
    <main class="flex-1 bg-gray-100">
        <div class="container mx-auto p-6">
            <h1 class="text-2xl font-semibold mb-4">User Management</h1>

            <div class="mb-6 p-4 border bg-white rounded-md shadow-md">
                <h2 class="text-xl mb-4">{{ editUser ? 'Edit User' : 'Create User' }}</h2>
                <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
                    <div>
                        <label class="block text-gray-700">Name</label>
                        <input v-model="userName" type="text" class="w-full p-2 border rounded-md"
                            placeholder="Enter name" />
                    </div>
                    <div>
                        <label class="block text-gray-700">Email</label>
                        <input v-model="userEmail" type="email" class="w-full p-2 border rounded-md"
                            placeholder="Enter email" />
                    </div>
                    <div>
                        <label class="block text-gray-700">Phone</label>
                        <input v-model="userPhone" type="text" class="w-full p-2 border rounded-md"
                            placeholder="Enter phone number" />
                    </div>
                    <div>
                        <label class="block text-gray-700">Address</label>
                        <input v-model="userAddress" type="text" class="w-full p-2 border rounded-md"
                            placeholder="Enter address" />
                    </div>
                    <div class="w-full">
                        <label class="block text-gray-700">Role</label>
                        <select v-model="userRole" class="w-full p-2 border rounded-md">
                            <option value="Admin">Admin</option>
                            <option value="User">User</option>
                        </select>
                    </div>
                </div>
                <div class="mt-4">
                    <button @click="editUser ? updateUser() : createUser()"
                        class="bg-blue-500 text-white p-2 rounded-md">
                        {{ editUser ? 'Update' : 'Create' }} User
                    </button>
                    <button v-if="editUser" @click="clearForm"
                        class="ml-2 p-2 text-gray-500 border rounded-md">Cancel</button>
                </div>
            </div>

            <div class="overflow-x-auto shadow-md bg-white border rounded-lg">
                <table class="min-w-full table-auto">
                    <thead class="bg-gray-100">
                        <tr>
                            <th class="border px-4 py-2">Name</th>
                            <th class="border px-4 py-2">Email</th>
                            <th class="border px-4 py-2">Phone</th>
                            <th class="border px-4 py-2">Address</th>
                            <th class="border px-4 py-2">Role</th>
                            <th class="border px-4 py-2">Actions</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr v-for="user in users" :key="user.id">
                            <td class="border px-4 py-2">{{ user.name }}</td>
                            <td class="border px-4 py-2">{{ user.email }}</td>
                            <td class="border px-4 py-2">{{ user.phone }}</td>
                            <td class="border px-4 py-2">{{ user.address }}</td>
                            <td class="border px-4 py-2">{{ user.role }}</td>
                            <td class="border px-4 py-2">
                                <button @click="editUserDetails(user)" class="text-blue-500">Edit</button>
                                <button @click="deleteUser(user.id)" class="ml-2 text-red-500">Delete</button>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
    </main>
</template>

<route lang="yaml">
    meta:
      layout: auth
      redirectIfLoggedIn: true
    </route>