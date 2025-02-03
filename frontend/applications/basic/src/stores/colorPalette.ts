import { ref } from 'vue';

export const colors = ref(['#1d6684', '#7653a0', '#8c5c47', '#993f5b', '#265689', '#0F4C5C', '#2C3E50', '#0077B6', '#5D6D7E', '#008B8B']);
const selectedColor = ref('#265689')

export function setColor(selected: string) {
    selectedColor.value = selected;
}

export function selectColor(): string {
    return selectedColor.value;
}