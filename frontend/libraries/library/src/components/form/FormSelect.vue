<script setup lang="ts">
import { ref, watch, defineProps, defineEmits } from 'vue';

// Validator type: a valid result returns true; otherwise it returns an error message string.
export type Validator = (value: unknown) => true | string;

const props = defineProps<{
    id: string;
    label: string;
    modelValue: string;
    // Options can be an array of objects or primitives.
    options: Array<Record<string, any> | string>;
    // Keys to extract the option value and label.
    valueKey?: string;
    labelKey?: string;
    validators?: Validator[];
}>();

const emits = defineEmits<{
    (e: 'update:modelValue', value: string): void;
}>();

// Use default keys if not provided.
const defaultValueKey = props.valueKey || 'value';
const defaultLabelKey = props.labelKey || 'label';

// Local internal value for the select control.
const internalValue = ref(props.modelValue);

// Keep the local value in sync with external modelValue.
watch(() => props.modelValue, (newVal) => {
    internalValue.value = newVal;
});

// Reactive error and touched state.
const error = ref<string>('');
const touched = ref(false);

// Validate the input using the provided validators.
function validate() {
    if (props.validators && props.validators.length) {
        for (const validator of props.validators) {
            const result = validator(internalValue.value);
            if (result !== true) {
                error.value = result as string;
                return;
            }
        }
    }
    error.value = '';
}

// Handle change event.
function onChange(e: Event) {
    const target = e.target as HTMLSelectElement;
    internalValue.value = target.value;
    emits('update:modelValue', target.value);
    // Mark as touched on first change.
    if (!touched.value) {
        touched.value = true;
    }
    validate();
}

// Handle blur event (mark as touched and validate).
function onBlur() {
    touched.value = true;
    validate();
}

// Helper functions to extract option value and label.
function getOptionValue(option: Record<string, any> | string): string {
    return typeof option === 'object' && option !== null
        ? String(option[defaultValueKey])
        : String(option);
}

function getOptionLabel(option: Record<string, any> | string): string {
    return typeof option === 'object' && option !== null
        ? String(option[defaultLabelKey])
        : String(option);
}
</script>

<template>
    <div class="mb-3">
        <label :for="id" class="text-sm font-medium text-gray-700">
            {{ label }}
        </label>
        <select :id="id" v-model="internalValue" @change="onChange" @blur="onBlur" class="input-primary">
            <option v-for="option in options" :key="getOptionValue(option)" :value="getOptionValue(option)">
                {{ getOptionLabel(option) }}
            </option>
        </select>
        <p v-if="touched && error" class="text-red-500 text-xs mt-1">{{ error }}</p>
    </div>
</template>

<style scoped>
/* Customize select styling as needed */
</style>
