<script setup lang="ts">
import { ref, defineEmits, defineProps } from 'vue';
import type { IUser } from '@/models';
import { newGuid } from 'c3k-library';

interface Props {
  isVisible: boolean;
}

const props = defineProps<Props>();
const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'save', user: IUser): void;
}>();

const generatedUserId = newGuid();
console.log('Generated UserId:', generatedUserId);

const user = ref<IUser>({
  UserId: generatedUserId,
  Username: '',
  DisplayName: '',
  Language: '',
  Password: '',
  Salt: '....',
  StatusId: '',
});

const languageOptions = ['en-Us'];
const statusOptions = [
  { id: '1', name: 'Active' },
  { id: '2', name: 'Inactive' },
];

const saveUser = () => {
  emit('save', user.value);
};

const cancel = () => {
  emit('close');
};
</script>

<template>
  <div v-if="props.isVisible" class="fixed inset-0 flex items-center justify-center bg-black bg-opacity-50 z-50">
    <div class="bg-white rounded-md shadow-md w-1/2 mx-4 md:mx-0">
      <header class="bg-gray-100 text-gray-800 text-center p-4 rounded-t-lg">
        <h3 class="text-lg font-semibold">Add New User</h3>
      </header>
      <div class="p-6">
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div class="space-y-2">
            <label class="block text-sm font-medium text-gray-700">Username</label>
            <input v-model="user.Username" type="text" class="w-full input-complete" />
          </div>
          <div class="space-y-2">
            <label class="block text-sm font-medium text-gray-700">Display Name</label>
            <input v-model="user.DisplayName" type="text" class="w-full input-complete" />
          </div>
          <div class="space-y-2">
            <label class="block text-sm font-medium text-gray-700">Language</label>
            <select v-model="user.Language" class="w-full input-complete">
              <option value="" disabled>Select language</option>
              <option v-for="language in languageOptions" :key="language" :value="language">
                {{ language }}
              </option>
            </select>
          </div>
          <div class="space-y-2">
            <label class="block text-sm font-medium text-gray-700">Password</label>
            <input v-model="user.Password" type="password" class="input-complete" />
          </div>
          <div class="space-y-2 md:col-span-2">
            <label class="block text-sm font-medium text-gray-700">Status</label>
            <select v-model="user.StatusId" class="w-full input-complete">
              <option value="" disabled>Select status</option>
              <option v-for="status in statusOptions" :key="status.id" :value="status.id">
                {{ status.name }}
              </option>
            </select>
          </div>
        </div>
      </div>
      <footer class="bg-gray-100 p-4 flex justify-end space-x-2 rounded-b-lg">
        <button
          class="px-3 py-1 text-sm btn-secondary"
          @click="cancel">
          Cancel
        </button>
        <button
          class="px-3 py-1 text-sm btn-primary"
          @click="saveUser">
          Save
        </button>
      </footer>
    </div>
  </div>
</template>

<style scoped>

</style>
