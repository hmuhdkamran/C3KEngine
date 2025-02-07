<script setup lang="ts">
import { ref, watch } from "vue";
import { useRoute } from "vue-router";
import Header from "./auth/header.vue";
import Sidebar from "./auth/sidebar.vue";
import Breadcrumb from "../breadcrumbs.vue";
import ColorPalette from "../color.vue";
import { menuItems } from '@/stores/menuData';
import Filter from "../filter.vue";
import { setFormOpen } from "@/stores/edit-form";

interface Breadcrumb {
  name: string;
  link?: string;
  icon?: string;
}

const breadcrumbs = ref<Breadcrumb[]>([]);
const route = useRoute();

const generateBreadcrumbs = () => {
  breadcrumbs.value = [];

  const parentMenu = menuItems.find((menu) =>
    menu.children.some((child) => child.link === route.path)
  );

  const childMenu = parentMenu?.children.find((child) => child.link === route.path);

  if (parentMenu) {
    breadcrumbs.value.push({
      name: parentMenu.name,
      link: parentMenu.link || undefined,
    });
  }

  if (childMenu) {
    breadcrumbs.value.push({
      name: childMenu.name,
      link: childMenu.link,
    });
  }
};

watch(() => route.path, generateBreadcrumbs, { immediate: true });

</script>

<template>
  <div class="flex flex-col h-screen">
    <Header />
    <div class="flex flex-grow">
      <Sidebar />
      <div class="flex flex-col flex-grow relative">
        <div class="flex items-center justify-between p-4">
          <Breadcrumb :breadcrumbs="breadcrumbs" />
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