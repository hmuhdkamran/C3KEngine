<script setup lang="ts">
import { ref, watch, defineProps, defineEmits, computed } from 'vue';
import dayjs from 'dayjs';

// Validator type: returns true if valid; otherwise returns an error message string.
export type Validator = (value: unknown) => true | string;

const props = defineProps<{
    id: string;
    label: string;
    modelValue: string;
    enableTime?: boolean;
    format?: string;
    validators?: Validator[];
}>();

const emits = defineEmits<{
    (e: 'update:modelValue', value: string): void;
}>();

// effectiveFormat is the output format. Defaults to "YYYY-MM-DD HH:mm" if enableTime is true, else "YYYY-MM-DD"
const effectiveFormat = computed(() => props.format || (props.enableTime ? 'YYYY-MM-DD HH:mm' : 'YYYY-MM-DD'));
// effectiveInputFormat is the native input format. For datetime-local, ISO format is "YYYY-MM-DDTHH:mm"
const effectiveInputFormat = computed(() => props.enableTime ? 'YYYY-MM-DDTHH:mm' : 'YYYY-MM-DD');

// Local internal value holds the raw string used by the input control.
const internalValue = ref('');

// Initialize the internal value from modelValue if provided.
if (props.modelValue) {
    const parsed = dayjs(props.modelValue, effectiveFormat.value);
    internalValue.value = parsed.isValid() ? parsed.format(effectiveInputFormat.value) : '';
}

// Keep internalValue in sync with external modelValue.
watch(() => props.modelValue, (newVal) => {
    const parsed = dayjs(newVal, effectiveFormat.value);
    internalValue.value = parsed.isValid() ? parsed.format(effectiveInputFormat.value) : '';
});

// Reactive state for inline validation.
const error = ref<string>('');
const touched = ref(false);

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

function onInput(e: Event) {
    const target = e.target as HTMLInputElement;
    internalValue.value = target.value;
    // Mark the field as touched on first input.
    if (!touched.value) {
        touched.value = true;
    }
    const parsed = dayjs(target.value, effectiveInputFormat.value);
    if (parsed.isValid()) {
        const formattedValue = parsed.format(effectiveFormat.value);
        emits('update:modelValue', formattedValue);
    } else {
        emits('update:modelValue', '');
    }
    validate();
}

function onBlur() {
    touched.value = true;
    validate();
}
</script>

<template>
    <div class="mb-3">
        <label :for="id" class="text-sm font-medium text-gray-700">{{ label }}</label>
        <input :id="id" :type="props.enableTime ? 'datetime-local' : 'date'" v-model="internalValue" @input="onInput"
            @blur="onBlur" class="input-primary" />
        <p v-if="touched && error" class="text-red-500 text-xs mt-1">{{ error }}</p>
    </div>
</template>

<style scoped></style>
