<script setup lang="ts">
import { useApplicationStore } from '@/stores/counter';
import { Filter } from 'c3k-library';
import { computed, ref } from 'vue';

const store = useApplicationStore();

const selectedCardTitle = ref('Manaloom Afrad');

const pageHeading = computed(() => {
    return selectedCardTitle.value !== '' ? selectedCardTitle.value : 'Apps';
});


</script>

<template>
    <div class="flex h-screen bg-gray-100">
        <div class="absolute w-full bg-gradient-to-r from-violet-600 to-violet-500 min-h-75">
            <div
                class="py-2 px-6 flex flex-col md:flex-row justify-between ml-auto mt-12 space-y-4 md:space-y-0 md:space-x-8">
                <div class="w-full md:w-1/2 flex flex-col justify-center space-y-6">
                    <div class="px-3">
                        <h1 class="text-2xl font-bold text-gray-100">{{ pageHeading }}</h1>
                    </div>
                    <nav class="sm:text-md px-2 text-sm flex space-x-2">
                        <div class="hover:underline cursor-pointer text-gray-100 flex items-center">
                            <i class="icon-[mdi--home-outline] mr-1 text-gray-50"></i> Apps
                        </div>
                        <span class="text-gray-100">/</span>
                        <span class="hover:underline cursor-pointer text-gray-100">Testing</span>
                    </nav>
                </div>
                <div class="w-full md:w-1/2 flex flex-col space-y-2">
                    <Filter>
                        <template #action>
                            <button class="flex items-center btn-primary px-2 py-1 rounded-sm shadow-md" @click="store.toggleDrawer = true">
                                <i class="icon-[fluent--filter-16-filled] mr-1"></i> Add
                            </button>
                        </template>
                    </Filter>
                </div>
            </div>
        </div>
        <div class="w-full px-6 py-6 mt-32 mx-auto">
            <div class="flex justify-end mr-2">
                <RouterView v-slot="{ Component }">
                    <Transition name="fade" mode="out-in">
                        <Component :is="Component" />
                    </Transition>
                </RouterView>
            </div>
        </div>
    </div>
</template>