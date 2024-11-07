<script setup lang="ts">
import { defineProps } from 'vue';
import { Icon } from '@iconify/vue';

defineProps<{ title: string; count?: number }>();
</script>

<template>
    <div class="p-6 bg-gradient-to-br from-white to-violet-50 rounded-xl shadow-lg transform transition-all hover:shadow-xl">
        <div class="flex items-center mb-4 space-x-3">
            <Icon icon="mdi:chart-bar" class="text-violet-600 w-6 h-6" />
            <h3 class="text-lg font-semibold">{{ title }}</h3>
        </div>
        <p v-if="count !== undefined" class="mt-2 text-4xl font-bold text-gray-800">{{ count }}</p>
        <slot></slot>
    </div>
</template>

<style scoped>
</style>