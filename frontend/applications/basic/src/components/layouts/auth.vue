<script lang="ts" setup>
import Header from "./auth/header.vue";
import Sidebar from "./auth/sidebar.vue";
import ColorPalette from "../color.vue";
</script>

<template>
  <div class="flex flex-col h-screen">
    <Header />
    <ColorPalette />
    <div class="flex flex-grow">
      <Sidebar />
      <RouterView v-slot="{ Component }">
        <Transition name="fade" mode="out-in">
          <Component :is="Component" />
        </Transition>
      </RouterView>
    </div>
  </div>
</template>
