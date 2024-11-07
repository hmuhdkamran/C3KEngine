<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { Chart, registerables } from 'chart.js';

Chart.register(...registerables);

const chartRef = ref(null);
let rolesChart = null;

const roles = ref({
    Admin: 3,
    User: 5,
    Moderator: 2,
    Guest: 1,
});

const setupChart = () => {
    if (chartRef.value) {
        rolesChart = new Chart(chartRef.value, {
            type: 'pie',
            data: {
                labels: Object.keys(roles.value),
                datasets: [
                    {
                        data: Object.values(roles.value),
                        backgroundColor: ['#ff6384', '#36a2eb', '#ffce56', '#4bc0c0'],
                        borderWidth: 2,
                        borderColor: '#ffffff',
                    },
                ],
            },
            options: {
                responsive: true,
                maintainAspectRatio: false,
                plugins: {
                    legend: {
                        display: true,
                        position: 'top',
                        labels: {
                            boxWidth: 10,
                            padding: 20,
                            font: {
                                size: 14,
                                weight: 'bold',
                            },
                        },
                    },
                    tooltip: {
                        callbacks: {
                            label: (tooltipItem) => {
                                const label = tooltipItem.label || '';
                                const value = tooltipItem.raw || 0;
                                return `${label}: ${value}`;
                            },
                        },
                    },
                },
            },
        });
    }
};

onMounted(() => setupChart());
</script>

<template>
    <div class="p-4 bg-white rounded-md shadow-lg border border-gray-200">
        <h3 class="text-lg font-semibold text-gray-800">Roles Distribution</h3>
        <div class="relative h-72">
            <canvas ref="chartRef"></canvas>
        </div>
    </div>
</template>

<style scoped>
</style>
