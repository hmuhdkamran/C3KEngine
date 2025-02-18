<script setup lang="ts">
import { useRoute, type RouteMeta } from "vue-router";
import { BreadCrumbs, useSystemStore } from "c3-library";
import { useSidebar } from '@/stores/useSidebar';
import { ref, watch } from "vue";
import { useI18n } from "vue-i18n";

const route = useRoute();

const { isSidebarOpen } = useSidebar();

const store = useSystemStore();
const { t } = useI18n()

const toggleSidebar = () => {
    isSidebarOpen.value = !isSidebarOpen.value;
};

const breadcrumbs = ref<string[]>([]);
interface CustomRouteMeta extends RouteMeta {
  breadcrumb?: string[];
}

function updateTranslatedBreadcrumbs() {
  const breadcrumbKeys = (route.meta as CustomRouteMeta)?.breadcrumb || [];
  breadcrumbs.value = breadcrumbKeys.map((key) => t(`menu.${key}`));
}

watch(() => route.path, () => updateTranslatedBreadcrumbs(), { immediate: true });

</script>

<template>
    <div :style="{ backgroundColor: store.application.titleColor }"
        class=" p-2 flex items-center justify-between sticky top-0 z-10">
        <div class="flex items-center justify-between space-x-4">
            <!-- <div class="flex items-center space-x-2">
                <img :src="logo" alt="Logo" class="h-8 w-8" />
                <h2 v-if="isSidebarOpen" class="hidden md:block text-md font-semibold text-gray-50 whitespace-nowrap overflow-hidden text-ellipsis">
                    Ultimate ERP Solution
                </h2>
            </div> -->

            <button @click="toggleSidebar" class="text-gray-800 flex items-center justify-center p-2 cursor-pointer">
                <span :class="isSidebarOpen ? 'fa-solid fa-bars' : 'fa-solid fa-bars'"></span>
            </button>
            <BreadCrumbs :breadcrumb="breadcrumbs" />

            <!-- <div class="flex items-center text-gray-50 relative flex-1">
                <button class=" transition duration-200 absolute flex items-center justify-center w-8 h-8">
                    <span class="fa-solid fa-magnifying-glass fa-sm"></span>
                </button>
                <input type="text" placeholder="Type for Search..."
                    class="pl-9 py-1 text-gray-50 w-full text-sm bg-transparent hover:bg-gray-50/10 rounded-sm focus:outline-none focus:ring-1 focus:ring-gray-500 transition duration-200" />
            </div> -->
        </div>

        <!-- <div class="flex items-center space-x-4">
            <div class="relative">
                <button class="relative text-gray-50 hover:text-gray-100 transition duration-200">
                    <span class="fa-solid fa-bell"></span>
                    <div class="absolute top-0 right-0 h-2 w-2 bg-red-400 rounded-full"></div>
                </button>
            </div>

            <button class="text-gray-50 hover:text-gray-100 transition duration-200">
                <span class="fa-solid fa-message"></span>
            </button>

            <button class="text-gray-50 hover:text-gray-100 transition duration-200">
                <span class="fa-solid fa-user"></span>
            </button>

            <button class="text-gray-50 hover:text-gray-100 transition duration-200">
                <span class="fa-solid fa-gear"></span>
            </button>
        </div> -->
    </div>
</template>

<style scoped>
@media (max-width: 768px) {
    .hidden-md {
        display: none;
    }
}
</style>