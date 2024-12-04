<script setup lang="ts">
import { defineProps } from 'vue';
import { Icon } from '@iconify/vue';
import { TextHelper } from 'c3k-library';

interface CardProps {
    title: string;
    description: string;
    iconClass: string;
}

const props = defineProps<CardProps>();

const { initials } = TextHelper.getInitialsWithColors(props.title);

const truncatedDescription = TextHelper.truncateText(props.description, 100);

const backgroundColor = TextHelper.generateColorFromText(props.title);
</script>

<template>
    <div
        class="card-container relative w-full max-w-sm h-80 sm:h-72 md:h-80 rounded-lg bg-gradient-to-b from-indigo-50 via-white to-purple-50 p-4 shadow-lg hover:translate-y-[-2px] transition-transform duration-300 group overflow-hidden">
        <div class="absolute top-0 left-0 border border-purple-50 w-full h-3/5 pointer-events-none overflow-hidden" :style="{ background: backgroundColor }">
            <div class="absolute w-96 h-96 bg-gradient-to-r from-purple-300 to-indigo-300 rounded-full opacity-30 blur-3xl -top-10 -left-10"></div>
            <div class="absolute w-72 h-72 bg-gradient-to-b from-indigo-300 to-purple-400 rounded-full opacity-20 blur-2xl -bottom-16 -right-8"></div>
            <div class="absolute inset-0 flex items-center justify-center">
                <div class="icon-container mx-auto flex items-center justify-center transition-all duration-300 group-hover:scale-110">
                    <Icon :icon="iconClass" class="text-violet-50 w-48 h-48 transition-all duration-300 group-hover:text-violet-100" />
                </div>
            </div>
        </div>
        <div class="relative z-10 flex flex-col h-full justify-between transition-all duration-500">
            <span
                class="absolute z-30 rounded-full text-gray-200 font-bold text-sm shadow-sm flex items-center justify-center"
                :style="{ background: backgroundColor, width: '50px', height: '50px' }">
                {{ initials }}
            </span>
            <div class="mt-auto mb-4 text-left h-24 flex flex-col justify-end overflow-hidden">
                <h3 class="font-semibold text-lg md:text-md text-gray-800 transition-colors duration-300">
                    {{ title }}
                </h3>
                <p class="text-gray-600 text-sm sm:text-xs mt-2 leading-relaxed group-hover:text-gray-800">
                    {{ truncatedDescription }}
                </p>
            </div>
        </div>
    </div>
</template>

<style scoped>
.card-container {
    border: 1px solid #e2e8f0;
}

.card-container:hover {
    box-shadow: 0 10px 20px -5px rgba(79, 70, 229, 0.3);
    transform: translateY(-2px);
}

.icon-container {
    transition: transform 0.4s ease-in-out, box-shadow 0.3s ease;
}

.icon-container:hover {
    box-shadow: 0 4px 8px rgba(99, 102, 241, 0.3);
}
.card-container:hover .icon-container {
    transform: translateY(-4px);
}
</style>