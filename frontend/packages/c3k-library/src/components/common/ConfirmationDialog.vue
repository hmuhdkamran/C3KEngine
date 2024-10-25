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
  <div v-if="props.isVisible" class="fixed inset-0 flex items-center justify-center bg-black bg-opacity-50 z-50">
    <div class="bg-white rounded-lg shadow-lg w-96 overflow-hidden">
      <!-- Title with Primary Background -->
      <h3 class="text-white font-semibold text-lg p-4 bg-blue-600">
        {{ props.title }}
      </h3>
      <!-- Message -->
      <div class="p-6">
        <p class="text-gray-700 text-sm">{{ props.message }}</p>
        <slot />
      </div>
      <!-- Action Buttons -->
      <div class="bg-gray-100 px-6 py-4 flex justify-end space-x-3">
        <button class="px-4 py-2 text-sm font-semibold text-gray-600 bg-gray-200 rounded hover:bg-gray-300 transition"
          @click="emit('cancel')">Cancel</button>
        <button class="px-4 py-2 text-sm font-semibold text-white bg-red-600 rounded hover:bg-red-700 transition"
          @click="emit('confirm')">Delete</button>
      </div>
    </div>
  </div>
</template>