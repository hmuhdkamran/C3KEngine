<script lang="ts" setup>
import Header from "./header.vue";
import Sidebar from "./sidebar.vue";

</script>

<template>
  <div class="flex flex-col h-screen">
    <Header />
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
