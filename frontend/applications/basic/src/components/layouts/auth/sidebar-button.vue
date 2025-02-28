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
    showOnCollapsed: {
        type: Boolean,
        default: false
    }
});

const { isSidebarOpen } = useSidebar();
const emit = defineEmits(['click']);
</script>

<template>
    <div v-if="isSidebarOpen || (showOnCollapsed && !isSidebarOpen)"
        class="sidebar-button group relative transition-all">
        <button
            class="btn-action cursor-pointer hover:translate-y-[-2px] transition-transform duration-300 w-7 h-7 text-white rounded-full flex items-center justify-center shadow-md border"
            @click="emit('click')">
            <span :class="`fa-solid ${icon} fa-sm`"></span>
        </button>
        <span
            class="tooltip text-xs absolute bottom-full mb-1 left-1/2 transform -translate-x-1/2 whitespace-nowrap bg-gray-700 text-white px-2 py-1 rounded-md">
            {{ tooltip }}
        </span>
    </div>
</template>

<style scoped>
.btn-action {
    border-color: var(--primary-color);
    background-color: color-mix(in srgb, var(--primary-color) 80%, transparent);
}

.tooltip {
    display: none;
    transform: scale(0.9);
    opacity: 0;
    transition: all 0.3s ease-in-out;
}

.group:hover .tooltip {
    display: block;
    transform: scale(1);
    opacity: 1;
}
</style>