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
  <div v-if="props.isVisible" class="fixed inset-0 bg-black bg-opacity-30 w-full h-full z-[998]" @click="onClose"></div>
  <transition name="slide">
    <div v-show="props.isVisible" class="bg-white w-82 h-full fixed right-0 top-0 shadow-lg overflow-hidden flex flex-col z-[999]">
      <div class="flex justify-between items-center p-4 bg-gray-100 border-b">
        <h2 class="text-lg font-semibold text-gray-800">
          {{ props.isEdit ? "Edit" : "View" }}
        </h2>
        <button class="text-gray-500 text-2xl close-button" @click="onClose">
          <span class="icon-[fluent--dismiss-12-regular] w-4 h-4"></span>
        </button>
      </div>
      <div class="p-4 overflow-y-auto flex-grow">
        <div v-if="props.isEdit">
          <span class="font-semibold text-gray-700">Title:</span>
          <input v-model="formData.title" placeholder="Enter Title" class="w-full px-3 py-1 mb-4 input-complete" required />
          <span class="font-semibold text-gray-700">Description:</span>
          <textarea v-model="formData.description" placeholder="Enter Description" class="w-full px-3 py-1 mb-4 input-complete" required></textarea>
        </div>
        <div v-else>
          <span class="font-semibold text-gray-700">Title:</span>
          <input v-model="formData.title" placeholder="Enter Title" class="w-full px-3 py-1 mb-4 input-complete" disabled />
          <span class="font-semibold text-gray-700">Description:</span>
          <textarea v-model="formData.description" placeholder="Enter Description" class="w-full px-3 py-1 mb-4 input-complete" disabled></textarea>
        </div>
        <div class="mb-4 relative">
          <label class="block text-gray-700 font-semibold mb-2" for="status">Status:</label>
          <div v-if="props.isEdit" class="relative">
            <select v-model="formData.status" id="status" class="w-full input-complete px-3 py-1" @focus="isDropdownOpen = true" @blur="isDropdownOpen = false">
              <option value="Activate">Activate</option>
              <option value="Installed">Installed</option>
            </select>
            <span v-if="isDropdownOpen" class="icon-[iconamoon--arrow-up-2] absolute right-3 top-3 text-gray-600"></span>
            <span v-else class="icon-[iconamoon--arrow-down-2] absolute right-3 top-3 text-gray-600"></span>
          </div>
          <div v-else>
            <span class="text-gray-600">{{ formData.status === "Activate" ? "Activate" : "Installed" }}</span>
          </div>
        </div>
      </div>
      <div class="flex justify-end p-4 border-t mt-auto">
        <button class="btn-secondary px-3 py-1 mr-2" @click="onClose">
          Close
        </button>
        <button class="btn-primary px-3 py-1" v-if="props.isEdit" @click="onSave">
          Save
        </button>
      </div>
    </div>
  </transition>
</template>

<style scoped>
.slide-enter-active,
.slide-leave-active {
  transition: transform 0.4s ease-in-out;
}

.slide-enter-from {
  transform: translateX(100%);
}

.slide-leave-to {
  transform: translateX(100%);
}
</style>
