<template>
  <transition name="fade" mode="out-in">
    <div v-if="visible" :class="[
      'absolute z-50 p-4 rounded-xl shadow-lg max-w-sm w-full flex transition-transform transform duration-500',
      notificationTypeClasses[type],
      positionClasses[position]
    ]" role="alert">
      <div class="flex items-start w-full space-x-4">
        <span :class="iconClasses[type]" class="flex-shrink-0 text-2xl"></span>
        <div class="flex-1">
          <strong class="block text-lg font-semibold">{{ title }}</strong>
          <p class="text-sm text-gray-700">{{ message }}</p>
        </div>
        <button @click="visible = false" class="text-gray-500 hover:text-gray-800 transition-colors">
          <span class="icon-[fluent--dismiss-12-filled] h-4 w-4"></span>
        </button>
      </div>
    </div>
  </transition>
</template>

<script lang="ts" setup>
import { ref, onMounted } from 'vue';

// Define props
interface Props {
  type: 'success' | 'error' | 'warning' | 'info';
  title: string;
  message: string;
  position: 'top-right' | 'top-left' | 'bottom-right' | 'bottom-left';
  duration?: number;
}

const props = defineProps<Props>();

// Define visibility state and hide after a certain duration
const visible = ref(true);
const duration = props.duration || 3000;

onMounted(() => {
  setTimeout(() => {
    visible.value = false;
  }, duration);
});

// Notification classes based on type
const notificationTypeClasses: Record<string, string> = {
  success: 'bg-green-100 text-green-800 border-l-4 border-green-600',
  error: 'bg-red-100 text-red-800 border-l-4 border-red-600',
  warning: 'bg-yellow-100 text-yellow-800 border-l-4 border-yellow-600',
  info: 'bg-blue-100 text-blue-800 border-l-4 border-blue-600',
};

// Icon classes for different types  
const iconClasses: Record<string, string> = {
  success: 'icon-[material-symbols--check-circle] text-green-600',
  error: 'icon-[tabler--exclamation-circle-filled] text-red-600',
  warning: 'icon-[mage--exclamation-triangle-fill] text-yellow-600',
  info: 'icon-[mdi--info-circle] text-blue-600',
};

// Position classes for different placements
const positionClasses: Record<string, string> = {
  'top-right': 'top-15 right-4',
  'top-left': 'top-15 left-4',
  'bottom-right': 'bottom-4 right-4',
  'bottom-left': 'bottom-4 left-4',
};
</script>

<style scoped>
.fade-enter-active, .fade-leave-active {
  transition: opacity 0.5s ease, transform 0.5s ease;
}

.fade-enter-from, .fade-leave-to {
  opacity: 0;
  transform: translateY(-10px) scale(0.95);
}

.fade-enter-to, .fade-leave-from {
  opacity: 1;
  transform: translateY(0) scale(1);
}

</style>