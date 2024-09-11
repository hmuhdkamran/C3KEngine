<script setup lang="ts">
import logo from "@/assets/images/vue.svg"
</script>

<template>
    <footer class="relative bg-gray-200 pt-8 pb-6">
        <div class="bottom-auto top-0 left-0 right-0 w-full absolute pointer-events-none overflow-hidden -mt-20"
            style="height:80px"><svg class="absolute bottom-0 overflow-hidden" xmlns="http://www.w3.org/2000/svg"
                preserveAspectRatio="none" version="1.1" viewBox="0 0 2560 100" x="0" y="0">
                <polygon class="text-gray-200 fill-current" points="2560 0 2560 100 0 100"></polygon>
            </svg></div>
        <div class="container mx-auto px-4">
            <div class="grid grid-cols-1 gap-8 lg:grid-cols-3">
                <div>
                    <div class="flex justify-center sm:justify-start">
                        <h4 class="text-3xl font-semibold">
                            <img :src="logo" alt="C3K Engine Logo" class="h-12 w-auto object-contain inline-block" />
                            C3K Engine
                        </h4>
                    </div>
                    <p
                        class="mx-auto mt-6 max-w-md text-center leading-relaxed text-gray-500 sm:mx-0 sm:max-w-xs sm:text-left">
                        Find us on any of these platforms, we respond 1-2
                        business days.
                    </p>
                    <ul class="mt-8 flex justify-center gap-4 sm:justify-start">
                        <li>
                            <a href="https://www.facebook.com/hmuhdkamran" target="_blank"
                                class="social-link">
                                <span class="social-icon icon-[ic--outline-facebook]"></span>
                            </a>
                        </li>

                        <li>
                            <a href="/" rel="noreferrer" target="_blank"
                                class="social-link">
                                <span class="icon-[mdi--instagram]"></span>
                            </a>
                        </li>

                        <li>
                            <a href="https://www.twitter.com/hmuhdkamran" target="_blank"
                                class="social-link">
                                <span class="social-icon icon-[grommet-icons--twitter]"></span>
                            </a>
                        </li>

                        <li>
                            <a href="https://www.github.com/hmuhdkamran" target="_blank"
                                class="social-link">
                                <span class="social-icon icon-[mdi--github]"></span>
                            </a>
                        </li>

                        <li>
                            <a href="https://www.dribbble.com/hmuhdkamran" target="_blank"
                                class="social-link">
                                <span class="social-icon icon-[icon-park-outline--dribble]"></span>
                            </a>
                        </li>
                    </ul>
                    <p class="text-sm mt-6 text-gray-500 font-semibold">Currently v1.1.0. Code<a
                            href="https://github.com/hmuhdkamran" class="text-gray-600" target="_blank">
                            licensed MIT</a>, docs<a href="https://creativecommons.org/licenses/by/4.0/" targe="_blank"
                            class="text-gray-600"> CC BY 4.0</a>.</p>
                </div>

                <div class="grid grid-cols-1 gap-8 sm:grid-cols-2 md:grid-cols-4 lg:col-span-2">
                    <div class="text-center sm:text-left">
                        <p class="text-lg font-medium text-gray-900">About Us</p>

                        <nav aria-label="Footer About Nav" class="mt-8">
                            <ul class="space-y-4 text-sm">
                                <li>
                                    <a class="text-gray-700 transition hover:text-gray-700/75" href="/">
                                        Company History
                                    </a>
                                </li>

                                <li>
                                    <a class="text-gray-700 transition hover:text-gray-700/75" href="/">
                                        Meet the Team
                                    </a>
                                </li>

                                <li>
                                    <a class="text-gray-700 transition hover:text-gray-700/75" href="/">
                                        Employee Handbook
                                    </a>
                                </li>

                                <li>
                                    <a class="text-gray-700 transition hover:text-gray-700/75" href="/">
                                        Careers
                                    </a>
                                </li>
                            </ul>
                        </nav>
                    </div>

                    <div class="text-center sm:text-left">
                        <p class="text-lg font-medium text-gray-900">Our Services</p>

                        <nav aria-label="Footer Services Nav" class="mt-8">
                            <ul class="space-y-4 text-sm">
                                <li>
                                    <a class="text-gray-700 transition hover:text-gray-700/75" href="/">
                                        Web Development
                                    </a>
                                </li>

                                <li>
                                    <a class="text-gray-700 transition hover:text-gray-700/75" href="/">
                                        Web Design
                                    </a>
                                </li>

                                <li>
                                    <a class="text-gray-700 transition hover:text-gray-700/75" href="/">
                                        Software Development
                                    </a>
                                </li>

                                <li>
                                    <a class="text-gray-700 transition hover:text-gray-700/75" href="/">
                                        Google Ads
                                    </a>
                                </li>
                            </ul>
                        </nav>
                    </div>

                    <div class="text-center sm:text-left">
                        <p class="text-lg font-medium text-gray-900">Helpful Links</p>

                        <nav aria-label="Footer Helpful Nav" class="mt-8">
                            <ul class="space-y-4 text-sm">
                                <li>
                                    <a class="text-gray-700 transition hover:text-gray-700/75" href="/">
                                        FAQs
                                    </a>
                                </li>

                                <li>
                                    <a class="text-gray-700 transition hover:text-gray-700/75" href="/">
                                        Support
                                    </a>
                                </li>

                                <li>
                                    <a class="group flex justify-center gap-1.5 sm:justify-start" href="/">
                                        <span class="text-gray-700 transition group-hover:text-gray-700/75">
                                            Live Chat
                                        </span>

                                        <span class="relative -mr-2 flex h-2 w-2">
                                            <span
                                                class="absolute inline-flex h-full w-full animate-ping rounded-full bg-teal-400 opacity-75"></span>
                                            <span class="relative inline-flex h-2 w-2 rounded-full bg-teal-500"></span>
                                        </span>
                                    </a>
                                </li>
                            </ul>
                        </nav>
                    </div>

                    <div class="text-center sm:text-left">
                        <p class="text-lg font-medium text-gray-900">Contact Us</p>

                        <ul class="mt-8 space-y-4 text-sm">
                            <li>
                                <a class="flex items-center justify-center gap-1.5 sm:justify-start" href="/">
                                    <span class="icon-[wpf--message-outline]"></span>
                                    <span class="text-gray-700">admin@c3k.org</span>
                                </a>
                            </li>

                            <li>
                                <a class="flex items-center justify-center gap-1.5 sm:justify-start" href="/">
                                    <span class="icon-[fluent--call-28-regular]"></span>
                                    <span class="text-gray-700">+012 345 6789</span>
                                </a>
                            </li>

                            <li class="flex items-start justify-center gap-1.5 sm:justify-start">
                                <span class="icon-[ep--location]"></span>
                                <address class="-mt-0.5 not-italic text-gray-700">
                                    8 Waris Road, Lahore, Pakistan
                                </address>
                            </li>
                        </ul>
                    </div>
                </div>
            </div>

            <div class="mt-12 border-t border-gray-100 pt-6">
                <div class="text-center sm:flex sm:justify-between sm:text-left">
                    <p class="text-sm text-gray-500">
                        <span class="block sm:inline">All rights reserved.</span>

                        <a class="inline-block text-violet-600 underline transition hover:text-violet-600/75" href="/">
                            Terms &amp; Conditions
                        </a>

                        <span>·</span>

                        <a class="inline-block text-violet-600 underline transition hover:text-violet-600/75" href="/">
                            Privacy Policy
                        </a>
                    </p>
                    <p class="mt-4 text-sm text-gray-500 sm:order-first sm:mt-0">
                        Copyright © 2024 C3K
                        Engine by <a href="https://combinesoft.net" class="text-gray-500 hover:text-gray-800"
                            target="_blank">COMBINE CHIPSoft</a>.
                    </p>
                </div>
            </div>
        </div>
    </footer>
</template>