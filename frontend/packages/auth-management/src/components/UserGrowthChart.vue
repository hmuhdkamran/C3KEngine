<script setup lang="ts">
import { onMounted, ref, computed, watch } from 'vue';
import { Chart, registerables } from 'chart.js';

Chart.register(...registerables);

const chartRef = ref<HTMLCanvasElement | null>(null);
const selectedView = ref<'monthly' | 'yearly'>('monthly');
let userGrowthChart: Chart<'line'> | null = null;

type DataSet = {
    labels: string[];
    data: number[];
    borderColor: string;
    backgroundColor: string;
};

const dataSets: Record<'monthly' | 'yearly', DataSet> = {
    monthly: {
        labels: ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul'],
        data: [10, 20, 15, 25, 30, 28, 50],
        borderColor: '#4caf50',
        backgroundColor: 'rgba(76, 175, 80, 0.3)',
    },
    yearly: {
        labels: ['2020', '2021', '2022', '2023', '2024', '2025'],
        data: [120, 150, 170, 200, 230, 250],
        borderColor: '#ff9800',
        backgroundColor: 'rgba(255, 152, 0, 0.3)',
    }
};

const chartData = computed(() => {
    const viewData = dataSets[selectedView.value];
    return {
        labels: viewData.labels,
        datasets: [
            {
                label: `User Growth (${selectedView.value.charAt(0).toUpperCase() + selectedView.value.slice(1)})`,
                data: viewData.data,
                borderColor: viewData.borderColor,
                backgroundColor: viewData.backgroundColor,
                fill: true,
                tension: 0.4,
                pointRadius: 5,
                pointHoverRadius: 8,
                pointBackgroundColor: viewData.borderColor,
                pointBorderColor: '#ffffff',
            },
        ],
    };
});

const setupChart = () => {
    if (chartRef.value) {
        const ctx = chartRef.value.getContext('2d');
        if (ctx) { 
            userGrowthChart = new Chart(ctx, {
                type: 'line',
                data: chartData.value,
                options: {
                    responsive: true,
                    maintainAspectRatio: false,
                    plugins: {
                        legend: {
                            display: true,
                            position: 'top',
                            labels: {
                                color: '#333',
                            },
                        },
                        tooltip: {
                            enabled: true,
                            backgroundColor: 'rgba(0, 0, 0, 0.7)',
                        },
                    },
                    scales: {
                        x: {
                            title: {
                                display: true,
                                text: 'Time',
                                font: { size: 14, weight: 'bold' },
                                color: '#333',
                            },
                            grid: {
                                display: false,
                            },
                        },
                        y: {
                            beginAtZero: true,
                            title: {
                                display: true,
                                text: 'Number of Users',
                                font: { size: 14, weight: 'bold' },
                                color: '#333',
                            },
                            grid: {
                                color: 'rgba(0, 0, 0, 0.1)',
                            },
                            ticks: {
                                stepSize: 10,
                            },
                        },
                    },
                },
            });
        }
    }
};

watch(chartData, (newData) => {
    if (userGrowthChart) {
        userGrowthChart.data = newData;
        userGrowthChart.update();
    }
});

onMounted(() => setupChart());
</script>

<template>
    <div class="p-6 bg-white rounded-md shadow-md">
        <div class="flex justify-between items-center mb-4">
            <h3 class="text-lg font-semibold text-gray-800">User Growth Over Time</h3>
            <select v-model="selectedView" class="px-3 py-1 border border-gray-300 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-green-500">
                <option value="monthly">Monthly</option>
                <option value="yearly">Yearly</option>
            </select>
        </div>
        <div class="relative h-64">
            <canvas ref="chartRef"></canvas>
        </div>
    </div>
</template>
