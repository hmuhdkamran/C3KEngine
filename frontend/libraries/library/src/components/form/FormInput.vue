<script setup lang="ts">
import { ref, watch, defineProps, defineEmits } from 'vue';

// Define a type for a validator function.
// A valid result returns true; otherwise it returns an error message string.
export type Validator = (value: unknown) => true | string;

const props = defineProps<{
    id: string;
    label: string;
    modelValue: string;
    type?: string;
    placeholder?: string;
    validators?: Validator[];
}>();

const emits = defineEmits<{
    (e: 'update:modelValue', value: string): void;
}>();

// Use a local ref to track the input value.
const internalValue = ref(props.modelValue);

// Watch the external modelValue to keep local state in sync.
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

// Handle input event.
function onInput(e: Event) {
    const target = e.target as HTMLInputElement;
    internalValue.value = target.value;
    emits('update:modelValue', target.value);
    // Mark field as touched on first input so errors display immediately.
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
</script>

<template>
    <div class="mb-3">
        <label :for="id" class="text-sm font-medium text-gray-700">
            {{ label }}
        </label>
        <input :id="id" :type="type || 'text'" :placeholder="placeholder" v-model="internalValue" @input="onInput"
            @blur="onBlur" class="input-primary" />
        <p v-if="touched && error" class="text-red-500 text-xs mt-1">{{ error }}</p>
    </div>
</template>

<style scoped>
/* Customize input styling as needed */
</style>
