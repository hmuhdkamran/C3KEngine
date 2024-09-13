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
  <div v-if="isVisible" class="fixed inset-0 flex items-center justify-center z-50">
    <div class="absolute inset-0 bg-gray-500 opacity-75" @click="closeModal"></div>
    <div class="bg-white rounded-lg shadow-lg p-6 relative z-10 w-full max-w-lg">
      <button @click="closeModal" class="absolute top-2 right-2 text-gray-600 hover:text-gray-900">
        <span class="icon-[ph--x-bold] text-xl"></span>
      </button>
      <FormContent :moduleTitle="props.moduleTitle" />
    </div>
  </div>
</template>
