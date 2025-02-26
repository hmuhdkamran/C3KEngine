<script setup lang="ts">
import { ref, watch, defineProps, defineEmits, computed } from 'vue';
import dayjs from 'dayjs';

// Define a type for a validator function.
// A valid result returns `true`; otherwise it returns an error message string.
export type Validator = (value: unknown) => true | string;

const props = defineProps<{
    id: string;
    label: string;
    // The formatted value from parent.
    modelValue: string;
    // If true, allow time selection; if false, date only.
    enableTime?: boolean;
    // Format to output (e.g., "YYYY-MM-DD HH:mm"); defaults based on enableTime.
    format?: string;
    validators?: Validator[];
}>();

const emits = defineEmits<{
    (e: 'update:modelValue', value: string): void;
}>();

// Determine effective formats:
// - effectiveFormat: the output format provided by parent (or default)
// - effectiveInputFormat: the ISO format expected by the native input element.
const effectiveFormat = computed(() => {
    return props.format || (props.enableTime ? "YYYY-MM-DD HH:mm:ss" : "YYYY-MM-DD");
});
const effectiveInputFormat = computed(() => {
    return props.enableTime ? "YYYY-MM-DDTHH:mm" : "YYYY-MM-DD";
});

// Internal value bound to the input element.
const internalValue = ref("");
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

// When the component is created, parse the incoming modelValue (if provided)
// into the format needed by the native input.
if (props.modelValue) {
    const parsed = dayjs(props.modelValue, effectiveFormat.value);
    if (parsed.isValid()) {
        internalValue.value = parsed.format(effectiveInputFormat.value);
    }
}

// Watch external modelValue changes and update internalValue accordingly.
watch(() => props.modelValue, (newVal) => {
    const parsed = dayjs(newVal, effectiveFormat.value);
    if (parsed.isValid()) {
        internalValue.value = parsed.format(effectiveInputFormat.value);
    } else {
        internalValue.value = "";
    }
});

// On user input, parse the raw value and emit it in the desired format.
function onInput(e: Event) {
    const target = e.target as HTMLInputElement;
    internalValue.value = target.value;
    const parsed = dayjs(target.value, effectiveInputFormat.value);
    if (parsed.isValid()) {
        emits('update:modelValue', parsed.format(effectiveFormat.value));
    } else {
        emits('update:modelValue', "");
    }
    // Optionally validate on input
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
        <input :id="id" :type="props.enableTime ? 'datetime-local' : 'date'" v-model="internalValue" @input="onInput" @blur="onBlur"
            class="input-primary" />
        <p v-if="touched && error" class="text-red-500 text-xs mt-1">{{ error }}</p>
    </div>
</template>

<style scoped>
/* Customize select styling as needed */
</style>
