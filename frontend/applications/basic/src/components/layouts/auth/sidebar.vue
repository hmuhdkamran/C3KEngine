<script setup lang="ts">
import { ref, computed } from 'vue';

const colors = ref([
  '#FF0000', '#FF7F00', '#FFFF00', '#00FF00', '#0000FF', '#4B0082', '#9400D3',
  '#808080', '#FFFFFF', '#000000'
]);

const selectedColor = ref('');
const shades = computed(() => {
  if (!selectedColor.value) return [];
  return generateShades(selectedColor.value);
});

const selectColor = (color: string) => {
  selectedColor.value = color;
};

const generateShades = (color: string): string[] => {
  const shades = [];
  for (let i = 10; i >= 0; i--) {
    const shade = adjustBrightness(color, i / 10);
    shades.push(shade);
  }
  return shades;
};

const adjustBrightness = (hex: string, factor: number): string => {
  const rgb = hexToRgb(hex);
  const newRgb = {
    r: Math.min(255, rgb.r * factor),
    g: Math.min(255, rgb.g * factor),
    b: Math.min(255, rgb.b * factor)
  };
  return rgbToHex(newRgb.r, newRgb.g, newRgb.b);
};

const hexToRgb = (hex: string) => {
  const r = parseInt(hex.slice(1, 3), 16);
  const g = parseInt(hex.slice(3, 5), 16);
  const b = parseInt(hex.slice(5, 7), 16);
  return { r, g, b };
};

const rgbToHex = (r: number, g: number, b: number): string => {
  const toHex = (c: number) => {
    const hex = c.toString(16);
    return hex.length === 1 ? '0' + hex : hex;
  };
  return `#${toHex(r)}${toHex(g)}${toHex(b)}`;
};
</script>

<template>
  <div class="color-hive">
    <div class="color-palette">
      <div v-for="(color, index) in colors" :key="index" :style="{ backgroundColor: color }" class="color-box"
        @click="selectColor(color)"></div>
    </div>
    <div v-if="selectedColor" class="selected-color">
      <h3>Selected Color: {{ selectedColor }}</h3>
      <div class="shade-box">
        <div v-for="(shade, index) in shades" :key="index" :style="{ backgroundColor: shade }" class="shade">
          {{ shade }}
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.color-hive {
  display: flex;
  flex-direction: column;
  align-items: center;
  margin: 20px;
}

.color-palette {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
}

.color-box {
  width: 50px;
  height: 50px;
  border: 1px solid #ccc;
  cursor: pointer;
}

.selected-color {
  margin-top: 20px;
}

.shade-box {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
}

.shade {
  width: 50px;
  height: 50px;
  border: 1px solid #ccc;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  font-weight: bold;
}
</style>