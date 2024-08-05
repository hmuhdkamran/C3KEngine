<script setup lang="ts">
import Moduleheader from '@/layouts/components/dashboardheader.vue';

</script>

<template>
    <div class="flex justify-center items-center h-screen w-screen space-x-12">
        <a href="/app/dashboard"
            class="w-64 p-4 py-6 bg-white shadow-lg rounded-2xl transition transform hover:scale-105 duration-200">
            <div class="flex flex-col items-center justify-center">
                <div class="relative w-24 h-24 bg-violet-200 rounded-xl">
                    <span class="icon-[fluent-mdl2--recruitment-management] absolute w-8 h-8 text-violet-700 transform -translate-x-1/2 
                    -translate-y-1/2 left-1/2 top-1/2"></span>
                </div>
                <p class="mt-4 mb-4 text-xl font-medium text-gray-800">
                    HRMS
                </p>
                <p class="px-2 text-center text-gray-400">
                    Our HRMS software streamlines your HR processes, making it easier to manage your workforce
                    effectively and efficiently.
                </p>
            </div>
        </a>

        <a href="/app/dashboard"
            class="w-64 p-4 py-6 bg-white shadow-lg rounded-2xl transition transform hover:scale-105 duration-200">
            <div class="flex flex-col items-center justify-center">
                <div class="relative w-24 h-24 bg-violet-200 rounded-full">
                    <span
                        class="icon-[vaadin--shop] absolute w-8 h-8 text-violet-700 transform -translate-x-1/2 -translate-y-1/2 left-1/2 top-1/2"></span>
                </div>
                <p class="mt-4 mb-4 text-xl font-medium text-gray-800">
                    Retail
                </p>
                <p class="px-2 text-center text-gray-400">
                    Elevate your retail operations with our cutting-edge software, designed to streamline
                    processes and enhance customer satisfaction.
                </p>
            </div>
        </a>

        <a href="/app/dashboard"
            class="w-64 p-4 py-6 bg-white shadow-lg rounded-2xl transition transform hover:scale-105 duration-200">
            <div class="flex flex-col items-center justify-center">
                <div class="relative w-24 h-24 bg-violet-200 rounded-full">
                    <span
                        class="icon-[mdi--office-building-settings-outline] absolute w-8 h-8 text-violet-700 transform -translate-x-1/2 
                        -translate-y-1/2 left-1/2 top-1/2"></span>
                </div>
                <p class="mt-4 mb-4 text-xl font-medium text-gray-800">
                    Production
                </p>
                <p class="px-2 text-center text-gray-400">
                    I therefore look forward to further developments in this area so that we can streamline our
                    processes and improve overall production efficiency.

                </p>
            </div>
        </a>
    </div>
    <Moduleheader />
</template>

<route lang="yaml">
    meta:
      layout: auth
      action: read
  </route>
