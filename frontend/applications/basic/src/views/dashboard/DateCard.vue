<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { Card } from "c3-library";

const now = ref(new Date());

const updateTime = () => {
    now.value = new Date();
};

let timer: number;
onMounted(() => {
    timer = window.setInterval(updateTime, 1000);
});

onUnmounted(() => {
    clearInterval(timer);
});

// Computed property for hours and minutes (HH:MM)
const hoursMinutes = computed(() => {
    const h = now.value.getHours().toString().padStart(2, '0');
    const m = now.value.getMinutes().toString().padStart(2, '0');
    return `${h}:${m}`;
});

// Computed property for seconds (small font)
const seconds = computed(() => {
    return now.value.getSeconds().toString().padStart(2, '0');
});

// Helper function to get date suffix (e.g., 'st', 'nd', 'rd', 'th')
const getDateSuffix = (day: number) => {
    if (day >= 11 && day <= 13) return 'th';
    switch (day % 10) {
        case 1: return 'st';
        case 2: return 'nd';
        case 3: return 'rd';
        default: return 'th';
    }
};

// Computed property for full date display (e.g., "Sunday, 15th January 2025")
const fullDate = computed(() => {
    const weekday = now.value.toLocaleDateString('en-US', { weekday: 'long' });
    const day = now.value.getDate();
    const suffix = getDateSuffix(day);
    const month = now.value.toLocaleDateString('en-US', { month: 'long' });
    const year = now.value.getFullYear();
    return `${weekday}, ${day}${suffix} ${month} ${year}`;
});
</script>

<template>
    <Card shadow="shadow-sm" hoverShadow="shadow-lg" class="bg-white p-4">
        <div class="w-full flex flex-col justify-center items-center">
            <div class="p-4 pb-0 flex-1 text-center">
                <!-- Time display: HH:MM in large font with seconds in a smaller font -->
                <div class="text-6xl font-bold">
                    {{ hoursMinutes }}<span class="text-2xl align-super">:{{ seconds }}</span>
                </div>
                <!-- Detailed date display -->
                <div class="mt-4 text-lg">
                    {{ fullDate }}
                </div>
            </div>
        </div>
    </Card>
</template>