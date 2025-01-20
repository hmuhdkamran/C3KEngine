<script setup lang="ts">
import { config } from "c3-library";
import { avatar} from '@/assets/images/images'
</script>

<template>
    <div class="bg-white dark:bg-gray-800 py-16 lg:py-20">
        <div class="container mx-auto px-4 sm:px-6 lg:px-8">
            <h2 class="pb-4 text-3xl md:text-5xl font-bold text-gray-900 dark:text-white text-center">Get Started
                with Our {{ config.application }} Today</h2>
            <p class="mx-auto max-w-3xl pb-8 text-gray-700 dark:text-gray-300 text-lg md:text-xl text-center">
                Experience a seamless transition to our {{ config.application }}. With intuitive features and
                user-friendly
                design, our platform simplifies enterprise management, making it easier for your team to stay
                organized and
                efficient.
            </p>
            <div class="lg:flex items-start justify-center lg:space-x-12">
                <div class="lg:max-w-xl space-y-4 text-lg md:text-xl leading-7">
                    <div class="flex items-center space-x-3 text-gray-700 dark:text-gray-300 font-semibold">
                        <div class="shrink-0 text-violet-500">
                            <span class="icon-[mdi--tick-decagram]" style="width: 36px; height: 36px;"></span>
                        </div>
                        <div>Free to get started with comprehensive features</div>
                    </div>
                    <div class="flex items-center space-x-3 text-gray-700 dark:text-gray-300 font-semibold">
                        <div class="shrink-0 text-violet-500">
                            <span class="icon-[mdi--tick-decagram]" style="width: 36px; height: 36px;"></span>
                        </div>
                        <div>Easy migration checklist</div>
                    </div>
                    <div class="flex items-center space-x-3 text-gray-700 dark:text-gray-300 font-semibold">
                        <div class="shrink-0 text-violet-500">
                            <span class="icon-[mdi--tick-decagram]" style="width: 36px; height: 36px;"></span>
                        </div>
                        <div>Personalized support from our team</div>
                    </div>
                </div>
                <div
                    class="lg:border-l border-dashed border-gray-400 lg:px-12 w-full lg:max-w-lg text-center sm:text-left">
                    <div class="py-6 text-xl lg:text-2xl leading-7 lg:leading-8">
                        <img :src="avatar" width="150" height="150" class="block w-32 h-32 rounded-full mx-auto sm:mx-0"
                            alt="Customer Testimonial" loading="lazy">
                        <p class="flex-grow py-6 text-gray-600 dark:text-gray-300">
                            “Adopting this {{ config.application }} was straightforward and effective. Its intuitive
                            design and powerful
                            features have revolutionized our enterprise management.”
                        </p>
                        <div class="text-xl text-gray-700 dark:text-gray-100 font-medium">Omar Ahmed</div>
                        <div class="text-lg text-gray-500 dark:text-gray-400">HR Manager at Sefam</div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>
