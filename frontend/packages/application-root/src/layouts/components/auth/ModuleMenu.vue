<script setup lang="ts">
import { ref, computed, defineProps, defineEmits, watch } from 'vue';

const props = defineProps<{
    selectedCardTitle: string;
    showModuleDropdown: boolean;
}>();

const emit = defineEmits(['toggleModuleDropdown']);

const searchQuery = ref('');

const services = ref([
    { name: 'Authentication Service', icon: 'icon-[mdi--account-check]', description: 'Manage user authentication' },
    { name: 'Business Setup Service', icon: 'icon-[mdi--office-building]', description: 'Setup business essentials' },
    { name: 'HRMS Service', icon: 'icon-[clarity--employee-group-solid]', description: 'Human resource management' },
    { name: 'Retail Service', icon: 'icon-[mdi--store]', description: 'Manage retail operations' },
    { name: 'Point of Sale Service', icon: 'icon-[mdi--cash-register]', description: 'Point of Sale functionality' },
    { name: 'Supply Chain Service', icon: 'icon-[mdi--truck-delivery]', description: 'Manage supply chain' },
    { name: 'Finance Service', icon: 'icon-[mdi--currency-usd]', description: 'Financial services' },
]);

const filteredServices = computed(() =>
    services.value.filter(service =>
        service.name.toLowerCase().includes(searchQuery.value.toLowerCase())
    )
);

watch(() => props.showModuleDropdown, (newVal) => {
    showDropdown.value = newVal;
});

const showDropdown = ref(props.showModuleDropdown);

function toggleModuleDropdown() {
    emit('toggleModuleDropdown');
}
</script>

<template>
    <div class="relative">
        <button @click="toggleModuleDropdown"
            class="flex items-center space-x-1 font-semibold text-white hover:text-gray-300 focus:outline-none">
            <span v-if="!props.selectedCardTitle">Dashboard</span>
            <h1 v-else >{{ props.selectedCardTitle }}
            </h1>
            <span class="icon-[ri--arrow-drop-down-line] text-white h-7 w-7"></span>
        </button>
        <transition name="fade">
            <div v-if="props.showModuleDropdown"
                class="absolute left-0 mt-2 w-[75vw] sm:w-[600px] bg-white rounded-lg shadow-lg z-50 p-2 sm:p-4 
                grid grid-cols-1 sm:grid-cols-2 gap-2 sm:gap-3 transform origin-top scale-95 transition-transform duration-300">
                <div class="col-span-full mb-2 relative">
                    <input v-model="searchQuery" type="text" placeholder="Search Modules..."
                        class="input-bottom pl-2 w-full" />
                </div>
                <div v-if="filteredServices.length" v-for="(service, index) in filteredServices" :key="index"
                    class="flex items-center cursor-pointer bg-gray-50 border border-gray-200 rounded-sm border-l-4 p-1 sm:p-4 hover:bg-gray-100 hover:shadow-md transition-all duration-300 relative">
                    <span :class="service.icon + ' text-violet-600 h-5 w-5 sm:h-8 sm:w-8 mr-4'"></span>
                    <div>
                        <h4 class="text-sm sm:text-md font-semibold text-gray-700">{{ service.name }}</h4>
                        <p class="text-xs sm:text-sm text-gray-500">{{ service.description }}</p>
                    </div>
                    <span v-if="service.name === 'HRMS Service'"
                        class="absolute top-2 right-2 text-xs text-white bg-red-500 rounded-full px-1 py-1">New</span>
                </div>
                <div v-if="!filteredServices.length" class="text-sm col-span-full text-center text-gray-500">
                    <p>No modules found.</p>
                </div>
            </div>
        </transition>
    </div>
</template>

<style scoped>
.fade-enter-active,
.fade-leave-active {
    transition: opacity 0.5s ease, transform 0.5s ease;
}

.fade-enter-from,
.fade-leave-to {
    opacity: 0;
    transform: scale(0.95);
}

@media (min-width: 640px) {

    .fade-enter-active,
    .fade-leave-active {
        transition: opacity 0.3s ease, transform 0.3s ease;
    }
}
</style>
