<script setup lang="ts">
import { ref, computed, watch, defineProps, defineEmits, Ref } from 'vue';

const props = defineProps({
  type: {
    type: String,
    default: 'text',
  },
  modelValue: {
    type: String,
    default: '',
  },
  caption: {
    type: String,
    default: '',
  },
  captionPosition: {
    type: String,
    default: 'top',
  },
  placeholder: {
    type: String,
    default: '',
  },
  variant: {
    type: String as () => 'complete' | 'bottom' | 'left' | 'danger' | 'info',
    default: 'complete',
  },
  state: {
    type: String as () => 'primary' | 'danger' | 'info',
    default: 'primary',
  },
  disabled: {
    type: Boolean,
    default: false,
  },
  multiline: {
    type: Boolean,
    default: false,
  },
  rows: {
    type: Number,
    default: 3,
  },
  validators: {
    type: Array as () => Array<(value: unknown) => string | boolean>,
    default: () => [],
  },
});

const emit = defineEmits(['update:modelValue']);

const inputValue = ref(props.modelValue);
const validationMessage: Ref<string | boolean> = ref('');

const validate = () => {
  for (const validator of props.validators) {
    const result = validator(inputValue.value);
    if (result !== true) {
      validationMessage.value = result;
      return false; // Validation failed
    }
  }
  validationMessage.value = ''; // Clear message if valid
  return true; // All validations passed
};

watch(inputValue, (newValue) => {
  emit('update:modelValue', newValue);
  validate(); // Validate whenever input changes
});

const inputClasses = computed(() => {
  const base = 'common-input';
  
  const variants: { [key in 'complete' | 'bottom' | 'left' | 'danger' | 'info']: string } = {
    complete: 'input-complete',
    bottom: 'input-bottom',
    left: 'input-left',
    danger: 'input-danger',
    info: 'input-info',
  };

  const states: { [key in 'primary' | 'danger' | 'info']: string } = {
    primary: 'input-complete-primary',
    danger: 'input-danger',
    info: 'input-info',
  };

  return `${base} ${variants[props.variant]} ${states[props.state]} ${
    props.disabled ? 'input-disabled' : ''
  }`;
});

const wrapperClasses = computed(() => {
  return props.captionPosition === 'left' ? 'flex items-center space-x-2' : 'block';
});

const captionClasses = computed(() => {
  return props.captionPosition === 'left' ? 'mr-2' : 'block mb-1';
});
</script>

<template>
  <div :class="wrapperClasses">
    <label v-if="caption" :class="captionClasses">{{ caption }}</label>
    <textarea
      v-if="multiline"
      v-model="inputValue"
      :rows="rows"
      :class="inputClasses"
      :placeholder="placeholder"
      :disabled="disabled"
    ></textarea>
    <input
      v-else
      :type="type"
      v-model="inputValue"
      :class="inputClasses"
      :placeholder="placeholder"
      :disabled="disabled"
    />
    <p v-if="validationMessage" class="text-red-500">{{ validationMessage }}</p>
  </div>
</template>

<style scoped>
/* Additional styles can go here if needed */
</style>
