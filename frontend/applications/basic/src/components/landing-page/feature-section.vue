<script setup lang="ts">
import { config } from 'c3-library';
</script>

<template>
    <div class="bg-gray-50 mx-auto py-20 px-6">
        <section class="text-gray-800">
            <div class="text-center mb-16">
                <p class="text-violet-500 uppercase font-bold text-sm mb-4">Features</p>
                <h2 class="text-4xl font-extrabold mb-6">Why is it so great?</h2>
                <p class="text-gray-600 text-lg mb-12">
                    Explore the exceptional features that make our {{ config.application }} a must-have for your
                    business. From
                    seamless integration to real-time analytics, our solution has it all.
                </p>
            </div>
            <div class="grid md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-8">
                <div class="flex items-start space-x-4 mb-12">
                    <div class="flex-shrink-0 bg-violet-100 p-2 rounded-full">
                        <span class="fa-solid fa-circle-check text-violet-600"></span>
                    </div>
                    <div>
                        <p class="text-lg font-semibold mb-1">24/7 Support</p>
                        <p class="text-gray-500">Our support team is always available to help with any queries or
                            issues you might have.</p>
                    </div>
                </div>
                <div class="flex items-start space-x-4 mb-12">
                    <div class="flex-shrink-0 bg-violet-100 p-2 rounded-full">
                        <span class="fa-solid fa-circle-check text-violet-600"></span>
                    </div>
                    <div>
                        <p class="text-lg font-semibold mb-1">Employee Tracking</p>
                        <p class="text-gray-500">Keep track of employee performance and progress effortlessly.</p>
                    </div>
                </div>
                <div class="flex items-start space-x-4 mb-12">
                    <div class="flex-shrink-0 bg-violet-100 p-2 rounded-full">
                        <span class="fa-solid fa-circle-check text-violet-600"></span>
                    </div>
                    <div>
                        <p class="text-lg font-semibold mb-1">Comprehensive Reporting</p>
                        <p class="text-gray-500">Generate detailed reports to gain deeper insights into your
                            workforce.</p>
                    </div>
                </div>
                <div class="flex items-start space-x-4 mb-12">
                    <div class="flex-shrink-0 bg-violet-100 p-2 rounded-full">
                        <span class="fa-solid fa-circle-check text-violet-600"></span>
                    </div>
                    <div>
                        <p class="text-lg font-semibold mb-1">Advanced Analytics</p>
                        <p class="text-gray-500">Leverage data-driven insights to make well-informed HR decisions.
                        </p>
                    </div>
                </div>
                <div class="flex items-start space-x-4 mb-12">
                    <div class="flex-shrink-0 bg-violet-100 p-2 rounded-full">
                        <span class="fa-solid fa-circle-check text-violet-600"></span>
                    </div>
                    <div>
                        <p class="text-lg font-semibold mb-1">Community Support</p>
                        <p class="text-gray-500">Join a vast community of users and get help when needed.</p>
                    </div>
                </div>
                <div class="flex items-start space-x-4 mb-12">
                    <div class="flex-shrink-0 bg-violet-100 p-2 rounded-full">
                        <span class="fa-solid fa-circle-check text-violet-600"></span>
                    </div>
                    <div>
                        <p class="text-lg font-semibold mb-1">Seamless Integration</p>
                        <p class="text-gray-500">Easily integrate with existing systems and tools.</p>
                    </div>
                </div>
                <div class="flex items-start space-x-4 mb-12">
                    <div class="flex-shrink-0 bg-violet-100 p-2 rounded-full">
                        <span class="fa-solid fa-circle-check text-violet-600"></span>
                    </div>
                    <div>
                        <p class="text-lg font-semibold mb-1">Customizable Dashboards</p>
                        <p class="text-gray-500">Tailor dashboards to suit your specific needs and preferences.</p>
                    </div>
                </div>
                <div class="flex items-start space-x-4 mb-12">
                    <div class="flex-shrink-0 bg-violet-100 p-2 rounded-full">
                        <span class="fa-solid fa-circle-check text-violet-600"></span>
                    </div>
                    <div>
                        <p class="text-lg font-semibold mb-1">Intuitive User Interface</p>
                        <p class="text-gray-500">Enjoy a user-friendly interface designed for ease of use and
                            efficiency.</p>
                    </div>
                </div>
            </div>
        </section>
    </div>
</template>
