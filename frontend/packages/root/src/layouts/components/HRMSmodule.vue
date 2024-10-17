<script setup lang="ts">
import { DataTable, useTableStore } from 'c3k-library';
import { onMounted } from 'vue';
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

const openModal = (row: any) => {
  console.log('Open modal for:', row);
}

</script>

<template>
  <div>
    <!-- <Modules :cardTitle="'HRMS'" :cardsData="hrmsCards" :columns="columns" /> -->
    <DataTable :data="hrmsCards" :columns="columns" :check-column="true">
      <template #status="{ row }">
        <span :class="row.status === 'Installed' ? 'bg-green-100 text-green-700' : 'bg-yellow-100 text-yellow-700'"
          class="px-1 py-0.5 rounded-full">
          {{ row.status }}
        </span>
      </template>
      <template #action="{ row }">
        <div className="flex justify-center space-x-2">
          <button className="grid-action-btn hover-btn-warning" @click="openModal(row)">
            <span className="icon-[ep--view]"></span>
          </button>

          <button className="grid-action-btn hover-btn-primary" @click="openModal(row)">
            <span className="icon-[akar-icons--edit]"></span>
          </button>
          <button className="grid-action-btn hover-btn-danger" @click="openModal(row)">
            <span className="icon-[hugeicons--delete-02]"></span>
          </button>
        </div>
      </template>
    </DataTable>
  </div>
</template>