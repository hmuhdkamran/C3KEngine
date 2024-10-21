<template>
    <button 
      :class="buttonClass"
      :disabled="disabled"
      @click="onClick"
    >
      <slot />
    </button>
  </template>
  
  <script lang="ts" setup>
  import { computed, defineProps } from 'vue';
  
  // Define the button props
  const props = defineProps({
    type: {
      type: String as () => 'primary' | 'secondary' | 'danger' | 'info' | 'gradient' | 'hyper',
      default: 'primary',
    },
    variant: {
      type: String as () => 'solid' | 'outline' | 'rounded',
      default: 'solid',
    },
    disabled: {
      type: Boolean,
      default: false,
    },
    size: {
      type: String as () => 'small' | 'medium' | 'large',
      default: 'medium',
    },
  });
  
  // Emit the click event
  const emit = defineEmits(['click']);
  const onClick = (event: Event) => {
    if (!props.disabled) {
      emit('click', event);
    }
  };
  
  // Computed property for button class based on props
  const buttonClass = computed(() => {
    let baseClass = 'common-btn'; // Common button styles
  
    // Handle button type classes
    const typeClassMap: Record<string, string> = {
      primary: 'btn-primary',
      secondary: 'btn-secondary',
      danger: 'btn-danger',
      info: 'btn-info',
      gradient: 'btn-gradient',
      hyper: 'btn-hyper',
    };
  
    // Handle variant classes
    const variantClassMap: Record<string, string> = {
      solid: 'btn-solid',
      outline: 'btn-outline',
      rounded: 'btn-rounded',
    };
  
    // Append the appropriate classes based on type and variant
    baseClass += ` ${typeClassMap[props.type]} ${variantClassMap[props.variant]}`;
  
    // Handle size classes
    const sizeClassMap: Record<string, string> = {
      small: 'py-1 px-2 text-sm',
      medium: 'py-2 px-4 text-base',
      large: 'py-3 px-6 text-lg',
    };
    baseClass += ` ${sizeClassMap[props.size]}`;
  
    // Handle disabled state
    if (props.disabled) {
      baseClass += ' btn-disabled';
    }
  
    return baseClass;
  });
  </script>
  
  <style scoped>
  /* Button transitions */
  .common-btn {
    @apply cursor-pointer font-medium transition-all duration-300 ease-in-out;
  }
  </style>
  