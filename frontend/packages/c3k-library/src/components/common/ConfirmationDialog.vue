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
    <div class="bg-white rounded-md shadow-lg w-96 overflow-hidden">
      <!-- Title with Primary Background -->
      <h3 class="text-gray-700 font-semibold text-md p-3">
        {{ props.title }}
      </h3>
      <!-- Message -->
      <div class="p-6">
        <p class="text-gray-700 text-sm">{{ props.message }}</p>
        <slot />
      </div>
      <!-- Action Buttons -->
      <div class="bg-gray-100 p-3 flex justify-end space-x-2">
        <button class="px-3 py-1 text-sm btn-secondary"
          @click="emit('cancel')">Cancel</button>
        <button class="px-3 py-1 text-sm btn-danger"
          @click="emit('confirm')">Delete</button>
      </div>
    </div>
  </div>
</template>