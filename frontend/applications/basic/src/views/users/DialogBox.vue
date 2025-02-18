<script setup lang="ts">
import { defineProps, defineEmits, computed } from 'vue';
import { circuit } from '@/assets/images/images';

const props = defineProps({
    show: {
        type: Boolean,
        required: true,
    },
    width: {
        type: String,
        default: '50%',
    },
    showClose: {
        type: Boolean,
        default: true,
    },
});

const emit = defineEmits(['close']);

const closeDialog = () => {
    emit('close');
};
</script>

<template>
    <Teleport to="body">
        <transition name="dialog">
            <div v-if="show" class="dialog-overlay">
                <div class="dialog" :style="{ width: width, maxWidth: '95%', }">
                    <div class="dialog-header" :style="{ 
                        backgroundImage: `url(${circuit})`,
                        backgroundSize: '50% auto, 100% 100%',
                        backgroundPosition: 'right center, center center',
                        backgroundRepeat: 'no-repeat, no-repeat',
                        boxShadow: '0 2px 4px rgba(0, 0, 0, 0.1)'
                    }" v-if="$slots.header">
                        <slot name="header" />
                        <button v-if="showClose" class="close-btn" @click="closeDialog">×</button>
                    </div>
                    <div class="dialog-body">
                        <slot />
                    </div>
                    <div class="dialog-footer" v-if="$slots.footer">
                        <slot name="footer" />
                    </div>
                </div>
            </div>
        </transition>
    </Teleport>
</template>

<style scoped>
/* Dialog Overlay Styles */
.dialog-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: rgba(0, 0, 0, 0.5);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 1000;
}

/* Dialog Container Styles */
.dialog {
    background-color: #fff;
    border-radius: 4px;
    overflow: hidden;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    max-height: 90vh;
    display: flex;
    flex-direction: column;
    position: relative;
    transition: all 0.3s ease;
}

/* Header Styles */
.dialog-header {
    padding: 12px;
    border-bottom: 1px solid #e0e0e0;
    align-items: center;
    text-align: left;
}

.close-btn {
    cursor: pointer;
    border: none;
    background: transparent;
    font-size: 18px;
}

/* Body Styles */
.dialog-body {
    padding: 20px;
    flex: 1;
    overflow-y: auto;
}

/* Footer Styles */
.dialog-footer {
    padding: 10px;
    border-top: 1px solid #e0e0e0;
    display: flex;
    justify-content: flex-end;
    gap: 8px;
    background: #f8f8f8;
}

/* Transition Styles */
.dialog-enter-active,
.dialog-leave-active {
    transition: opacity 0.3s ease;
}

.dialog-enter-from,
.dialog-leave-to {
    opacity: 0;
}

.dialog-enter-active .dialog,
.dialog-leave-active .dialog {
    transition: all 0.3s ease;
}

.dialog-enter-from .dialog {
    transform: scale(0.9);
    opacity: 0;
}

.dialog-leave-to .dialog {
    transform: scale(0.9);
    opacity: 0;
}

@media (max-width: 900px) {
    .dialog-header {
        background-size: 100% auto, 100% 100% !important;
    }
}
</style>