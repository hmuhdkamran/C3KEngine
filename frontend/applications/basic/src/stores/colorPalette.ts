import { ref } from 'vue';

export const colors = ref(['#1d6684', '#665999', '#8c5c47', '#80264d', '#687380','#265689', "#008a99", '#007366', '#114455', '#332244', ]);
const selectedColor = ref('#265689')

export function setColor(selected: string) {
    selectedColor.value = selected;
}

export function selectColor(): string {
    return selectedColor.value;
}