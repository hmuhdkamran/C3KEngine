<script setup lang="ts">
import type { IMenuItem } from '@/index';

interface Props {
    menuItem: IMenuItem;
}
const props = defineProps<Props>();

const toggleMenu = (menu: IMenuItem) => {
    menu.open = !menu.open;
};

</script>

<template>
    <transition name="slide-down">
        <li v-if="menuItem.children">
            <div @click.prevent="toggleMenu(props.menuItem)"
                class="relative group flex items-center justify-between px-3 py-2 cursor-pointer bg-gray-50 hover:text-violet-700 hover:bg-violet-50 transition-all duration-200 ease-in-out">
                <span class="flex items-center space-x-2 relative z-10">
                    <span :class="`${menuItem.icon} h-5 w-5 text-violet-600`"></span>
                    <span class="text-sm font-semibold text-gray-700">{{ menuItem.title }} {{ menuItem.open }}</span>
                </span>
                <span :class="menuItem.open
                    ? 'icon-[mdi--chevron-up] text-gray-600'
                    : 'icon-[mdi--chevron-down] text-gray-500'"></span>
                <div class="absolute top-0 left-0 bottom-0 bg-transparent overflow-hidden z-0 w-full">
                    <div
                        class="absolute top-0 left-0 h-full w-0.5 border-r-2 border-violet-600 transform group-hover:w-full transition-all duration-500 ease-in-out">
                    </div>
                </div>
            </div>
            <transition name="slide-down">
                <ul v-show="menuItem.open" class="mt-2 pl-4 space-y-1">
                    <li v-for="(child, index) in menuItem.children || []" :key="index">
                        <MenuItem :menuItem="child" />
                    </li>
                </ul>
            </transition>
        </li>
        <router-link v-else :to="menuItem.route"
            class="relative group flex items-center px-2 py-1 text-sm cursor-pointer bg-gray-50 hover:text-violet-700 hover:bg-violet-50 transition-all duration-200 ease-in-out">
            <span>{{ menuItem.title }}</span>
            <div class="absolute top-0 left-0 bottom-0 bg-transparent overflow-hidden z-0 w-full">
                <div
                    class="absolute top-0 left-0 h-full w-0.5 border-r-2 border-violet-600 transform group-hover:w-full transition-all duration-500 ease-in-out">
                </div>
            </div>
        </router-link>
    </transition>
</template>

<style scoped>
/* Slide-down transition for nested menus */
.slide-down-enter-active,
.slide-down-leave-active {
    transition: max-height 0.3s ease, opacity 0.3s ease;
}

.slide-down-enter-from,
.slide-down-leave-to {
    max-height: 0;
    opacity: 0;
}

.slide-down-enter-to,
.slide-down-leave-from {
    max-height: 200px;
    /* Adjust based on content size */
    opacity: 1;
}

/* Slide transition for individual list items */
.slide-enter-active,
.slide-leave-active {
    transition: transform 0.4s ease;
}

.slide-enter-from,
.slide-leave-to {
    transform: translateX(-100%);
}

/* Fade transition for hover effects */
.fade-enter-active,
.fade-leave-active {
    transition: opacity 0.3s ease;
}
</style>
