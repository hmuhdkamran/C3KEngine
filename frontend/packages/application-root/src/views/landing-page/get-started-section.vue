<script setup lang="ts">
import { Card } from "c3k-library";
</script>

<template>
    <section class="py-2 bg-gray-200 relative sm:mt-36">
        <!-- Decorative SVG Section -->
        <div class="-mt-20 top-0 bottom-auto left-0 right-0 w-full absolute" style="height:80px">
            <svg class="absolute bottom-0 overflow-hidden" xmlns="http://www.w3.org/2000/svg" preserveAspectRatio="none"
                version="1.1" viewBox="0 0 2560 100" x="0" y="0">
                <polygon class="text-gray-200 fill-current" points="2560 0 2560 100 0 100"></polygon>
            </svg>
        </div>

        <!-- Main Content Section -->
        <Card :showHeader="false" :showFooter="false" containerClass="container mx-auto">
            <div class="flex flex-wrap justify-center bg-white shadow-xl rounded-lg -mt-64 py-16 px-12 relative z-10">
                <div class="w-full text-center lg:w-8/12">
                    <!-- Decorative Icon or Emoji -->
                    <p class="text-4xl text-center">
                        <span role="img" aria-label="rocket">🚀</span>
                    </p>

                    <!-- Title and Subtitle -->
                    <h3 class="font-semibold text-3xl">
                        Ready to Transform Your Business Operations?
                    </h3>
                    <p class="text-gray-500 text-lg leading-relaxed mt-4 mb-4">
                        Whether you're in retail, education, healthcare, or any other industry, our ERP solution is
                        designed to optimize and streamline your operations. From inventory to payroll, we provide
                        all the modules you need to run your business efficiently.
                        Click the button below to get started with our advanced ERP system—built to scale with your
                        business.
                    </p>

                    <!-- Call to Action Buttons -->
                    <div class="sm:block flex flex-col mt-10">
                        <a class="get-started text-white font-bold px-6 py-4 rounded outline-none focus:outline-none mr-1 mb-2 bg-violet-600 active:bg-violet-700 uppercase text-sm shadow hover:shadow-lg ease-linear transition-all duration-150"
                            href="/get-started">
                            Get Started
                        </a>
                        <a href="https://github.com/your-repo" target="_blank"
                            class="github-star sm:ml-1 text-white font-bold px-6 py-4 rounded outline-none focus:outline-none mr-1 mb-1 bg-gray-700 active:bg-gray-600 uppercase text-sm shadow hover:shadow-lg">
                            <i class="fab fa-github text-lg mr-1"></i><span>Help With a Star</span>
                        </a>
                    </div>

                    <!-- Optional Additional Content Section -->
                    <div class="text-center mt-16"></div>
                </div>
            </div>
        </Card>
    </section>
</template>