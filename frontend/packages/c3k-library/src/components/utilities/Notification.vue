<template>
    <transition name="fade" mode="out-in">
      <div 
        v-if="visible"
        :class="[
          'absolute z-50 p-4 rounded-lg shadow-lg max-w-xs transition-all duration-300', 
          notificationTypeClasses[type], 
          positionClasses[position],
        ]"
        role="alert"
      >
        <div class="flex items-start border-l-4" :class="borderClasses[type]">
          <span :class="iconClasses[type]" class="mr-3 mt-1"></span>
          <div class="flex flex-col">
            <strong class="font-bold">{{ title }}</strong>
            <p class="text-sm">{{ message }}</p>
          </div>
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
    success: 'bg-green-100 text-green-800',
    error: 'bg-red-100 text-red-800',
    warning: 'bg-yellow-100 text-yellow-800',
    info: 'bg-blue-100 text-blue-800',
  };
  
  // Thick border classes based on type
  const borderClasses: Record<string, string> = {
    success: 'border-green-500',
    error: 'border-red-500',
    warning: 'border-yellow-500',
    info: 'border-blue-500',
  };
  
  // Icon classes for different types
  const iconClasses: Record<string, string> = {
    success: 'icon-[mdi--success-circle-outline]',
    error: 'icon-[nonicons-error-16]',
    warning: 'icon-[mingcute--warning-line]',
    info: 'icon-[ph--info]',
  };
  
  // Position classes for different placements
  const positionClasses: Record<string, string> = {
    'top-right': 'top-4 right-4',
    'top-left': 'top-4 left-4',
    'bottom-right': 'bottom-4 right-4',
    'bottom-left': 'bottom-4 left-4',
  };
  </script>
  
  <style scoped>
  .fade-enter-active, .fade-leave-active {
    transition: opacity 0.3s;
  }
  .fade-enter, .fade-leave-to /* .fade-leave-active in <2.1.8 */ {
    opacity: 0;
  }
  </style>
  