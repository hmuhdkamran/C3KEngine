<script setup lang="ts">
import { useRoute, type RouteMeta } from "vue-router";
import { useI18n } from "vue-i18n";
import { BreadCrumbs } from "c3-library";

import Header from "./auth/header.vue";
import Sidebar from "./auth/sidebar.vue";
import ColorPalette from "../color.vue";
import Filter from "../filter.vue";
import { ref, watch } from "vue";

const route = useRoute();
const { t } = useI18n()

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
  <div class="flex h-screen">
    <Sidebar />
    <div class="flex flex-col flex-grow">
      <Header />
      <div class="flex flex-col flex-grow relative">
        <div class="flex items-center justify-end p-4">
          <!-- <BreadCrumbs :breadcrumb="breadcrumbs" /> -->
          <Filter />
        </div>
        <div class="flex-grow p-4 bg-gray-100">
          <RouterView v-slot="{ Component }">
            <Transition name="fade" mode="out-in">
              <Component :is="Component" />
            </Transition>
          </RouterView>
        </div>
        <div class="fixed right-0 top-1/2 transform -translate-y-1/2">
          <ColorPalette />
        </div>
      </div>
    </div>
  </div>
</template>