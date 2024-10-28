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
            <div class="icon-container relative w-11 h-11 bg-gradient-to-br from-indigo-200 via-purple-200 to-indigo-300 rounded-full flex items-center justify-center shadow-md">
                <span :class="`${iconClass} text-indigo-700 text-2xl`"></span>
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
    transition: transform 0.4s ease-in-out, box-shadow 0.3s ease;
}

.icon-container:hover {
    transform: rotate(10deg);
    box-shadow: 0 4px 8px rgba(99, 102, 241, 0.3);
}

.card-footer button:hover {
    box-shadow: 0 4px 8px rgba(99, 102, 241, 0.2);
}
</style>