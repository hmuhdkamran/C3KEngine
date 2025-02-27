<script setup lang="ts">
import { ref } from 'vue';
import { useSidebar } from '@/stores/useSidebar';
import { useSystemStore } from 'c3-library';

defineProps({ items: Object })

const { user } = useSystemStore();

const { isSidebarOpen } = useSidebar();
const emit = defineEmits(['toggle-tooltip']);
const isTooltipListOpen = ref(false);

const toggleTooltip = () => {
    if (!isSidebarOpen) {
        isTooltipListOpen.value = !isTooltipListOpen.value;
        emit('toggle-tooltip');
    }
};
</script>

<template>
    <div class="user-profile flex items-center mb-4 transition-all">
        <div class="relative">
            <img :src="user.displayName" alt="User" class="avatar rounded-full mr-2 transition-transform"
                :class="{ 'w-10 h-10': isSidebarOpen, 'w-8 h-8': !isSidebarOpen }" @click="toggleTooltip" />

            <div v-if="!isSidebarOpen && isTooltipListOpen" class="tooltip-list">
                <ul class="py-1" role="menu" aria-orientation="vertical">
                    <li v-for="(item, index) in items" :key="index" class="tooltip-item">
                        <router-link :to="item.link" class="flex items-center w-full">
                            <span :class="item.icon" class="mr-3 fa-md" style="color: var(--primary-color)"></span>
                            {{ item.label }}
                        </router-link>
                    </li>
                </ul>
            </div>
        </div>

        <div v-if="isSidebarOpen" class="user-details">
            <h3 class="text-sm font-bold">{{ user.displayName }}</h3>
            <p class="text-sm">{{ user.email }}</p>
        </div>
    </div>
</template>

<style scoped>
.user-profile {
    transition-duration: 500ms;
    transition-timing-function: cubic-bezier(0.4, 0.0, 0.2, 1);
}

.avatar:not(.w-10) {
    cursor: pointer;
}

.avatar:not(.w-10):hover {
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1),
        0 2px 4px -1px rgba(0, 0, 0, 0.06);
    transform: translateY(-2px);
}

.tooltip-list {
    position: absolute;
    left: 100%;
    bottom: 4rem;
    /* Tailwind's bottom-16 (4rem) */
    width: 12rem;
    /* Tailwind's w-48 (12rem) */
    padding: 0.25rem;
    /* Tailwind's p-1 (0.25rem) */
    background-color: #ffffff;
    border-radius: 0.375rem;
    /* Tailwind's rounded-md (0.375rem) */
    box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1),
        0 4px 6px -2px rgba(0, 0, 0, 0.05);
    /* Tailwind's shadow-lg */
    overflow: hidden;
    z-index: 50;
}

.tooltip-item {
    padding-left: 1rem;
    /* Tailwind's px-4 */
    padding-right: 1rem;
    padding-top: 0.5rem;
    /* Tailwind's py-2 */
    padding-bottom: 0.5rem;
    font-size: 0.875rem;
    /* Tailwind's text-sm */
    color: #374151;
    /* Tailwind's text-gray-700 */
    transition: all 500ms cubic-bezier(0.4, 0.0, 0.2, 1);
}

.user-details {
    flex: 1 1 0%;
    display: none;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

@media (min-width: 768px) {
    .user-details {
        display: block;
    }
}
</style>