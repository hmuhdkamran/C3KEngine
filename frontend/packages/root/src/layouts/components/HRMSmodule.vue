<script setup lang="ts">
import { DataTable, useTableStore } from "c3k-library";
import { ref, onMounted } from "vue";
import ConfirmationDialog from '@/layouts/components/ConfirmationDialog.vue';
import Dialog from "@/pages/role/useredit-dialog.vue";
import Drawer from "@/pages/role/useredit-drawer.vue";
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
const isFormVisible = ref(false);
const isEditdialog = ref(false);
const currentEntry = ref<any>(null);
const isEditMode = ref(false);
const isDropdownOpen = ref(false);
const formData = ref({ title: '', description: '', status: '' });

const openModal = (action: string, row: any) => {
  currentEntry.value = row;
  formData.value = { ...row };
  
  if (action === 'view') {
    isDrawerVisible.value = true;
    isEditMode.value = false;
  } else if (action === 'edit') {
    isDrawerVisible.value = true;
    isEditMode.value = true;
  } else if (action === 'dialog') {
    isEditdialog.value = true;
    isFormVisible.value = true;
  }
};

const closeDrawer = () => {
  isDrawerVisible.value = false;
  isEditMode.value = false;
};

const saveEntry = (data: any) => {
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
  saveEntry(formData.value);
};

const goBack = () => {
  closeDrawer();
  isFormVisible.value = false;
};
</script>

<template>
  <div>
    <DataTable :data="hrmsCards" :columns="columns" :check-column="true" v-if="!isFormVisible">
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
          <button class="grid-action-btn hover-btn-primary" @click="openModal('dialog', row)">
            <span class="icon-[material-symbols--dialogs-outline]"></span>
          </button>
          <button class="grid-action-btn hover-btn-danger" @click="onDelete">
            <span class="icon-[hugeicons--delete-02]"></span>
          </button>
        </div>
      </template>
    </DataTable>
    <div class="min-h-screen flex flex-col items-center justify-start bg-gray-100 p-8" v-if="isFormVisible">
      <form class="w-full bg-white rounded-md p-6 space-y-8 shadow-lg">
        <div class="bg-gray-50 p-6 rounded-lg shadow-inner mb-6">
          <h1 class="text-lg font-bold text-gray-800 text-center">
            {{ isEditMode ? "Edit Form" : "View Form" }}
          </h1>
          <p class="text-md text-gray-500 text-center mt-2">
            {{ isEditMode ? "Update the form details below." : "Review the form details below." }}
          </p>
        </div>
        <div class="space-y-8">
          <div class="grid grid-cols-1 sm:grid-cols-2 gap-6">
            <div class="w-full">
              <label class="block font-semibold text-gray-700 mb-2" for="title">Title</label>
              <input v-model="formData.title" id="title" placeholder="Enter Title"
                class="w-full input-complete transition-all" :disabled="!isEditMode" required />
            </div>
            <div class="w-full">
              <label class="block font-semibold text-gray-700 mb-2" for="status">Status</label>
              <div class="relative">
                <select v-model="formData.status" id="status" class="w-full input-complete transition-all"
                  :disabled="!isEditMode" @focus="isDropdownOpen = true" @blur="isDropdownOpen = false">
                  <option value="Activate">Activate</option>
                  <option value="Installed">Installed</option>
                </select>
                <span v-if="isDropdownOpen"
                  class="icon-[iconamoon--arrow-up-2] absolute right-4 top-1/2 transform -translate-y-1/2 text-gray-600"></span>
                <span v-else
                  class="icon-[iconamoon--arrow-down-2] absolute right-4 top-1/2 transform -translate-y-1/2 text-gray-600"></span>
              </div>
            </div>
          </div>
          <div class="w-full">
            <label class="block font-semibold text-gray-700 mb-2" for="description">Description</label>
            <textarea v-model="formData.description" id="description" placeholder="Enter Description"
              class="w-full input-complete transition-all" rows="6" :disabled="!isEditMode" required></textarea>
          </div>
        </div>
        <div class="flex justify-end space-x-2 mt-8">
          <button @click="goBack" class="px-3 py-1 btn-secondary transition-all">
            Back
          </button>
          <button v-if="isEditMode" type="button" @click="onSave" class="px-3 py-1 btn-primary transition-all">
            Save
          </button>
        </div>
      </form>
    </div>

    <Dialog
      v-if="isEditdialog"
      :isVisible="isFormVisible"
      :entryData="currentEntry"
      :isEdit="isEditdialog"
      @Close="isFormVisible = false"
      @onSave="saveEntry"
    />

    <Drawer
      v-if="isDrawerVisible"
      :isVisible="isDrawerVisible"
      :entryData="currentEntry"
      :isEdit="isEditMode"
      @Close="closeDrawer"
      @onSave="saveEntry"
    />

    <ConfirmationDialog
      v-if="isDeleteDialogVisible"
      title="Confirm Delete"
      message="Are you sure you want to delete this entry?"
      :isVisible="isDeleteDialogVisible"
      @confirm="onConfirmDelete"
      @cancel="onCancelDelete"
    />
  </div>
</template>
