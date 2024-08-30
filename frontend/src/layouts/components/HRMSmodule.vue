<script setup lang="ts">
import Modules from '../../components/base/modules.vue';

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
];

const columns = [
  { key: 'title', label: 'Title' },
  { key: 'description', label: 'Description' },
  { key: 'status', label: 'Status' }
];

</script>

<template>
  <Modules :cardTitle="'HRMS'" :cardsData="hrmsCards" :columns="columns" />
</template>