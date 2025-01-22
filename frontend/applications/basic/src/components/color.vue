<script setup lang="ts">
import { useColorPalette } from '@/stores/colorPalette';

const { state, selectColor } = useColorPalette();
</script>

<template>
  <div class="flex space-x-2 p-4">
    <div
      v-for="color in state.colors"
      :key="color"
      :style="{ backgroundColor: color }"
      class="w-8 h-8 rounded-full cursor-pointer"
      @click="selectColor(color)" 
    ></div>
  </div>
</template>
