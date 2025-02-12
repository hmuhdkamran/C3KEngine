import { ref } from 'vue';
import { acceptHMRUpdate, defineStore } from 'pinia';

export const useThemePalleteStore = defineStore('themePallet', () => {
    const colors = ref(['#1d6684', '#7653a0', '#8c5c47', '#993f5b', '#265689', '#0F4C5C', '#2C3E50', '#0077B6', '#5D6D7E', '#008B8B']);
    const selectedColor = ref(localStorage.getItem('selectedColor') || '#265689')

    function setSelectedColor(color: string) {
        selectedColor.value = color;
    }

    return {
        colors,
        selectedColor,
        setSelectedColor
    }
});

if (import.meta.hot)
    import.meta.hot.accept(acceptHMRUpdate(useThemePalleteStore, import.meta.hot))
