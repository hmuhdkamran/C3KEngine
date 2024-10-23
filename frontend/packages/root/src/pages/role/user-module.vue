<script setup lang="ts">
import { ref, defineProps } from 'vue';
import ConfirmationDialog from '@/layouts/components/ConfirmationDialog.vue';

const props = defineProps<{
  row: any;
  onOpenModal: (action: string, row: any) => void;
}>();

const isDeleteDialogVisible = ref(false);

const onView = () => {
  props.onOpenModal('view', props.row);
};

const onEdit = () => {
  props.onOpenModal('edit', props.row);
};

const onDelete = () => {
  isDeleteDialogVisible.value = true;
};

const onConfirmDelete = () => {
  props.onOpenModal('delete', props.row);
  isDeleteDialogVisible.value = false;
};

const onCancelDelete = () => {
  isDeleteDialogVisible.value = false;
};
</script>

<template>
  <div class="flex justify-center space-x-2">
    <button class="grid-action-btn hover-btn-warning" @click="onView">
      <span class="icon-[ep--view]"></span>
    </button>
    <button class="grid-action-btn hover-btn-primary" @click="onEdit">
      <span class="icon-[akar-icons--edit]"></span>
    </button>
    <button class="grid-action-btn hover-btn-danger" @click="onDelete">
      <span class="icon-[hugeicons--delete-02]"></span>
    </button>
  </div>
  <ConfirmationDialog
    v-if="isDeleteDialogVisible"
    title="Confirm Delete"
    message="Are you sure you want to delete this entry?"
    :isVisible="isDeleteDialogVisible"
    @confirm="onConfirmDelete"
    @cancel="onCancelDelete"
  />
</template>
