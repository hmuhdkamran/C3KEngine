<script setup lang="ts">
import { UseAxios } from './plugins/axios';

UseAxios();
</script>

<template>
  <div class="bg-gray-50 font-inter tracking-tight text-gray-900 antialiased">
    <router-view />
  </div>
</template>

<style scoped></style>
