<script setup lang="ts">
import { ref, computed, defineProps, defineEmits } from 'vue';

const props = defineProps({
  modelValue: {
    type: [String, Object], // Can be a string or an object
    default: '',
  },
  caption: {
    type: String,
    default: '',
  },
  captionPosition: {
    type: String, // 'top' or 'left'
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
  options: {
    type: Array as () => Array<any>,
    default: () => [],
  },
  displayValue: {
    type: String,
    required: true,
  },
  allowCustomValue: {
    type: Boolean,
    default: false,
  },
});

const emit = defineEmits(['update:modelValue', 'selectionChanged']);

const inputValue = ref(props.modelValue);

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

// Update value and emit events
const updateValue = (value: any) => {
  emit('update:modelValue', value);
  inputValue.value = value;
  emit('selectionChanged', value); // Emit selection changed event
};

// Method to handle selection changes
const handleSelectionChange = (event: Event) => {
  const selectedValue = (event.target as HTMLSelectElement)?.value;
  const selectedOption = props.options.find(
    (option) => option[props.displayValue] === selectedValue
  );
  updateValue(selectedOption || selectedValue);
};

</script>

<template>
  <div :class="wrapperClasses">
    <label v-if="caption" :class="captionClasses">{{ caption }}</label>

    <!-- ComboBox allowing both dropdown and custom value -->
    <div v-if="allowCustomValue">
      <input
        type="text"
        v-model="inputValue"
        :class="inputClasses"
        :placeholder="placeholder"
        :disabled="disabled"
        @input="updateValue(inputValue)"
        list="combo-options"
      />
      <datalist id="combo-options">
        <option
          v-for="option in options"
          :key="option[props.displayValue]"
          :value="option[props.displayValue]"
        >
          {{ option[props.displayValue] }}
        </option>
      </datalist>
    </div>

    <!-- Select box for dropdown -->
    <select
      v-else
      :class="inputClasses"
      :disabled="disabled"
      @change="handleSelectionChange"
    >
      <option :value="''" disabled hidden>{{ placeholder }}</option>
      <option
        v-for="option in options"
        :key="option[props.displayValue]"
        :value="option[props.displayValue]"
      >
        {{ option[props.displayValue] }}
      </option>
    </select>
  </div>
</template>

<style scoped>
/* Additional styles can go here if needed */
</style>
