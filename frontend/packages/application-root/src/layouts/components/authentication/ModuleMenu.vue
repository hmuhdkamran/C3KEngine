<script setup lang="ts">
import type { UserProduct } from '@/models/user-products';
import { store } from '@/stores';
import { Icon } from '@iconify/vue';
import { LocalStorageHelper } from 'c3k-library';
import { ref, computed, defineProps, defineEmits, watch } from 'vue';
import { useRouter } from 'vue-router';

const props = defineProps<{
    selectedCardTitle: string;
    showModuleDropdown: boolean;
}>();

const router = useRouter()

const emit = defineEmits(['toggleModuleDropdown']);

const searchQuery = ref('');
const products = store.userModules as UserProduct[];

const filteredServices = computed(() =>
    products.length > 0 ? products.filter(item =>
        item.FullName.toLowerCase().includes(searchQuery.value.toLowerCase())
    ) : []
);

watch(() => props.showModuleDropdown, (newVal) => {
    showDropdown.value = newVal;
});

const showDropdown = ref(props.showModuleDropdown);

function toggleModuleDropdown() {
    emit('toggleModuleDropdown');
}

function handleCardClick(card: UserProduct) {
    LocalStorageHelper.set("application", `c3k-${card.Abbreviation.replace('/', '-')}`)
    router.replace(`/c3k-${card.Abbreviation.replace('/', '-')}`);
}

</script>

<template>
    <div class="relative">
        <button @click="toggleModuleDropdown"
            class="flex items-center space-x-1 font-semibold text-white hover:text-gray-300 focus:outline-none">
            <span v-if="!props.selectedCardTitle">Dashboard</span>
            <h1 v-else>{{ props.selectedCardTitle }}
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
                    class="flex items-center cursor-pointer bg-gray-50 border border-gray-200 rounded-sm border-l-4 p-1 sm:p-4 hover:bg-gray-100 hover:shadow-md transition-all duration-300 relative"
                    @click.prevent="handleCardClick(service)">
                    <Icon :icon="service.Icon" class="text-violet-600 h-5 w-5 sm:h-8 sm:w-8 mr-4" />
                    <div>
                        <h4 class="text-sm sm:text-md font-semibold text-gray-700">{{ service.FullName }}</h4>
                        <p class="text-xs sm:text-sm text-gray-500">{{ service.Description }}</p>
                    </div>
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
