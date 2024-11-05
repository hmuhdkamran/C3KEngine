<script setup lang="ts">
import { defineProps } from 'vue';

defineProps<{ activities: { activity: string; timestamp: string }[] }>();
</script>

<template>
    <div class="p-6 bg-white rounded-lg shadow-md">
        <h3 class="text-lg font-semibold text-gray-700">Recent Activity Logs</h3>
        <ul class="mt-4 space-y-2">
            <li v-for="activity in activities" :key="activity.timestamp" class="text-sm text-gray-500">
                {{ activity.activity }} - {{ activity.timestamp }}
            </li>
        </ul>
    </div>
</template>

<style scoped></style>