<script setup lang="ts">
// Import necessary components and utilities
import { DataTable, ConfirmationDialog, useSystemStore } from 'c3k-library'; // Import UI components for data display and confirmation dialog
import type { IUser } from '@/models'; // Import the IUser type for type safety
import { computed, onMounted } from 'vue'; // Import Vue utilities for reactivity and lifecycle hooks
import EntityOperation from './operation.vue'; // Import the component for add/edit operations
import { useRoleUserStore } from '@/stores'; // Import the store for managing user data

// Initialize the store for user operations
const store = useRoleUserStore();

// Reactive computed property for the delete confirmation dialog visibility
const deleteDialog = computed(() => store.store.Open && store.store.OperationType === 'delete');

// Define columns for the DataTable component
const columns = [
    { key: 'Username', label: 'User Name', sort: true }, // Sortable column for user names
    { key: 'DisplayName', label: 'Name', sort: true }, // Sortable column for display names
    { key: 'Language', label: 'Language', sort: false, width: '300px', class: 'text-center' }, // Fixed-width, centered column for language
    { key: 'StatusId', label: 'Status', sort: false, width: '300px', class: 'text-center' }, // Fixed-width, centered column for status
    { key: 'action', label: 'Action', sort: false, width: '300px', class: 'text-center' }, // Column for action buttons
];

// Lifecycle hook to fetch data when the component is mounted
onMounted(() => store.execute('get')); // Fetch the user data from the store
</script>

<template>
    <div>
        <!-- DataTable component for displaying user data -->
        <DataTable :data="store.allItems" :columns="columns" :check-column="false">
            <!-- Slot for custom rendering of the "action" column -->
            <template #action="{ row }">
                <div class="flex justify-center">
                    <!-- Button to view a user record -->
                    <button class="actionbtn rounded-full bg-white hover:bg-gray-50 shadow-sm hover:shadow-md transition duration-300 
                        transform hover:scale-110 p-1.5 flex items-center justify-center"
                        @click="store.selectItem((row as IUser), 'View Record')" title="View Record">
                        <span class="icon-[ep--view] text-yellow-700 text-sm"></span>
                    </button>
                    <!-- Button to edit a user record -->
                    <button class="actionbtn rounded-full bg-white hover:bg-gray-50 shadow-sm hover:shadow-md transition duration-300 
                        transform hover:scale-110 p-1.5 flex items-center justify-center"
                        @click="store.selectItem((row as IUser), 'Edit Record')" title="Edit Record">
                        <span class="icon-[akar-icons--edit] text-blue-700 text-sm"></span>
                    </button>
                    <!-- Button to delete a user record -->
                    <button class="actionbtn rounded-full bg-white hover:bg-gray-50 shadow-sm hover:shadow-md transition duration-300 
                        transform hover:scale-110 p-1.5 flex items-center justify-center"
                        @click="store.selectItem((row as IUser), 'Delete Record')" title="Delete Record">
                        <span class="icon-[hugeicons--delete-02] text-red-700 text-sm"></span>
                    </button>
                </div>
            </template>
        </DataTable>

        <!-- Include the EntityOperation component for add/edit functionality -->
        <EntityOperation />
        
        <!-- ConfirmationDialog for delete operation -->
        <ConfirmationDialog :is-visible="deleteDialog" title="Delete Record"
            message="Are you sure you want to remove this record" 
            @cancel="store.store.resetOperation()" 
            @confirm="store.execute('delete')" />
    </div>
</template>

<route lang="yaml">
    meta:
      layout: application
      action: read
</route>