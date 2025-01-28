<script setup lang="ts">
import { config } from "c3-library";
import { hero } from '@/assets/images/images';
</script>

<template>
    <section class="header relative items-center flex h-screen">
        <div class="container mx-auto items-center flex flex-wrap">
            <div class="absolute w-full lg:w-1/2 inset-y-0 lg:right-0 hidden lg:block">
                <span
                    class="absolute -left-12 top-24 w-40 h-40 rounded-full bg-gradient-to-r from-purple-500 to-violet-500 blur-3xl opacity-50 transform rotate-45 lg:block hidden"></span>
            </div>
            <span
                class="w-3/12 lg:w-2/12 aspect-square bg-gradient-to-r from-violet-600 to-purple-400 absolute -top-5 lg:left-0 rounded-full blur-3xl opacity-50 transform rotate-45"></span>

            <div class="w-full md:w-8/12 lg:w-6/12 px-4 relative z-10">
                <div class="pt-16 md:pt-24 lg:pt-32">
                    <h2 class="font-semibold text-3xl md:text-4xl lg:text-5xl xl:text-6xl text-gray-800">
                        The {{ config.application.slice(0, Math.ceil(config.application.length / 2)) }}
                        <span class="bg-gradient-to-tr from-violet-800 to-violet-400 bg-clip-text text-transparent">
                            {{ config.application.slice(Math.ceil(config.application.length / 2)) }}
                        </span>
                        for Any Business
                    </h2>
                    <p class="mt-4 md:mt-6 text-base md:text-lg lg:text-xl leading-relaxed text-gray-500 max-w-md">
                        A comprehensive ERP solution designed for all industries—whether it's retail, education,
                        healthcare, or manufacturing. Our cloud-based and on-premise platform covers every aspect of
                        business management, streamlining your processes while ensuring compliance with best
                        practices worldwide.
                    </p>
                    <a class="mt-8 md:mt-12 group relative inline-flex items-center overflow-hidden rounded btn-gradient px-6 md:px-8 py-2 md:py-3 text-white focus:outline-none focus:ring active:bg-violet-700"
                        href="#">
                        <span class="absolute -end-full transition-all group-hover:end-4">
                            <span class="fa-regular fa-arrow-right mt-2"
                                style="width: 24px; height: 24px;"></span>
                        </span>
                        <span class="text-sm font-medium transition-all group-hover:me-4">Explore All Modules</span>
                    </a>
                </div>
            </div>

            <!-- Illustration Section -->
            <div class="w-full lg:w-6/12 mt-10 lg:mt-0">
                <div class="text-center lg:text-right lg:ml-auto">
                    <div class="relative inline-block z-10 pt-11 lg:pt-0">
                        <img :src="hero" alt="ERP Illustration" class="max-w-full lg:ml-auto" />
                    </div>
                </div>
            </div>
        </div>
    </section>
</template>
