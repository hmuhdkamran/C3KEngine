<script setup lang="ts">
const { t } = useI18n()
const props = defineProps({
    width: {
        type: String,
        default: '50%',
    },
    title: {
        type: String,
        default: '',
    },
    showFooter: {
        type: Boolean,
        default: true,
    },
    visible: {
        type: Boolean,
        required: true,
    },
    loading: {
        type: Boolean,
        default: false,
    },
})

const emit = defineEmits(['update:visible', 'onSave'])
const show = computed({
    get() {
        return props.visible
    },
    set(v) {
        emit('update:visible', v)
    },
})
</script>

<template>
    <n-modal v-model:show="show" :style="{ width }" preset="dialog" :title="title" size="huge" :bordered="false" draggable
        :positive-text="t('forms.save')" :negative-text="t('forms.cancel')" @positive-click="emit('onSave')"
        @negative-click="show = false">
        <slot />
    </n-modal>
</template>