<script setup lang="ts">
import { defineProps, defineEmits } from 'vue';

const props = defineProps<{
    id: string;
    modelValue: boolean;
    label: string;
    icon?: string;
}>();

const emits = defineEmits<{
    (e: 'update:modelValue', value: boolean): void;
}>();

function toggle() {
    emits('update:modelValue', !props.modelValue);
}
</script>

<template>
    <div class="checkbox">
        <label class="checkbox-wrapper">
            <input type="checkbox" class="checkbox-input" :id="id" :checked="modelValue" @change="toggle" />
            <span class="checkbox-tile">
                <span class="checkbox-icon">
                    {{ icon }}
                </span>
                <span class="checkbox-label">{{ label }}</span>
            </span>
        </label>
    </div>
</template>

<style scoped>
.checkbox-input {
    clip: rect(0 0 0 0);
    clip-path: inset(100%);
    height: 1px;
    overflow: hidden;
    position: absolute;
    white-space: nowrap;
    width: 1px;
}

.checkbox-input:checked+.checkbox-tile {
    border-color: var(--primary-color);
    box-shadow: 0 5px 10px rgba(0, 0, 0, 0.1);
    color: var(--primary-color);
}

.checkbox-input:checked+.checkbox-tile:before {
    transform: scale(1);
    opacity: 1;
    background-color: var(--primary-color);
    border-color: var(--primary-color);
}

.checkbox-input:checked+.checkbox-tile .checkbox-icon,
.checkbox-input:checked+.checkbox-tile .checkbox-label {
    color: var(--primary-color);
}

.checkbox-input:focus+.checkbox-tile {
    border-color: var(--primary-color);
    box-shadow: 0 5px 10px rgba(0, 0, 0, 0.1), 0 0 0 4px #b5c9fc;
}

.checkbox-input:focus+.checkbox-tile:before {
    transform: scale(1);
    opacity: 1;
}

.checkbox-tile {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    width: 7rem;
    min-height: 7rem;
    border-radius: 0.5rem;
    border: 2px solid #b5bfd9;
    background-color: #fff;
    box-shadow: 0 5px 10px rgba(0, 0, 0, 0.1);
    transition: 0.15s ease;
    cursor: pointer;
    position: relative;
}

.checkbox-tile:before {
    content: "";
    position: absolute;
    display: block;
    width: 1.25rem;
    height: 1.25rem;
    border: 2px solid var(--primary-color);
    background-color: #fff;
    border-radius: 50%;
    top: 0.25rem;
    left: 0.25rem;
    opacity: 0;
    transform: scale(0);
    transition: 0.25s ease;
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='192' height='192' fill='%23FFFFFF' viewBox='0 0 256 256'%3E%3Crect width='256' height='256' fill='none'%3E%3C/rect%3E%3Cpolyline points='216 72.005 104 184 48 128.005' fill='none' stroke='%23FFFFFF' stroke-linecap='round' stroke-linejoin='round' stroke-width='32'%3E%3C/polyline%3E%3C/svg%3E");
    background-size: 12px;
    background-repeat: no-repeat;
    background-position: 50% 50%;
}

.checkbox-tile:hover {
    border-color: var(--primary-color);
}

.checkbox-tile:hover:before {
    transform: scale(1);
    opacity: 1;
}

.checkbox-icon {
    transition: 0.375s ease;
    color: var(--primary-color);
}

.checkbox-icon svg {
    width: 3rem;
    height: 3rem;
}

.checkbox-label {
    color: var(--primary-color);
    transition: 0.375s ease;
    text-align: center;
}
</style>
