import { reactive } from 'vue';

export const useColorPalette = () => {
  const state = reactive({
    colors: ['#2196F3', '#4CAF50', '#FF5722', '#FFC107', '#9C27B0', '#00BCD4'],
    selectedColor: '#4CAF50',
  });

  const selectColor = (color: string) => {
    state.selectedColor = color;
  };

  return {
    state,
    selectColor,
  };
};
