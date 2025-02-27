<script setup lang="ts">
import { useSidebar } from '@/stores/useSidebar';

defineProps({
    icon: {
        type: String,
        required: true
    },
    tooltip: {
        type: String,
        required: true
    },
});

const { isSidebarOpen } = useSidebar();
const emit = defineEmits(['click']);
</script>

<template>
    <div v-if="isSidebarOpen" class="sidebar-button group relative transition-all">
        <button
            class="btn-action cursor-pointer w-7 h-7 text-white rounded-full flex items-center justify-center shadow-md border"
            @click="emit('click')">
            <span :class="`fa-solid ${icon} fa-sm`"></span>
        </button>
        <span class="btn-tooltip">
            {{ tooltip }}
        </span>
    </div>
</template>

<style scoped>
.btn-action {
    border-color: var(--primary-color);
    background-color: color-mix(in srgb, var(--primary-color) 80%, transparent);
    transition: transform 300ms;
}

.btn-action:hover {
    transform: translateY(-2px);
}

.btn-tooltip {
    position: absolute;
    bottom: 100%;
    left: 50%;
    transform: translateX(-50%);
    white-space: nowrap;
    padding-left: 0.5rem;
    padding-right: 0.5rem;
    padding-top: 0.25rem;
    padding-bottom: 0.25rem;
    opacity: 0;
    transition: opacity 200ms;
}

.group:hover .btn-tooltip {
    opacity: 1;
}
</style>