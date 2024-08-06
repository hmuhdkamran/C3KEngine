<script setup lang="ts">
import { defineProps } from 'vue';

interface CardProps {
    title: string;
    description: string;
    status: string;
    buttonText: string;
    iconClass: string;
}

defineProps<CardProps>();
</script>
<template>
    <a href="/app/dashboard"
        class="w-80 max-w-sm rounded overflow-hidden shadow-lg bg-white transition transform hover:scale-105 duration-200 sm:max-w-full lg:max-w-sm">
        <div class="px-2 py-1 flex flex-col sm:flex-row items-start sm:items-center gap-3">
            <div class="relative w-20 h-20 bg-violet-200 rounded-xl">
                <span
                    :class="`${iconClass} absolute w-9 h-9 text-violet-700 transform -translate-x-1/2 -translate-y-1/2 left-1/2 top-1/2`"></span>
            </div>
            <div class="flex-1">
                <div class="font-bold text-xl mb-2 flex items-center justify-between">
                    <span>{{ title }}</span>
                    <span class="icon-[ph--dots-three-vertical-bold] ml-auto"></span>
                </div>
                <p class="text-gray-700">
                    <span>{{ description }}</span>
                </p>
                <p class="text-green-500 text-sm mt-3 flex items-center justify-between">
                    <span>{{ status }}</span>
                    <button
                        class="bg-transparent hover:bg-violet-500 text-violet-700 hover:text-white py-1 px-2 rounded ml-auto">{{
                        buttonText }}</button>
                </p>
            </div>
        </div>
    </a>
</template>
