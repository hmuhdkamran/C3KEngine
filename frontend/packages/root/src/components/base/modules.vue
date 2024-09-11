<script setup lang="ts">
import { ref, computed } from 'vue';
import ModuleFormModal from '@/components/forms/moduleform.vue';

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

const selectedCard = ref<Record<string, any> | null>(null);
const searchQuery = ref('');
const currentPage = ref(1);
const itemsPerPage = ref(8);
const sortColumn = ref<string>('');
const sortOrder = ref<'asc' | 'desc'>('asc');

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
};

const closeModal = () => {
  selectedCard.value = null;
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
</script>


<template>
  <div class="h-screen p-6 bg-gray-100">
    <div class="mb-6 flex flex-col md:flex-row md:justify-between items-start md:items-center">
      <h1 class="text-2xl md:text-3xl font-bold text-gray-800">{{ cardTitle }} - Modules</h1>
      <div class="flex flex-col md:flex-row space-y-4 md:space-y-0 md:space-x-4 w-full md:w-auto">
        <button @click="handleAction('add', {} as Record<string, any>)"
          class="text-black py-2 px-4 rounded-full hover:bg-gray-200 transition-colors flex items-center">
          <span class="icon-[mdi--plus] w-5 h-5"></span>
        </button>
        <input v-model="searchQuery" type="text" placeholder="Search..."
          class="border border-gray-300 rounded-md p-2 w-full md:max-w-xs focus:ring-2 focus:ring-violet-500 focus:outline-none" />
      </div>
    </div>

    <div class="overflow-x-auto shadow-lg">
      <table class="min-w-full bg-white border border-gray-200 rounded-lg">
        <thead>
          <tr class="bg-gray-200 border-b border-gray-300">
            <th v-for="column in props.columns" :key="column.key"
              class="p-4 text-left text-gray-600 cursor-pointer"
              @click="changeSort(column.key)">
              {{ column.label }}
              <span v-if="sortColumn === column.key" class="ml-2">{{ sortOrder === 'asc' ? '↑' : '↓' }}</span>
            </th>
            <th class="p-4 text-center text-gray-600">Actions</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="card in paginatedCards" :key="card[props.columns[0].key]"
            class="border-b border-gray-300 hover:bg-gray-100 transition-transform transform hover:scale-103">
            <td class="p-4">{{ card.title }}</td>
            <td class="p-4">{{ card.description }}</td>
            <td class="p-4">
              <slot name="status" :status="card.status">
                <span :class="card.status === 'Installed' ? 'bg-green-100 text-green-700' : 'bg-yellow-100 text-yellow-700'"
                  class="px-2 py-1 rounded-full text-xs font-semibold">
                  {{ card.status }}
                </span>
              </slot>
            </td>
            <td class="p-4 text-center">
              <button @click="openModal(card)"
                class="transition-colors focus:outline-none">
                <span class="icon-[mdi--edit-outline] w-5 h-5"></span>
              </button>
              <button @click="handleAction('delete', card)"
                class="transition-colors focus:outline-none ml-2">
                <span class="icon-[material-symbols--delete-outline] w-5 h-5"></span>
              </button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <div class="mt-4 flex flex-col md:flex-row justify-between items-center">
      <div class="text-gray-600 mb-4 md:mb-0">
        Page {{ currentPage }} of {{ totalPages }}
      </div>
      <div class="flex space-x-2">
        <button @click="goToPage(currentPage - 1)" :disabled="currentPage === 1"
          class="px-4 py-2 bg-gray-200 text-gray-600 rounded-md hover:bg-gray-300 disabled:opacity-50 disabled:cursor-not-allowed">
          Previous
        </button>
        <button @click="goToPage(currentPage + 1)" :disabled="currentPage === totalPages"
          class="px-4 py-2 bg-gray-200 text-gray-600 rounded-md hover:bg-gray-300 disabled:opacity-50 disabled:cursor-not-allowed">
          Next
        </button>
      </div>
    </div>

    <ModuleFormModal v-if="selectedCard" :isVisible="!!selectedCard" :moduleTitle="selectedCard.title"
      @close="closeModal" />
  </div>
</template>


<style scoped>
table tbody tr {
 transition: transform 0.3s ease;
}

table tbody tr:hover {
 transform: scale(1.02);
}
</style>