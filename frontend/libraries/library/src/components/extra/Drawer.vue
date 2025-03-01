<script setup lang="ts">
import { defineProps, defineEmits } from 'vue';
import circuit from '@/assets/images/headerbg.jpeg';

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
  size: 'w-1/3',
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
  left: '-translate-x-full',
  right: 'translate-x-full',
  top: '-translate-y-full',
  bottom: 'translate-y-full'
};

const sizeClasses = {
  left: 'h-full',
  right: 'h-full',
  top: 'w-full',
  bottom: 'w-full'
};

</script>

<template>
  <Teleport to="body">
    <!-- Drawer -->
    <transition :name="`${props.position}-slide`">
      <div v-if="props.isOpen"
        :class="`fixed ${positionClasses[props.position]} ${transformClasses[props.position]} ${props.size} ${sizeClasses[props.position]} bg-white text-gray-800 shadow-lg z-50`"
        class="rounded-sm flex flex-col z-50">
        <!-- Header -->
        <div class="flex items-center justify-between bg-blue-600 text-violet-600"
             :style="{ 
               backgroundImage: `url(${circuit})`,
               backgroundSize: '70% auto',
               backgroundPosition: 'right center',
               backgroundRepeat: 'no-repeat',
               boxShadow: '0 2px 4px rgba(0, 0, 0, 0.1)'
             }">
          <template v-if="$slots.header">
            <slot name="header" />
          </template>
          <template v-else>
            <h2 class="font-semibold text-lg">{{ props.title }}</h2>
          </template>
        </div>
        <!-- Content -->
        <div class="flex-grow overflow-y-auto p-4">
          <slot />
        </div>
      </div>
    </transition>
    <!-- Background Overlay -->
    <transition name="fade">
      <div v-if="props.isOpen && props.closeOnOutside" class="fixed inset-0 bg-black bg-opacity-50 z-40"
        @click="emit('toggleDrawer')"></div>
    </transition>
  </Teleport>
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