<script setup lang="ts">
import { defineProps, useSlots } from 'vue';

interface CardProps {
    title: string;
    description: string;
    status: string;
    buttonText: string;
    iconClass: string;
}

defineProps<CardProps>();

const slots = useSlots();
</script>

<template>
    <div
        class="card-container max-w-sm rounded-md overflow-hidden shadow-lg bg-white transition-transform transform hover:scale-105 hover:shadow-xl duration-300 sm:max-w-full lg:max-w-sm relative group">
        <!-- Header -->
        <div v-if="slots.header" class="card-header">
            <slot name="header"></slot>
        </div>
        <div v-else class="card-header px-4 py-2 flex items-center gap-3 border-b border-gray-100">
            <div class="icon-container relative w-12 h-12 bg-violet-200 rounded-full flex items-center justify-center">
                <span :class="`${iconClass} text-violet-700 text-lg`"></span>
            </div>
            <div class="flex-1">
                <div class="font-bold text-md flex items-center justify-between">
                    <span>{{ title }}</span>
                    <span class="icon-[ph--dots-three-vertical-bold] cursor-pointer"></span>
                </div>
                <p class="text-gray-500 text-sm">{{ description }}</p>
            </div>
        </div>

        <!-- Body -->
        <div v-if="slots.body" class="card-body p-4">
            <slot name="body"></slot>
        </div>

        <!-- Footer -->
        <div v-if="slots.footer" class="card-footer px-4 py-2 border-t border-gray-100">
            <slot name="footer"></slot>
        </div>
        <div v-else
            class="card-footer px-4 py-2 text-green-500 text-sm flex justify-between items-center border-t border-gray-100">
            <span>{{ status }}</span>
            <button
                class="bg-violet-100 hover:bg-violet-500 text-violet-700 hover:text-white py-1 px-3 rounded text-sm transition-colors duration-300">{{
                buttonText }}</button>
        </div>
    </div>
</template>

<style scoped>
.card-container {
    border: 1px solid #e2e8f0;
    border-left-width: 3px;
    border-left-color: #7c3aed;
}

.card-container:hover {
    box-shadow: 0 10px 20px -5px rgba(156, 163, 175, 0.3);
    transform: translateY(-2px);
}

.card-header {
    background-color: #f3f4f6;
}

.icon-container {
    transition: all 0.5s ease-in-out;
}
</style>