<script setup lang="ts">
import { ref, computed } from 'vue';
import ModuleFormModal from '@/components/forms/moduleform.vue';
import { useRouter } from 'vue-router';

const props = defineProps({
  cardTitle: String,
  cardsData: {
    type: Array as () => Record<string, any>[],
    default: () => [],
  },
  columns: {
    type: Array as () => { key: string, label: string }[],
    default: () => []
  },
});

const router = useRouter()
const selectedCard = ref<Record<string, any> | null>(null);
const searchQuery = ref('');
const currentPage = ref(1);
const itemsPerPage = ref(8);
const sortColumn = ref<string>('');
const sortOrder = ref<'asc' | 'desc'>('asc');
const isEditing = ref(false);

const filteredCards = computed(() => {
  const query = searchQuery.value.toLowerCase();
  return props.cardsData.filter(card =>
    Object.values(card).some(value =>
      value.toString().toLowerCase().includes(query)
    )
  ).sort((a, b) => {
    const compareA = a[sortColumn.value]?.toString().toLowerCase() || '';
    const compareB = b[sortColumn.value]?.toString().toLowerCase() || '';
    if (compareA < compareB) return sortOrder.value === 'asc' ? -1 : 1;
    if (compareA > compareB) return sortOrder.value === 'asc' ? 1 : -1;
    return 0;
  });
});

const paginatedCards = computed(() => {
  const start = (currentPage.value - 1) * itemsPerPage.value;
  const end = start + itemsPerPage.value;
  return filteredCards.value.slice(start, end);
});

const totalPages = computed(() => Math.ceil(filteredCards.value.length / itemsPerPage.value));

const openModal = (card: Record<string, any>) => {
  selectedCard.value = card;
  isEditing.value = true;
};

const closeModal = () => {
  selectedCard.value = null;
  isEditing.value = false;
};

const changeSort = (column: string) => {
  if (sortColumn.value === column) {
    sortOrder.value = sortOrder.value === 'asc' ? 'desc' : 'asc';
  } else {
    sortColumn.value = column;
    sortOrder.value = 'asc';
  }
};

const goToPage = (page: number) => {
  if (page >= 1 && page <= totalPages.value) {
    currentPage.value = page;
  }
};

const handleAction = (action: 'add' | 'edit' | 'delete', card: Record<string, any>) => {
  console.log(`${action} card`, card);
};

const reloadPage = () => {
  window.location.reload();
};

function goToMain() {
  router.replace('/app/main');
}
</script>


<template>
  <div class="h-screen py-2 px-16 bg-gray-50">
    <nav class="flex py-4 text-gray-600 text-sm">
      <button @click="goToMain" class="hover:underline">Dashboard</button>
      <span class="mx-2">/</span>
      <span>{{ cardTitle }}</span>
    </nav>

    <div class="flex justify-between items-center mb-4">
      <h1 class="text-2xl font-bold text-gray-800">{{ cardTitle }} - Modules</h1>
      <div class="flex items-center space-x-4">

      </div>
    </div>
    <div v-if="!isEditing">
      <div class="flex justify-between text-sm items-center mb-4">
        <input v-model="searchQuery" type="text" placeholder="Search modules..."
          class="input-complete w-full md:w-1/3 px-4 py-2 border rounded-md shadow-sm bg-white" />
        <div class="flex items-center">
          <button @click="reloadPage" class="text-gray-600 hover:bg-gray-200 rounded-full p-2 transition-all">
            <span class="icon-[mage--reload] w-5 h-5"></span>
          </button>
          <button @click="handleAction('add', {} as Record<string, any>)"
            class="text-gray-600 hover:bg-gray-200 rounded-full p-2 transition-all">
            <span class="icon-[mdi--plus] w-5 h-5"></span>
          </button>
        </div>
      </div>
      <div class="overflow-x-auto shadow-md bg-white rounded-lg">
        <table class="min-w-full bg-white border border-gray-200">
          <thead>
            <tr class="bg-gray-200 border-b border-gray-300">
              <th v-for="column in props.columns" :key="column.key"
                class="p-2 text-left text-gray-600 cursor-pointer hover:bg-gray-300 transition-colors text-md font-medium"
                @click="changeSort(column.key)">
                {{ column.label }}
                <span v-if="sortColumn === column.key" class="ml-1 text-md">{{ sortOrder === 'asc' ? '↑' : '↓' }}</span>
              </th>
              <th class="p-2 text-center text-gray-600 text-md font-medium 
              cursor-pointer hover:bg-gray-300 transition-colors">Actions</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="card in paginatedCards" :key="card[props.columns[0].key]"
              class="border-b border-gray-300 hover:bg-gray-50 transition-all text-md">
              <td class="p-2">{{ card.title }}</td>
              <td class="p-2">{{ card.description }}</td>
              <td class="p-2 text-xs">
                <span :class="card.status === 'Installed' ? 'bg-green-100 text-green-700' : 'bg-yellow-100 text-yellow-700'"
                  class="px-2 py-0.5 rounded-full">
                  {{ card.status }}
                </span>
              </td>
              <td class="p-2 text-center">
                <button @click="openModal(card)"
                  class="transition-all text-gray-600 focus:outline-none hover:bg-gray-200 rounded-full">
                  <span class="icon-[mdi--edit-outline] w-5 h-5"></span>
                </button>
                <button @click="handleAction('delete', card)"
                  class="transition-all text-gray-600 focus:outline-none ml-1 hover:bg-gray-200 rounded-full">
                  <span class="icon-[material-symbols--delete-outline] w-5 h-5"></span>
                </button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <div class="mt-4 flex flex-col md:flex-row justify-between items-center">
        <div class="text-gray-600 text-sm mb-4 md:mb-0">
          Page {{ currentPage }} of {{ totalPages }}
        </div>
        <div class="flex space-x-2">
          <button @click="goToPage(currentPage - 1)" :disabled="currentPage === 1"
            class="px-4 py-2 bg-gray-200 text-gray-600 rounded-md hover:bg-gray-300 disabled:opacity-50 text-sm disabled:cursor-not-allowed transition-all">
            Previous
          </button>
          <button @click="goToPage(currentPage + 1)" :disabled="currentPage === totalPages"
            class="px-4 py-2 bg-gray-200 text-gray-600 rounded-md hover:bg-gray-300 disabled:opacity-50 text-sm disabled:cursor-not-allowed transition-all">
            Next
          </button>
        </div>
      </div>
    </div>

    <ModuleFormModal v-if="isEditing" :isVisible="isEditing" :moduleTitle="selectedCard?.title" @close="closeModal" />
  </div>
</template>


<style scoped>
table {
  border-spacing: 0 10px;
}

table thead th {
  border-bottom: 2px solid #e0d8f3;
}

table tbody tr {
  background-color: #f9fafb;
  border: 1px solid #e5e7eb;
  border-radius: 8px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

table tbody tr:hover {
  background-color: #f3f4f6;
  transform: translateY(-2px);
}
</style>