<script setup lang="ts">
// import { ref } from 'vue';
import { defineProps, defineEmits, useSlots } from 'vue';

interface Props {
  isOpen: boolean;
  title: string;
  position: 'left' | 'right' | 'top' | 'bottom';
  size: string;
  showCloseButton?: boolean;
  closeOnOutside?: boolean;
};

interface Emit {
  (e: 'toggleDrawer'): void;
}

const props = withDefaults(defineProps<Props>(), {
  isOpen: false,
  title: '',
  position: 'left',
  size: 'size-48',
  showCloseButton: true,
  closeOnOutside: true
});
const emit = defineEmits<Emit>();

const positionClasses = {
  left: 'inset-y-0 left-0',
  right: 'inset-y-0 right-0',
  top: 'inset-x-0 top-0',
  bottom: 'inset-x-0 bottom-0'
};
const transformClasses = {
  left: 'translate-x-[-100%]',
  right: 'translate-x-[100%]',
  top: 'translate-y-[-100%]',
  bottom: 'translate-y-[100%]'
};

const slots = useSlots();
</script>

<template>
  <!-- Background Overlay -->
  <transition name="fade">
    <div v-if="props.isOpen && props.closeOnOutside" class="fixed inset-0 bg-black bg-opacity-50 z-40"
      @click="emit('toggleDrawer')"></div>
  </transition>

  <!-- Drawer -->
  <transition :name="`${props.position}-slide`">
    <div v-if="props.isOpen"
      :class="`fixed ${positionClasses[props.position]} ${transformClasses[props.position]} ${props.size} bg-white text-gray-800 shadow-lg z-50`"
      class="rounded-lg flex flex-col h-full w-full">
      <!-- Header -->
      <div class="flex items-center justify-between px-5 py-4 bg-blue-600 text-white">
        <template v-if="slots.header">
          <slot name="header" />
        </template>
        <template v-else>
          <h2 class="font-semibold text-lg">{{ props.title }}</h2>
        </template>
        <button v-if="props.showCloseButton" @click="emit('toggleDrawer')" class="text-white hover:text-gray-200">
          <span class="icon-[fluent--dismiss-20-filled] h-5 w-5"></span>
        </button>
      </div>
      <!-- Content -->
      <div class="flex-grow overflow-y-auto p-4">
        <slot />
      </div>
    </div>
  </transition>
</template>

<style scoped>
/* Slide Transitions */
.left-slide-enter-active,
.left-slide-leave-active,
.right-slide-enter-active,
.right-slide-leave-active,
.top-slide-enter-active,
.top-slide-leave-active,
.bottom-slide-enter-active,
.bottom-slide-leave-active {
  transition: transform 0.4s ease;
}

.left-slide-enter-from {
  transform: translateX(-100%);
}

.left-slide-leave-to {
  transform: translateX(-100%);
}

.right-slide-enter-from {
  transform: translateX(100%);
}

.right-slide-leave-to {
  transform: translateX(100%);
}

.top-slide-enter-from {
  transform: translateY(-100%);
}

.top-slide-leave-to {
  transform: translateY(-100%);
}

.bottom-slide-enter-from {
  transform: translateY(100%);
}

.bottom-slide-leave-to {
  transform: translateY(100%);
}

/* Fade Transition for Background */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
