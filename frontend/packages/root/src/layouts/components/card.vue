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
    <div class="max-w-sm rounded overflow-hidden shadow-lg bg-white transition transform hover:scale-105 duration-200 sm:max-w-full lg:max-w-sm">
        <!-- Header -->
        <div v-if="slots.header">
            <slot name="header"></slot>
        </div>
        <div v-else class="px-2 py-1 flex items-center gap-3">
            <div class="relative w-10 h-10 bg-violet-200 rounded-full">
                <span
                    :class="`${iconClass} absolute w-6 h-6 text-violet-700 transform -translate-x-1/2 -translate-y-1/2 left-1/2 top-1/2`"></span>
            </div>
            <div class="flex-1">
                <div class="font-bold text-md mb-1 flex items-center justify-between">
                    <span>{{ title }}</span>
                    <span class="icon-[ph--dots-three-vertical-bold]"></span>
                </div>
                <p class="text-gray-500 text-xm">{{ description }}</p>
            </div>
        </div>

        <!-- Body -->
        <div v-if="slots.body">
            <slot name="body"></slot>
        </div>

        <!-- Footer -->
        <div v-if="slots.footer">
            <slot name="footer"></slot>
        </div>
        <div v-else class="px-2 py-1 text-green-500 text-sm flex justify-between">
            <span>{{ status }}</span>
            <button class="bg-transparent hover:bg-violet-500 text-violet-700 hover:text-white py-1 px-2 rounded text-xm">{{
                buttonText }}</button>
        </div>
    </div>
</template>
