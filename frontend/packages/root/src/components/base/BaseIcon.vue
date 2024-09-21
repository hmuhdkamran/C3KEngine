<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps({
    path: {
        type: String,
        required: true
    },
    w: {
        type: String,
        default: 'w-6'
    },
    h: {
        type: String,
        default: 'h-6'
    },
})

const spanClass = computed(() => `inline-flex justify-center items-center ${props.path} ${props.w} ${props.h}`)
</script>

<template>
    <span :class="spanClass">
        <slot />
    </span>
</template>