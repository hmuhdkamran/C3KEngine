import { ref } from 'vue';

export const colors = ref(['#1d6684', '#7653a0', '#8c5c47', '#993f5b', '#98a3b0','#265689', '#0ea5e9 ', '#b788f0', '#ea916a', '#d2649d']);
const selectedColor = ref('#265689')

export function setColor(selected: string) {
    selectedColor.value = selected;
}

export function selectColor(): string {
    return selectedColor.value;
}