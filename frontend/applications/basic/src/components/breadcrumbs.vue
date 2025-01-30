<script setup lang="ts">
import { defineProps } from "vue";
import { RouterLink } from "vue-router";

defineProps<{ breadcrumbs: { name: string; link?: string; }[] }>();
</script>

<template>
  <nav class="px-6 py-3">
    <ul class="flex items-center">
      <li v-for="(breadcrumb, index) in breadcrumbs" :key="index" class="flex items-center">
        <div class="flex items-center space-x-2">
          <RouterLink v-if="breadcrumb.link" :to="breadcrumb.link" class="text-gray-600 text-md">
            {{ breadcrumb.name }}
          </RouterLink>
          <span v-else class="text-gray-500 text-md">{{ breadcrumb.name }}</span>
        </div>
        <span v-if="index !== breadcrumbs.length - 1" class="mx-2 text-gray-400">/</span>
      </li>
    </ul>
  </nav>
</template>
