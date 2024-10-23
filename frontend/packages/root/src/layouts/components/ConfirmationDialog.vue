<script setup lang="ts">
import { defineProps, defineEmits } from 'vue';
interface Props {
  isVisible: boolean;
  message: string;
  title: string;
}

interface Emit {
  (e: "cancel"): void;
  (e: "confirm"): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emit>();

</script>

<template>
  <div v-if="props.isVisible" class="fixed inset-0 flex items-center justify-center bg-black bg-opacity-50 z-40">
    <div class="bg-white p-4 rounded shadow-lg w-96">
      <h3 class="text-xl font-semibold">{{ title }}</h3>
      <p class="mt-2">{{ message }}</p>
      <div class="mt-6 flex justify-end space-x-2">
        <button class="px-4 py-2 btn-secondary" @click="emit('cancel')">Cancel</button>
        <button class="px-4 py-2 btn-danger" @click="emit('confirm')">Delete</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
</style>