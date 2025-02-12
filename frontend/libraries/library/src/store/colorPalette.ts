import { ref } from 'vue';
import { acceptHMRUpdate, defineStore } from 'pinia';

export const useThemePaletteStore = defineStore('themePallet', () => {
    const selectedColor = ref(localStorage.getItem('selectedColor') || '#265689')

    function setSelectedColor(color: string) {
        selectedColor.value = color;
    }

    return {
        selectedColor,
        setSelectedColor
    }
});

if (import.meta.hot)
    import.meta.hot.accept(acceptHMRUpdate(useThemePaletteStore, import.meta.hot))
