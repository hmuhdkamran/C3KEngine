<script setup lang="ts">
import { DataTable, useTableStore, ConfirmationDialog, Drawer } from "c3k-library";
import { ref, onMounted } from "vue";
const tableStore = useTableStore();

const hrmsCards = [
  {
    title: 'Employee Management',
    description: 'Manage employee records and workflows efficiently.',
    status: 'Installed',
    buttonText: 'LEARN MORE',
    iconClass: 'icon-[fluent-mdl2--people]',
  },
  {
    title: 'Payroll',
    description: 'Automate payroll processing and tax management.',
    status: 'Activate',
    buttonText: 'LEARN MORE',
    iconClass: 'icon-[fluent-mdl2--money]',
  },
  {
    title: 'Attendance Tracking',
    description: 'Track employee attendance with real-time reporting.',
    status: 'Installed',
    buttonText: 'LEARN MORE',
    iconClass: 'icon-[fluent-mdl2--clock]',
  },
  {
    title: 'Recruitment',
    description: 'Streamline the recruitment process with automation.',
    status: 'Installed',
    buttonText: 'LEARN MORE',
    iconClass: 'icon-[fluent-mdl2--recruitment-management]',
  },
  {
    title: 'Benefits Management',
    description: 'Easily manage employee benefits and compliance.',
    status: 'Activate',
    buttonText: 'LEARN MORE',
    iconClass: 'icon-[fluent-mdl2--heart]',
  },
  {
    title: 'Training',
    description: 'Facilitate employee training and development programs.',
    status: 'Installed',
    buttonText: 'LEARN MORE',
    iconClass: 'icon-[fluent-mdl2--education]',
  },
  {
    title: 'Performance Management',
    description: 'Evaluate and improve employee performance.',
    status: 'Installed',
    buttonText: 'LEARN MORE',
    iconClass: 'icon-[fluent-mdl2--task-manager]',
  },
  {
    title: 'Employee Self-Service',
    description: 'Allow employees to manage their own information.',
    status: 'Installed',
    buttonText: 'LEARN MORE',
    iconClass: 'icon-[flowbite--user-settings-outline]',
  },
  {
    title: 'Time-Off Management',
    description: 'Manage employee leave requests and approvals.',
    status: 'Activate',
    buttonText: 'LEARN MORE',
    iconClass: 'icon-[fluent-mdl2--calendar-settings]',
  },
  {
    title: 'Expense Management',
    description: 'Track and manage employee expenses.',
    status: 'Activate',
    buttonText: 'LEARN MORE',
    iconClass: 'icon-[healthicons--money-bag]',
  },
  {
    title: 'Onboarding',
    description: 'Simplify the onboarding process for new hires.',
    status: 'Installed',
    buttonText: 'LEARN MORE',
    iconClass: 'icon-[healthicons--default]',
  },
  {
    title: 'Offboarding',
    description: 'Manage exit processes and final paperwork.',
    status: 'Installed',
    buttonText: 'LEARN MORE',
    iconClass: 'icon-[healthicons--default]',
  },
  {
    title: 'Employee Surveys',
    description: 'Conduct surveys to gather employee feedback.',
    status: 'Activate',
    buttonText: 'LEARN MORE',
    iconClass: 'icon-[healthicons--default]',
  },
  {
    title: 'Health & Safety',
    description: 'Ensure compliance with health and safety regulations.',
    status: 'Activate',
    buttonText: 'LEARN MORE',
    iconClass: 'icon-[healthicons--default]',
  },
  {
    title: 'Compliance Management',
    description: 'Maintain regulatory compliance and reporting.',
    status: 'Activate',
    buttonText: 'LEARN MORE',
    iconClass: 'icon-[healthicons--default]',
  },
  {
    title: 'Project Management',
    description: 'Plan, execute, and monitor projects effectively.',
    status: 'Installed',
    buttonText: 'LEARN MORE',
    iconClass: 'icon-[healthicons--default]',
  },
  {
    title: 'Document Management',
    description: 'Organize and manage employee documents securely.',
    status: 'Activate',
    buttonText: 'LEARN MORE',
    iconClass: 'icon-[healthicons--default]',
  },
  {
    title: 'Succession Planning',
    description: 'Prepare for future leadership needs and talent gaps.',
    status: 'Installed',
    buttonText: 'LEARN MORE',
    iconClass: 'icon-[healthicons--default]',
  },
  {
    title: 'HR Analytics',
    description: 'Analyze HR metrics for data-driven decisions.',
    status: 'Activate',
    buttonText: 'LEARN MORE',
    iconClass: 'icon-[healthicons--default]',
  },
  {
    title: 'Employee Engagement',
    description: 'Enhance employee satisfaction and retention strategies.',
    status: 'Activate',
    buttonText: 'LEARN MORE',
    iconClass: 'icon-[healthicons--default]',
  },
];

const columns = [
  { key: 'check', label: 'check', sort: false, check: true },
  { key: 'action', label: 'Action', sort: false, width: '100px', class: 'text-center' },
  { key: 'title', label: '[Title]', sort: true },
  { key: 'description', label: 'Description', sort: true },
  { key: 'status', label: 'Status', sort: false, width: '100px', class: 'text-center' },
];

