<script setup lang="ts">
import Modules from '../../components/base/modules.vue';

const productionCards = [
    {
        title: 'Production Planning',
        description: 'Optimize production schedules for maximum efficiency.',
        status: 'Installed',
        buttonText: 'LEARN MORE',
        iconClass: 'icon-[mdi--chart-timeline]',
    },
    {
        title: 'Quality Control',
        description: 'Ensure product quality with automated checks.',
        status: 'Activate',
        buttonText: 'LEARN MORE',
        iconClass: 'icon-[mdi--check-circle-outline]',
    },
    {
        title: 'Resource Management',
        description: 'Manage resources effectively to meet production goals.',
        status: 'Installed',
        buttonText: 'LEARN MORE',
        iconClass: 'icon-[mdi--account-group-outline]',
    },
    {
        title: 'Supply Chain Management',
        description: 'Coordinate supply chain operations for efficiency.',
        status: 'Activate',
        buttonText: 'LEARN MORE',
        iconClass: 'icon-[mdi--truck-outline]',
    },
];
const columns = [
    { key: 'title', label: 'Title' },
    { key: 'description', label: 'Description' },
    { key: 'status', label: 'Status' }
];
</script>

<template>
    <Modules :cardTitle="'Production'" :cardsData="productionCards" :columns="columns" />
</template>
