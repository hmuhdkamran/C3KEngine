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
  { key: 'title', label: 'Title' },
  { key: 'description', label: 'Description' },
  { key: 'status', label: 'Status' }
];

</script>

<template>
  <Modules :cardTitle="'HRMS'" :cardsData="hrmsCards" :columns="columns" />
</template>