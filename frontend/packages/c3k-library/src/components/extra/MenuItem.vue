<script setup lang="ts">
import type { IMenuItem } from '@/index';
import { Icon } from '@iconify/vue';

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
                class="relative group flex items-center justify-between px-4 py-2 cursor-pointer bg-gray-50 hover:text-violet-700 hover:bg-violet-50 duration-200 ease-in-out">
                <span class="flex items-center space-x-2 relative z-10">
                    <Icon :icon="menuItem.icon" class="h-5 w-5 text-violet-600" />
                    <span class="text-sm font-semibold text-gray-700">{{ menuItem.title }}</span>
                </span>
                <Icon :icon="menuItem.open ? 'mdi:chevron-up' : 'mdi:chevron-down'" class="h-4 w-4 text-gray-600" />
                <div class="custom-border ">
                    <div class="custom-border-line ">
                    </div>
                </div>
            </div>
            <transition name="slide-down">
                <ul v-show="menuItem.open" class="mt-2 pl-6 space-y-2">
                    <li v-for="(child, index) in menuItem.children || []" :key="index">
                        <MenuItem :menuItem="child" />
                    </li>
                </ul>
            </transition>
        </li>
        <router-link v-else :to="menuItem.route"
            class="relative group flex items-center px-4 py-1 text-sm cursor-pointer bg-gray-50 hover:text-violet-700 hover:bg-violet-50 transition-all duration-200 ease-in-out">
            <span>{{ menuItem.title }}</span>
            <div class="custom-border absolute top-0 left-0 bottom-0 w-full overflow-hidden">
                <div
                    class="custom-border-line absolute left-0 top-0 h-full w-0.5 border-r-2 border-violet-600 group-hover:w-full duration-500 ease-in-out">
                </div>
            </div>
        </router-link>
    </transition>
</template>

<style scoped>
.custom-border {
    position: absolute;
    top: 0;
    left: 0;
    bottom: 0;
    width: 100%;
    overflow: hidden;
}

.custom-border-line {
    position: absolute;
    left: 0;
    top: 0;
    height: 100%;
    width: 0.5rem;
    border-right: 2px solid #7c3aed;
    transition: width 0.5s ease-in-out;
}

.group:hover .custom-border-line {
    width: 100%;
}

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

</style>
