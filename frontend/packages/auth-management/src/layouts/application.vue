<script setup lang="ts">
import { Filter, useApplicationEventStore, AppSidebar } from 'c3k-library';
import { computed, ref } from 'vue';

const selectedCardTitle = ref('');
const pageHeading = computed(() => `Manage ${selectedCardTitle.value} Users`);

const eventStore = useApplicationEventStore();

const openAddRecord = () => {
    eventStore.setDrawerEvent({ Open: true, Title: 'Add Record', OperationType: 'add' });
};
const exportData = () => { };

const refreshData = () => { };
</script>

<template>
    <div>        
        <div
            class="border-b border-gray-300 mt-12 py-2 px-4 flex flex-col md:flex-row justify-between shadow-md w-full space-y-4 md:space-y-0 md:space-x-8">
            <div class="w-full md:w-1/2 flex flex-col justify-center space-y-6">
                <div class="px-3">
                    <h1 class="text-2xl font-bold text-gray-800">{{ pageHeading }}</h1>
                </div>
                <nav class="sm:text-md px-2 text-sm flex space-x-2">
                    <div class="hover:underline cursor-pointer text-gray-800 flex items-center">
                        <i class="icon-[mdi--home-outline] mr-1 text-gray-800"></i> Apps
                    </div>
                    <span class="text-gray-700">/</span>
                    <span class="hover:underline cursor-pointer text-gray-700">Testing</span>
                </nav>
            </div>
            <div class="w-full md:w-1/2 flex flex-col space-y-2">
                <Filter>
                    <template #action>
                        <button class="btn-primary px-3 py-1 rounded-sm flex items-center" @click="openAddRecord">
                            <span class="icon-[fa--plus-circle] mr-2 text-white"></span> Add
                        </button>
                        <button class="btn-primary px-3 py-1 rounded-sm flex items-center" @click="exportData">
                            <span class="icon-[pajamas--import] mr-2 text-white"></span>Export
                        </button>
                        <button class="btn-primary px-3 py-1 rounded-sm flex items-center" @click="refreshData">
                            <span class="icon-[tabler--refresh] mr-2 text-white"></span>Load
                        </button>
                    </template>
                </Filter>
            </div>
        </div>
        <!-- <div class="w-full px-6 py-2 mx-auto"> -->
        <div class="bg-white p-4">
            <RouterView v-slot="{ Component }">
                <Transition name="fade" mode="out-in">
                    <Component :is="Component" />
                </Transition>
            </RouterView>
        </div>
        <!-- </div> -->
    </div>
</template>