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
        class="flex items-center justify-between sticky top-0 z-10">
        <div class="flex items-center justify-between space-x-4">
            <button @click="toggleSidebar" class="text-gray-800 flex items-center justify-center px-6 cursor-pointer">
                <span :class="isSidebarOpen ? 'fa-solid fa-bars' : 'fa-solid fa-bars'"></span>
            </button>
            <BreadCrumbs :breadcrumb="breadcrumbs" />
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