onMounted(() => {
  tableStore.updateTotalRecords(hrmsCards.length);
});

const isDeleteDialogVisible = ref(false);
const isDrawerVisible = ref(false);
const isEditMode = ref(false);
const currentEntry = ref<any>(null);
const isDropdownOpen = ref(false);

const openModal = (action: string, row: any) => {
  currentEntry.value = row;
  isEditMode.value = action === 'edit';
  isDrawerVisible.value = true;
};

const closeDrawer = () => {
  isDrawerVisible.value = false;
  isEditMode.value = false;
  currentEntry.value = null;
};

const saveEntry = () => {
  closeDrawer();
};

const onDelete = () => {
  isDeleteDialogVisible.value = true;
};

const onConfirmDelete = () => {
  isDeleteDialogVisible.value = false;
};

const onCancelDelete = () => {
  isDeleteDialogVisible.value = false;
};

const onSave = () => {
  saveEntry();
};
</script>

<template>
  <div>
    <DataTable :data="hrmsCards" :columns="columns" :check-column="true">
      <template #status="{ row }">
        <span :class="row.status === 'Installed' ? 'bg-green-100 text-green-700' : 'bg-yellow-100 text-yellow-700'"
          class="px-1 py-0.5 rounded-full">
          {{ row.status }}
        </span>
      </template>
      <template #action="{ row }">
        <div class="flex justify-center space-x-2">
          <button class="grid-action-btn hover-btn-warning" @click="openModal('view', row)">
            <span class="icon-[ep--view]"></span>
          </button>
          <button class="grid-action-btn hover-btn-primary" @click="openModal('edit', row)">
            <span class="icon-[akar-icons--edit]"></span>
          </button>
          <button class="grid-action-btn hover-btn-danger" @click="onDelete">
            <span class="icon-[hugeicons--delete-02]"></span>
          </button>
        </div>
      </template>
    </DataTable>
    <ConfirmationDialog 
      v-if="isDeleteDialogVisible" 
      title="Confirm Delete"
      message="Are you sure you want to delete this entry?" 
      :isVisible="isDeleteDialogVisible"
      @confirm="onConfirmDelete" 
      @cancel="onCancelDelete" 
      />
    <Drawer :isOpen="isDrawerVisible" :title="isEditMode ? 'Edit Entry' : 'View Entry'" position="right" size="w-1/4">
      <template #header>
        <div class="w-full flex justify-between items-center p-4 bg-gray-100 border-b">
          <h2 class="text-lg font-semibold text-gray-800">
            {{ isEditMode ? "Edit" : "View" }}
          </h2>
          <button @click="closeDrawer" class="text-gray-400 hover:text-gray-600 focus:outline-none">
            <span class="icon-[fluent--dismiss-20-filled] h-4 w-4"></span>
          </button>
        </div>
      </template>
      <template #default>
        <div class="flex flex-col h-full">
          <div v-if="currentEntry" class="p-4 overflow-y-auto flex-grow">
            <div v-if="isEditMode">
              <span class="font-semibold text-gray-700">Title:</span>
              <input v-model="currentEntry.title" placeholder="Enter Title" class="w-full px-3 py-1 mb-4 input-complete"
                required />
              <span class="font-semibold text-gray-700">Description:</span>
              <textarea v-model="currentEntry.description" placeholder="Enter Description"
                class="w-full px-3 py-1 mb-4 input-complete" rows="3" required></textarea>
            </div>
            <div v-else>
              <span class="font-semibold text-gray-700">Title:</span>
              <input v-model="currentEntry.title" placeholder="Enter Title" class="w-full px-3 py-1 mb-4 input-complete"
                disabled />
              <span class="font-semibold text-gray-700">Description:</span>
              <textarea v-model="currentEntry.description" placeholder="Enter Description"
                class="w-full px-3 py-1 mb-4 input-complete" rows="3" disabled></textarea>
            </div>

            <div class="mb-4 relative">
              <label class="block text-gray-700 font-semibold" for="status">Status:</label>
              <div v-if="isEditMode" class="relative">
                <select v-model="currentEntry.status" id="status" class="w-full input-complete px-3 py-1"
                  @focus="isDropdownOpen = true" @blur="isDropdownOpen = false">
                  <option value="Activate">Activate</option>
                  <option value="Installed">Installed</option>
                </select>
                <span v-if="isDropdownOpen"
                  class="icon-[iconamoon--arrow-up-2] absolute right-3 top-3 text-gray-600"></span>
                <span v-else class="icon-[iconamoon--arrow-down-2] absolute right-3 top-3 text-gray-600"></span>
              </div>
              <div v-else>
                <span class="text-gray-600">{{ currentEntry.status === "Activate" ? "Activate" : "Installed" }}</span>
              </div>
            </div>
          </div>
          <div class="flex justify-end p-4 border-t">
            <button class="btn-secondary px-3 py-1 mr-2" @click="closeDrawer">Close</button>
            <button class="btn-primary px-3 py-1" v-if="isEditMode" @click="onSave">Save</button>
          </div>
        </div>
      </template>
    </Drawer>
  </div>
</template>