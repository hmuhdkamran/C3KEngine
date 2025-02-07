import { computed, ref } from 'vue';

const isFormOpen = ref(false);

export function setFormOpen(value: boolean) {
    isFormOpen.value = value;
}

export const formStatus = computed(() => isFormOpen.value)