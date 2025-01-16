<script setup lang="ts">
import { ref } from 'vue';

const loadingText = ref('Be patient till process is complete');
</script>

<template>
    <div class="fixed inset-0 bg-gray-100 bg-opacity-50 flex items-center justify-center">
        <div class="p-6 max-w-sm bg-white rounded-sm shadow-lg flex items-center space-x-4">
            <div class="shrink-0">
                <div
                    class="w-12 h-12 rounded-full animate-spin border-8 border-solid border-purple-500 border-t-transparent">
                </div>
            </div>
            <div>
                <div class="text-xl font-medium text-black">Loading...</div>
                <p class="text-slate-500">{{ loadingText }}</p>
            </div>
        </div>
    </div>
</template>
