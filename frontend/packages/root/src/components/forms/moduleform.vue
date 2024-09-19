<script setup lang="ts">
import { defineProps, defineEmits } from 'vue';
import FormContent from '@/layouts/components/formcontent.vue';

interface Props {
  isVisible: boolean;
  moduleTitle: string;
}

const props = defineProps<Props>();
const emit = defineEmits<{
  (e: 'close'): void;
}>();

const closeModal = () => {
  emit('close');
};
</script>

<template>
  <div v-if="isVisible" class="relative w-full bg-white rounded-lg shadow-md p-6 space-y-4 border border-gray-300">
    <div class="flex justify-between items-center p-2 border-b border-gray-300">
      <button @click="closeModal" class="flex items-center space-x-2 text-gray-600 hover:text-gray-900">
        <span class="icon-[mdi--arrow-left] text-md"></span>
        <span>Back</span>
      </button>
      <button @click="closeModal" class="text-gray-600 hover:text-gray-900">
        <span class="icon-[ph--x-bold] text-md"></span>
      </button>
    </div>
   <FormContent :moduleTitle="props.moduleTitle" />
  </div>
</template>
