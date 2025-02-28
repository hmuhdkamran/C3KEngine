<script setup lang="ts">
import { useSidebar } from '@/stores/useSidebar';
import { useSystemStore, type IMenuItem } from 'c3-library';
import { computed, defineProps, ref, type Ref } from 'vue';

interface Item {
    item: IMenuItem
}

const store = useSystemStore();
const props = defineProps<Item>();
const item: Ref<IMenuItem> = ref(props.item);

const { isSidebarOpen } = useSidebar();

const primaryColor = computed(() => store.application.primaryColor);
const hasChildren = computed(() => item.value.children && item.value.children.length > 0);

const handleItemClick = () => {
    if (isSidebarOpen.value) {
        item.value.open = !item.value.open;
    } else if (hasChildren.value) {

    } else {

    }
};
</script>

<template>
    <li v-if="hasChildren" class="menu-parent text-gray-700 relative group px-2">
        <div class="menu-parent-header p-2 flex items-center cursor-pointer rounded-sm" @click="handleItemClick">
            <span class="menu-icon" :class="{ 'h-5 w-5': isSidebarOpen, 'h-6 w-6': !isSidebarOpen }">
                <i :class="item.icon" />
            </span>
            <span v-if="isSidebarOpen" class="menu-title ml-4 font-medium">{{ item.name }}</span>
            <span v-if="isSidebarOpen" :class="item.open ? 'fa-solid fa-angle-up' : 'fa-solid fa-angle-down'"
                class="ml-auto">
            </span>
        </div>
        <transition name="dropdown" v-if="isSidebarOpen">
            <ul class="menu-children ml-1" v-show="item.open">
                <menu-item v-for="child of item.children" :key="child.name" :item="child" />
            </ul>
        </transition>
        <div v-if="!isSidebarOpen"
            class="popup-menu absolute left-full top-0 bg-white shadow-md rounded-md z-50 hidden group-hover:block"
            style="min-width: 200px;">
            <div class="popup-header px-4 py-2 border-b border-gray-100 font-medium">
                {{ item.name }}
            </div>
            <ul class="p-2">
                <li v-for="child of item.children" :key="child.name">
                    <router-link :to="child.route" class="flex items-center bghover p-2 gap-2">
                        <span :class="child.icon" class="menu-icon h-4 w-4 flex-shrink-0"></span>
                        <span>{{ child.title }}</span>
                    </router-link>
                </li>
            </ul>
        </div>
    </li>

    <div v-else class="px-2">
        <router-link :to="item.route" class="menu-parent-header p-2 flex items-center cursor-pointer rounded-sm">
            <span class="menu-icon" :class="{ 'h-5 w-5': isSidebarOpen, 'h-6 w-6': !isSidebarOpen }">
                <i :class="item.icon" />
            </span>
            <span v-if="isSidebarOpen" class="menu-title">{{ item.title }}</span>
            <span v-if="!isSidebarOpen" class="tooltip absolute left-full ml-2 px-2 py-1 bg-gray-800 text-white text-xs rounded 
                opacity-0 invisible group-hover:visible group-hover:opacity-100 whitespace-nowrap">
                {{ item.title }}
            </span>
        </router-link>
    </div>
</template>

<style scoped>
:root {
    --primary-color: v-bind('primaryColor');
    --primary-hover: v-bind('primaryColor + "99"');
    --primary-selected: v-bind('primaryColor + "80"');
}

.menu-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--primary-color);
}

.bghover:hover {
    background-color: color-mix(in srgb, var(--primary-color) 10%, transparent);
}

.menu-item:hover,
.menu-parent-header:hover {
    background-color: color-mix(in srgb, var(--primary-color) 10%, transparent);
    color: var(--primary-color);
}

.router-link-active {
    background-color: var(--primary-selected);
    color: var(--primary-color);
}

.dropdown-enter-active,
.dropdown-leave-active {
    transition: all 0.3s ease;
}

.dropdown-enter-from,
.dropdown-leave-to {
    opacity: 0;
    transform: translateY(-10px);
}

.menu-parent-header,
.menu-item {
    display: flex;
    align-items: center;
    width: 100%;
}

.menu-title {
    margin-left: 0.5rem;
}

.menu-children {
    padding-left: 0.5rem;
}

.popup-menu {
    transition: opacity 0.2s, visibility 0.2s;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.tooltip {
    transition: opacity 0.2s, visibility 0.2s;
    z-index: 50;
}

.menu-parent {
    position: relative;
}
</style>