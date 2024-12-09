<script setup lang="ts">
import { Card, config } from "c3k-library";
</script>

<template>
    <section>
        <div class="mt-16 bg-gray-50 radius-for-skewed">
            <div class="container mx-auto px-4">
                <div class="flex flex-wrap items-center">
                    <!-- Text Section -->
                    <div class="w-full lg:w-1/2 mb-12 lg:mb-0">
                        <div class="max-w-md lg:mx-auto">
                            <span class="text-violet-600 font-bold">{{ config.application }}</span>
                            <h2 class="my-2 text-4xl lg:text-5xl font-bold font-heading">Streamline Your Business
                                Operations</h2>
                            <p class="mb-6 text-violet-500 leading-loose">
                                Our ERP solution simplifies complex business processes, enhances operational
                                efficiency, and provides comprehensive data management across all departments.
                            </p>
                            <ul class="text-gray-500 font-bold">
                                <li class="flex mb-4">
                                    <span class="icon-[ep--success-filled] mr-2 w-6 h-6 text-violet-600"></span>
                                    <span>Unified Business Management</span>
                                </li>
                                <li class="flex mb-4">
                                    <span class="icon-[ep--success-filled] mr-2 w-6 h-6 text-violet-600"></span>
                                    <span>Automated Financial Processes</span>
                                </li>
                                <li class="flex mb-4">
                                    <span class="icon-[ep--success-filled] mr-2 w-6 h-6 text-violet-600"></span>
                                    <span>Detailed Analytics & Reporting</span>
                                </li>
                            </ul>
                        </div>
                    </div>

                    <!-- Features Section -->
                    <div class="w-full lg:w-1/2 flex flex-wrap -mx-4 py-3">
                        <div class="mb-8 lg:mb-0 w-full md:w-1/2 px-4">
                            <!-- Feature Card 1 -->
                            <div class="mb-8 py-6 pl-6 pr-4 shadow rounded bg-white hover:translate-y-[-2px] transition-transform duration-300">
                                <span class="mb-4 inline-block p-3 rounded-lg bg-violet-100">
                                    <span class="icon-[tabler--photo] w-10 h-10 text-violet-600"></span>
                                </span>
                                <h4 class="mb-2 text-2xl font-bold font-heading">Inventory Management</h4>
                                <p class="text-gray-500 leading-loose">
                                    Monitor stock levels, track orders, and manage your supply chain efficiently
                                    with our advanced inventory tools.
                                </p>
                            </div>

                            <!-- Feature Card 2 -->
                            <div class="py-6 pl-6 pr-4 shadow rounded bg-white hover:translate-y-[-2px] transition-transform duration-300">
                                <span class="mb-4 inline-block p-3 rounded-lg bg-violet-100">
                                    <span class="icon-[fluent--toolbox-32-filled] w-10 h-10 text-violet-600"></span>
                                </span>
                                <h4 class="mb-2 text-2xl font-bold font-heading">Financial Management</h4>
                                <p class="text-gray-500 leading-loose">
                                    Streamline accounting, billing, and financial reporting with automated tools to
                                    improve accuracy and compliance.
                                </p>
                            </div>
                        </div>

                        <div class="w-full md:w-1/2 lg:mt-20 px-4">
                            <!-- Feature Card 3 -->
                            <div class="mb-8 py-6 pl-6 pr-4 shadow rounded-lg bg-white hover:translate-y-[-2px] transition-transform duration-300">
                                <span class="mb-4 inline-block p-3 rounded bg-violet-100">
                                    <span class="icon-[streamline--star-badge-solid] w-10 h-10 text-violet-600"></span>
                                </span>
                                <h4 class="mb-2 text-2xl font-bold font-heading">CRM Integration</h4>
                                <p class="text-gray-500 leading-loose">
                                    Manage customer relationships with built-in CRM tools, providing a holistic view
                                    of customer data and interactions.
                                </p>
                            </div>

                            <!-- Feature Card 4 -->
                            <div class="py-6 pl-6 pr-4 shadow rounded-lg bg-white hover:translate-y-[-2px] transition-transform duration-300">
                                <span class="mb-4 inline-block p-3 rounded bg-violet-100">
                                    <span class="icon-[mdi--tick-decagram] w-10 h-10 text-violet-600"></span>
                                </span>
                                <h4 class="mb-2 text-2xl font-bold font-heading">Compliance and Risk Management</h4>
                                <p class="text-gray-500 leading-loose">
                                    Ensure your business meets industry standards and regulations with automated
                                    compliance monitoring and risk assessments.
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </section>
</template>