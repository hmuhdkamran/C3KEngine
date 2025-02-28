<script setup lang="ts">
import { defineProps, defineEmits } from 'vue';

const props = defineProps<{
    id: string;
    modelValue: string[];
    label: string;
    value: string;
    icon: string | null;
}>();

const emits = defineEmits<{
    (e: 'update:modelValue', value: string[]): void;
}>();

function toggle() {
    const newValue = props.modelValue.includes(props.value)
        ? props.modelValue.filter(v => v !== props.value)
        : [...props.modelValue, props.value];
    emits('update:modelValue', newValue);
}
</script>

<template>
    <div class="checkbox-grid">
        <label class="checkbox-wrapper">
            <input type="checkbox" class="checkbox-input" :id="id" :checked="modelValue.includes(value)"
                @change="toggle" />
            <span class="checkbox" :class="{ 'is-checked': modelValue.includes(value) }">
                <span v-if="icon" class="checkbox-icon-left" :class="{ 'is-checked': modelValue.includes(value) }">
                    <i :class="icon" aria-hidden="true"></i>
                </span>
                <span class="checkbox-label" :class="{ 'is-checked': modelValue.includes(value) }">{{ label }}</span>
                <span class="checkbox-icon-right">
                    <i v-if="modelValue.includes(value)" class="fa-solid fa-circle-check" aria-hidden="true"></i>
                    <i v-else class="fa-regular fa-circle" aria-hidden="true"></i>
                </span>
            </span>
        </label>
    </div>
</template>