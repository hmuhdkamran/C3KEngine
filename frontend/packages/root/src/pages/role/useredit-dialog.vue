<script setup lang="ts">
import { ref, defineProps, defineEmits, watch } from "vue";

interface Props {
  isVisible: boolean;
  entryData: any;
  isEdit: boolean;
}

interface Emit {
  (e: "close"): void;
  (e: "save", data: any): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emit>();

const isDropdownOpen = ref(false);
const formData = ref({ ...props.entryData });

watch(
  () => props.entryData,
  (newData) => {
    formData.value = { ...newData };
  }
);

const onClose = () => {
  emit("close");
};

const onSave = () => {
  emit("save", formData.value);
};
</script>

<template>
  <div v-if="props.isVisible" class="fixed inset-0 bg-black bg-opacity-40 w-full h-full z-[998]"></div>
  <transition name="fade">
    <div v-show="props.isVisible" class="fixed inset-0 flex items-center justify-center z-[999]">
      <div class="bg-white w-full max-w-xl rounded-md shadow-2xl overflow-hidden">
        <div class="flex justify-between items-center p-5 border-b bg-gray-100">
          <h2 class="text-xl font-semibold tracking-tight">
            {{ props.isEdit ? "Edit" : "View" }}
          </h2>
          <button class="text-gray-500 text-lg hover:text-gray-800 transition-all" @click="onClose">
            <span class="icon-[fluent--dismiss-12-regular] w-4 h-4"></span>
          </button>
        </div>
        <div class="p-6 space-y-6 overflow-y-auto max-h-[60vh]">
          <div v-if="props.isEdit">
            <label class="block font-medium text-gray-700 mb-2" for="title">Title</label>
            <input v-model="formData.title" id="title" placeholder="Enter Title" class="w-full input-complete" required />
            
            <label class="block font-medium text-gray-700 mt-6 mb-2" for="description">Description</label>
            <textarea v-model="formData.description" id="description" placeholder="Enter Description" class="w-full input-complete" rows="4" required></textarea>
          </div>

          <div v-else>
            <label class="block font-medium text-gray-700 mb-2">Title</label>
            <input v-model="formData.title" placeholder="Title" class="w-full input-complete bg-gray-50" disabled />
            
            <label class="block font-medium text-gray-700 mt-6 mb-2">Description</label>
            <textarea v-model="formData.description" placeholder="Description" class="w-full input-complete bg-gray-50" rows="4" disabled></textarea>
          </div>
          <div class="mt-6">
            <label class="block font-medium text-gray-700 mb-2" for="status">Status</label>
            <div v-if="props.isEdit" class="relative">
              <select v-model="formData.status" id="status" class="w-full input-complete" @focus="isDropdownOpen = true" @blur="isDropdownOpen = false">
                <option value="Activate">Activate</option>
                <option value="Installed">Installed</option>
              </select>
              <span v-if="isDropdownOpen" class="icon-[iconamoon--arrow-up-2] absolute right-4 top-1/2 transform -translate-y-1/2 text-gray-600"></span>
              <span v-else class="icon-[iconamoon--arrow-down-2] absolute right-4 top-1/2 transform -translate-y-1/2 text-gray-600"></span>
            </div>
            <div v-else>
              <span class="text-gray-600">{{ formData.status === "Activate" ? "Activate" : "Installed" }}</span>
            </div>
          </div>
        </div>
        <div class="flex justify-end items-center p-6 border-t mt-4 space-x-3">
          <button class="px-3 py-1 btn-secondary transition-all" @click="onClose">
            Close
          </button>
          <button v-if="props.isEdit" class="px-3 py-1 btn-primary transition-all" @click="onSave">
            Save
          </button>
        </div>
      </div>
    </div>
  </transition>
</template>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}
.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
