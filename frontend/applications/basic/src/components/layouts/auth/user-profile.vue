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
    if (!isSidebarOpen.value) {
        isTooltipListOpen.value = !isTooltipListOpen.value;
    }
};
</script>

<template>
    <div class="user-profile flex items-center mb-4 transition-all">
        <div class="relative">
            <div class="avatar shadow-md rounded-full mr-2 transition-transform flex items-center justify-center"
                :class="{ 'w-10 h-10': isSidebarOpen, 'w-8 h-8 cursor-pointer shadow-md hover:translate-y-[-2px] transition-transform duration-300': !isSidebarOpen }"
                @click="toggleTooltip">
                <i class="fa-regular fa-user"></i>
            </div>

            <div v-if="!isSidebarOpen && isTooltipListOpen"
                class="absolute left-full bottom-2 w-48 p-2 bg-white rounded-md shadow-lg overflow-hidden z-50">
                <ul role="menu" aria-orientation="vertical">
                    <li v-for="(item, index) in items" :key="index"
                        class="tooltip-item text-sm text-gray-700 transition-all rounded-sm duration-500 ease-[cubic-bezier(0.4, 0.0, 0.2, 1)]"
                        >
                        <router-link :to="item.link" class="flex items-center w-full p-2">
                            <span :class="item.icon" class="mr-3 fa-md" style="color: var(--primary-color)"></span>
                            {{ item.label }}
                        </router-link>
                    </li>
                </ul>
            </div>
        </div>

        <div v-if="isSidebarOpen" class="flex-1 hidden whitespace-nowrap overflow-hidden text-ellipsis md:block">
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

.avatar {
    border: 2px solid var(--primary-color);
}

.tooltip-item:hover {
    background-color: color-mix(in srgb, var(--primary-color) 10%, transparent);
}
</style>