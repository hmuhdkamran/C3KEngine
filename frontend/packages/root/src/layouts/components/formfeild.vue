<script setup lang="ts">
import { defineProps, computed } from 'vue';

interface FormFieldProps {
  field: {
    label: string;
    type: string;
    options?: string[];
  };
}

const props = defineProps<FormFieldProps>();

const componentType = computed(() => {
  switch (props.field.type) {
    case 'text':
    case 'number':
    case 'date':
    case 'time':
      return 'input';
    case 'select':
      return 'select';
    default:
      return 'input';
  }
});

const inputAttributes = computed(() => {
  return {
    type: props.field.type,
  };
});
</script>

<template>
  <div class="mb-4">
    <label class="block text-gray-700">{{ props.field.label }}</label>
    <component
      :is="componentType"
      v-bind="inputAttributes"
      class="input-complete"
      :placeholder="`Enter ${props.field.label.toLowerCase()}...`"
    >
      <template v-if="props.field.type === 'select'">
        <option v-for="option in props.field.options" :key="option" :value="option">
          {{ option }}
        </option>
      </template>
    </component>
  </div>
</template>
