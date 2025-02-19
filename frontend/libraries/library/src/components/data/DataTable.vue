<script setup lang="ts">
import { useTableStore } from '@/store'
import { ref, computed } from 'vue'

const props = defineProps({
  data: {
    type: Array as () => Record<string, any>[],
    default: () => [],
  },
  columns: {
    type: Array as () => {
      key: string
      label: string
      sort?: boolean
      width?: string
      class?: string
      check?: boolean
    }[],
    default: () => [],
  },
  uniqueKey: {
    type: String,
    default: 'id',
  },
})

const tableStore = useTableStore()
const sortColumn = ref<string>('')
const sortOrder = ref<'asc' | 'desc'>('asc')
const selectedRecords = ref<string[]>([])
const selectAll = ref(false)

// Computed Properties
const itemsPerPage = computed(() => tableStore.itemsPerPage)
const filteredRecords = computed(() => {
  const query = tableStore.searchQuery.toLowerCase()
  let records = props.data.filter((record) =>
    Object.values(record).some((value) => String(value).toLowerCase().includes(query)),
  )

  if (sortColumn.value) {
    const key = sortColumn.value
    const order = sortOrder.value
    records = records.sort((a, b) => {
      const valA = String(a[key]).toLowerCase()
      const valB = String(b[key]).toLowerCase()
      if (valA < valB) return order === 'asc' ? -1 : 1
      if (valA > valB) return order === 'asc' ? 1 : -1
      return 0
    })
  }

  tableStore.updateTotalRecords(records.length)
  return records
})

const paginatedRecords = computed(() => {
  const start = (tableStore.currentPage - 1) * itemsPerPage.value
  return filteredRecords.value.slice(start, start + itemsPerPage.value)
})

// Methods
const changeSort = (column: string) => {
  if (sortColumn.value === column) {
    sortOrder.value = sortOrder.value === 'asc' ? 'desc' : 'asc'
  } else {
    sortColumn.value = column
    sortOrder.value = 'asc'
  }
}

const toggleSelectAll = () => {
  selectAll.value = !selectAll.value
  selectedRecords.value = selectAll.value
    ? paginatedRecords.value.map((record) => record[props.uniqueKey!])
    : []
}

// const isSelected = (record: Record<string, any>) => selectedRecords.value.includes(record[props.uniqueKey!]);
</script>

<template>
  <div>
    <div class="overflow-x-auto rounded-sm shadow-md">
      <!-- Check for template-list slot -->
      <slot name="template" :records="paginatedRecords" :unique-key="props.uniqueKey" v-if="$slots.template"></slot>

      <!-- Default Table -->
      <table v-else class="table-auto w-full border-collapse">
        <thead>
          <tr class="border-b bg-gray-50 border-gray-300">
            <template v-for="column in props.columns" :key="column.key">
              <th v-if="column.check" class="cursor-pointer">
                <input class="cursor-pointer" type="checkbox" v-model="selectAll" @change="toggleSelectAll" />
              </th>
              <th v-else class="cursor-pointer" :class="column.class" :style="{ width: column.width }"
                @click="column.sort && changeSort(column.key)">
                {{ column.label }}
                <span v-if="sortColumn === column.key" class="ml-1 text-md">
                  {{ sortOrder === 'asc' ? '↑' : '↓' }}
                </span>
              </th>
            </template>
          </tr>
        </thead>

        <tbody class="text-sm">
          <tr v-for="(record, index) in paginatedRecords" :key="record[props.uniqueKey!]" :class="[
            index % 2 === 0 ? 'bg-gray-50' : 'bg-white',
            'hover:shadow-lg hover:bg-gray-100 transition-shadow duration-200',
            'border-b border-gray-300 border-dashed',
          ]">
            <template v-for="column in props.columns" :key="column.key">
              <td v-if="column.check" class="cursor-pointer text-center p-1">
                <input class="cursor-pointer" type="checkbox" :value="record[props.uniqueKey!]"
                  v-model="selectedRecords" />
              </td>
              <td v-else :class="[column.class, 'p-1']" :style="{ width: column.width }">
                <slot :name="column.key" :field="column.key" :row="record" v-if="$slots[column.key]"></slot>
                <span v-else>{{ record[column.key] }}</span>
              </td>
            </template>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- Pagination Info -->
    <div class="mt-4 text-sm flex justify-between items-center">
      Showing
      {{ (tableStore.currentPage - 1) * itemsPerPage + 1 }} to
      {{ Math.min(tableStore.currentPage * itemsPerPage, filteredRecords.length) }}
      of {{ filteredRecords.length }} records
    </div>
  </div>
</template>

<style scoped>
.overflow-x-auto {
  overflow-x: auto;
  -webkit-overflow-scrolling: touch;
}

thead tr th {
  height: 40px;
}
</style>
