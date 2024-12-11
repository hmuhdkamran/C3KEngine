<script setup lang="ts">
import type { IUser } from '@/models';
import { defineProps } from 'vue';

const props = defineProps<{ users: IUser[] }>();
</script>

<template>
    <div class="p-6 bg-white rounded-lg shadow-md">
        <h3 class="text-lg font-semibold text-gray-700">User List</h3>
        <table class="w-full mt-4 border-collapse">
            <thead>
                <tr>
                    <th class="px-4 py-2 text-left text-sm font-semibold text-gray-600">User</th>
                    <th class="px-4 py-2 text-left text-sm font-semibold text-gray-600">Role</th>
                    <th class="px-4 py-2 text-left text-sm font-semibold text-gray-600">Status</th>
                </tr>
            </thead>
            <tbody>
                <tr v-for="user in users" :key="user.UserId">
                    <td class="px-4 py-2 text-gray-800">{{ user.DisplayName }}</td>
                    <td class="px-4 py-2 text-gray-800">{{ user.Username }}</td>
                    <td :class="{ 'text-green-600': user.StatusId, 'text-red-600': !user.StatusId }" class="px-4 py-2">
                        {{ user.StatusId ? 'Active' : 'Inactive' }}
                    </td>
                </tr>
            </tbody>
        </table>
    </div>
</template>

<style scoped></style>