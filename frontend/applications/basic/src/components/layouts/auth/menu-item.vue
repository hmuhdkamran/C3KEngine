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

// Get sidebar state from composable
const { isSidebarOpen } = useSidebar();

const primaryColor = computed(() => store.application.primaryColor);
const hasChildren = computed(() => item.value.children && item.value.children.length > 0);

// Handle item click based on sidebar state
const handleItemClick = () => {
    if (isSidebarOpen.value) {
        item.value.open = !item.value.open;
    } else if (hasChildren.value) {
        // If sidebar is closed and item has children, we don't toggle open state
        // Child items will be shown in the popup on hover
    } else {
        // For leaf items without children, we may want to navigate even when sidebar is closed
    }
};
</script>

<template>
    <!-- Parent menu item with children -->
    <li v-if="hasChildren" class="menu-parent text-gray-700 relative group">
        <div class="menu-parent-header flex items-center cursor-pointer rounded-sm"
            :class="{ 'justify-center': !isSidebarOpen, 'px-4 py-2': isSidebarOpen }" @click="handleItemClick">
            <!-- Icon always visible -->
            <span class="menu-icon" :class="{ 'h-5 w-5': isSidebarOpen, 'h-6 w-6 mx-auto': !isSidebarOpen }">
                <i :class="item.icon" />
            </span>

            <!-- Text only visible when sidebar is open -->
            <span v-if="isSidebarOpen" class="menu-title ml-4 font-medium">{{ item.name }}</span>

            <!-- Arrow only visible when sidebar is open -->
            <span v-if="isSidebarOpen" :class="item.open ? 'fa-solid fa-angle-up' : 'fa-solid fa-angle-down'"
                class="ml-auto">
            </span>
        </div>

        <!-- Children for expanded sidebar -->
        <transition name="dropdown" v-if="isSidebarOpen">
            <ul class="menu-children ml-5" v-show="item.open">
                <menu-item v-for="child of item.children" :key="child.name" :item="child" />
            </ul>
        </transition>

        <!-- Popup menu for collapsed sidebar -->
        <div v-if="!isSidebarOpen"
            class="popup-menu absolute left-full top-0 bg-white shadow-md rounded-md z-50 hidden group-hover:block"
            style="min-width: 200px;">
            <div class="popup-header px-4 py-2 border-b border-gray-100 font-medium">
                {{ item.name }}
            </div>
            <ul class="py-2">
                <li v-for="child of item.children" :key="child.name" class="px-4 py-2 hover:bg-gray-50">
                    <router-link :to="child.route" class="flex items-center gap-2">
                        <span :class="child.icon" class="menu-icon h-4 w-4 flex-shrink-0"></span>
                        <span>{{ child.title }}</span>
                    </router-link>
                </li>
            </ul>
        </div>
    </li>

    <!-- Leaf menu item (no children) -->
    <router-link v-else :to="item.route" class="menu-item flex items-center rounded-sm"
        :class="{ 'justify-center py-3': !isSidebarOpen, 'gap-2 px-4 py-2': isSidebarOpen }">
        <!-- Icon always visible -->
        <span class="menu-icon" :class="{ 'h-4 w-4 flex-shrink-0': isSidebarOpen, 'h-6 w-6 mx-auto': !isSidebarOpen }">
            <i :class="item.icon" />
        </span>

        <!-- Title only visible when sidebar is open -->
        <span v-if="isSidebarOpen" class="menu-title">{{ item.title }}</span>

        <!-- Tooltip for collapsed sidebar -->
        <span v-if="!isSidebarOpen" class="tooltip absolute left-full ml-2 px-2 py-1 bg-gray-800 text-white text-xs rounded 
                    opacity-0 invisible group-hover:visible group-hover:opacity-100 whitespace-nowrap">
            {{ item.title }}
        </span>
    </router-link>
</template>

<style scoped>
/* Define CSS variables from store at component level */
:root {
    --primary-color: v-bind('primaryColor');
    --primary-hover: v-bind('primaryColor + "99"');
    /* 60% transparent */
    --primary-selected: v-bind('primaryColor + "80"');
    /* 50% transparent */
}

/* Menu icon styling */
.menu-icon {
    color: var(--primary-color);
}

/* Menu item hover state */
.menu-item:hover,
.menu-parent-header:hover {
    background-color: var(--primary-hover);
    color: var(--primary-color);
}

/* Selected state */
.router-link-active {
    background-color: var(--primary-selected);
    color: var(--primary-color);
}

/* Dropdown animation */
.dropdown-enter-active,
.dropdown-leave-active {
    transition: all 0.3s ease;
}

.dropdown-enter-from,
.dropdown-leave-to {
    opacity: 0;
    transform: translateY(-10px);
}

/* Consistent alignment between parent and child items */
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

/* Popup menu styling */
.popup-menu {
    transition: opacity 0.2s, visibility 0.2s;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

/* Tooltip styling */
.tooltip {
    transition: opacity 0.2s, visibility 0.2s;
    z-index: 50;
}

/* Parent items need to be relative for proper popup positioning */
.menu-parent {
    position: relative;
}
</style>