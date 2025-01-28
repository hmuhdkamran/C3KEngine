<script setup lang="ts">
import { config } from "c3-library";

import {image1, image3, image2, image4} from '@/assets/images/images'
</script>

<template>
    <section>
        <div class="relative p-4 px-6 mx-auto bg-white dark:bg-gray-800 sm:px-6 md:px-8 lg:px-8 py-26 lg:mt-20">
            <div class="container mx-auto px-4">
                <div class="lg:grid lg:grid-flow-row-dense lg:grid-cols-1 lg:gap-8 lg:items-center">
                    <div class="ml-16 lg:col-start-2 lg:max-w-2xl lg:ml-36">
                        <p class="text-base font-semibold leading-6 text-violet-600 uppercase">
                            {{ config.application }}
                        </p>
                        <h4
                            class="mt-2 text-2xl font-extrabold leading-8 text-gray-900 dark:text-white sm:text-3xl sm:leading-9">
                            Interactivity and Efficiency for Your Enterprise
                        </h4>
                        <p class="mt-4 text-lg leading-6 text-gray-500 dark:text-gray-300">
                            Create a seamless and powerful collaborative space for your enterprise. Track, share,
                            and measure
                            efficiently with full control. It’s never been simpler and more effective.
                        </p>
                        <ul class="gap-6 mt-8 md:grid md:grid-cols-2">
                            <li class="mt-6 md:mt-0">
                                <div class="flex">
                                    <span
                                        class="flex items-center justify-center flex-shrink-0 w-6 h-6 text-violet-800 bg-violet-100 rounded-full dark:text-violet-500 dark:bg-transparent">
                                        <span class="fa-solid fa-check"></span>
                                    </span>
                                    <span class="ml-4 text-base font-medium leading-6 text-gray-500 dark:text-gray-200">
                                        Real-time Updates
                                    </span>
                                </div>
                            </li>
                            <li class="mt-6 md:mt-0">
                                <div class="flex">
                                    <span
                                        class="flex items-center justify-center flex-shrink-0 w-6 h-6 text-violet-800 bg-violet-100 rounded-full dark:text-violet-500 dark:bg-transparent">
                                        <span class="fa-solid fa-check"></span>
                                    </span>
                                    <span class="ml-4 text-base font-medium leading-6 text-gray-500 dark:text-gray-200">
                                        Comprehensive Data Tracking
                                    </span>
                                </div>
                            </li>
                            <li class="mt-6 md:mt-0">
                                <div class="flex">
                                    <span
                                        class="flex items-center justify-center flex-shrink-0 w-6 h-6 text-violet-800 bg-violet-100 rounded-full dark:text-violet-500 dark:bg-transparent">
                                        <span class="fa-solid fa-check"></span>
                                    </span>
                                    <span class="ml-4 text-base font-medium leading-6 text-gray-500 dark:text-gray-200">
                                        24/7 Support
                                    </span>
                                </div>
                            </li>
                            <li class="mt-6 md:mt-0">
                                <div class="flex">
                                    <span
                                        class="flex items-center justify-center flex-shrink-0 w-6 h-6 text-violet-800 bg-violet-100 rounded-full dark:text-violet-500 dark:bg-transparent">
                                        <span class="fa-solid fa-check"></span>
                                    </span>
                                    <span class="ml-4 text-base font-medium leading-6 text-gray-500 dark:text-gray-200">
                                        Tips to Optimize Work Time
                                    </span>
                                </div>
                            </li>
                        </ul>
                    </div>
                    <div class="relative mt-12 lg:mt-0 space-y-4">
                        <div class="flex items-end justify-center space-x-4 sm:justify-start">
                            <img class="w-28 sm:w-32 md:w-56 lg:w-72 rounded-lg shadow-lg" :src="image3" alt="1" />
                            <img class="w-36 sm:w-40 md:w-64 lg:w-80 rounded-lg shadow-lg" :src="image2" alt="2" />
                        </div>
                        <div class="flex items-start justify-center space-x-4 sm:justify-start">
                            <img class="w-36 sm:w-40 md:w-64 lg:w-80 rounded-lg shadow-lg" :src="image4" alt="4" />
                            <img class="w-28 sm:w-32 md:w-56 lg:w-72 rounded-lg shadow-lg" :src="image1" alt="3" />
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </section>
</template>
