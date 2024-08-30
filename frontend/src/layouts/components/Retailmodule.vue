<script setup lang="ts">
import Modules from '../../components/base/modules.vue';

const retailCards = [
    {
        title: 'Inventory Management',
        description: 'Optimize inventory with real-time tracking and automation.',
        status: 'Installed',
        buttonText: 'LEARN MORE',
        iconClass: 'icon-[prime--warehouse]',
    },
    {
        title: 'Point of Sale',
        description: 'Streamline sales with an intuitive POS system.',
        status: 'Activate',
        buttonText: 'LEARN MORE',
        iconClass: 'icon-[mdi--cash-register]',
    },
    {
        title: 'Customer Loyalty',
        description: 'Enhance customer retention with loyalty programs.',
        status: 'Installed',
        buttonText: 'LEARN MORE',
        iconClass: 'icon-[mdi--star-circle-outline]',
    },
    {
        title: 'Order Management',
        description: 'Manage orders from multiple channels efficiently.',
        status: 'Activate',
        buttonText: 'LEARN MORE',
        iconClass: 'icon-[hugeicons--task-done-01]',
    },
];

const columns = [
    { key: 'title', label: 'Title' },
    { key: 'description', label: 'Description' },
    { key: 'status', label: 'Status' }
];
</script>

<template>
    <Modules :cardTitle="'Retail'" :cardsData="retailCards" :columns="columns" />
</template>
