<script setup lang="ts">
import {Filter} from 'c3k-library';
import { computed, ref } from 'vue';

const selectedCardTitle = ref('');

const pageHeading = computed(() => {
    return selectedCardTitle.value !== '' ? selectedCardTitle.value : 'Apps';
});

</script>

<template>
    <div class="text-blueGray-700 antialiased">
        <div class="bg-white mt-12 flex flex-col">
            <div
                class="border-b border-gray-300 py-2 px-4 flex flex-col md:flex-row justify-between shadow-md w-full space-y-4 md:space-y-0 md:space-x-8">
                <div class="w-full md:w-1/2 flex flex-col justify-center space-y-6">
                    <div class="px-3">
                        <h1 class="text-2xl font-bold text-gray-800">{{ pageHeading }}</h1>
                    </div>
                    <nav class="sm:text-md px-2 text-sm flex space-x-2">
                        <!-- <div @click.prevent="goToMain"
                            class="hover:underline cursor-pointer text-gray-600 flex items-center">
                            <i class="icon-[mdi--home-outline] mr-1 text-gray-500"></i> Apps
                        </div>
                        <span class="text-gray-400">/</span>
                        <div v-if="selectedCardCategory !== 'All'" class="hover:underline cursor-pointer text-gray-600">
                            {{ selectedCardCategory }}
                        </div>
                        <div v-else class="text-gray-600">
                            {{ selectedCategory }}
                        </div>
                        <span class="text-gray-400" v-if="selectedCardTitle !== ''">/</span>
                        <span v-if="selectedCardTitle !== ''" class="hover:underline cursor-pointer text-gray-600">{{
                            selectedCardTitle }}</span> -->
                    </nav>
                </div>
                <div class="w-full md:w-1/2 flex flex-col space-y-2">
                    <Filter>
                        <template #action>
                            <button
                                class="flex items-center btn-primary px-2 py-1 rounded-sm shadow-md">
                                <i class="icon-[fluent--filter-16-filled] mr-1"></i> Filters
                            </button>
                            <button
                                class="flex items-center btn-secondary px-2 py-1 rounded-sm shadow-md">
                                <i class="icon-[fluent--group-24-filled] mr-1"></i> Group By
                            </button>
                            <button
                                class="flex items-center btn-danger px-2 py-1 rounded-sm shadow-md">
                                <i class="icon-[mdi--star-outline] mr-1"></i> Favorites
                            </button>
                        </template>
                    </Filter>
                </div>
            </div>


            <RouterView v-slot="{ Component }">
                <Transition name="fade" mode="out-in">
                    <Component :is="Component" />
                </Transition>
            </RouterView>
        </div>
    </div>
</template>