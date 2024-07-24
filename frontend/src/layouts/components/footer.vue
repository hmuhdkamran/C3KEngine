<script setup lang="ts">
import logo from "@/assets/vue.svg"
</script>

<template>
    <footer class="relative bg-gray-200 pt-8 pb-6">
        <div class="bottom-auto top-0 left-0 right-0 w-full absolute pointer-events-none overflow-hidden -mt-20"
            style="height:80px"><svg class="absolute bottom-0 overflow-hidden" xmlns="http://www.w3.org/2000/svg"
                preserveAspectRatio="none" version="1.1" viewBox="0 0 2560 100" x="0" y="0">
                <polygon class="text-gray-200 fill-current" points="2560 0 2560 100 0 100"></polygon>
            </svg></div>
        <div class="container mx-auto px-4">
            <div class="flex flex-wrap">
                <div class="w-full md:w-6/12 px-4">
                    <h4 class="text-3xl font-semibold">
                        <img :src="logo" alt="C3K Engine Logo" class="h-12 w-auto object-contain inline-block" />
                        C3K Engine
                    </h4>
                    <h5 class="text-lg mt-0 mb-2 text-gray-600">Find us on any of these platforms, we respond 1-2
                        business days.</h5>
                    <div class="mt-6">
                        <a href="https://www.twitter.com/hmuhdkamran" target="_blank">
                            <i
                                class="icon-[fa--twitter] bg-white text-blue-400 shadow-lg font-lg p-3 items-center justify-center align-center rounded-full outline-none focus:outline-none mr-2 inline-block text-center"></i>
                        </a>
                        <a href="https://www.facebook.com/hmuhdkamran" target="_blank">
                            <i
                                class="icon-[fa--facebook-square] bg-white text-blue-600 shadow-lg font-lg p-3 items-center justify-center align-center rounded-full outline-none focus:outline-none mr-2 inline-block text-center"></i>
                        </a>
                        <a href="https://www.dribbble.com/hmuhdkamran" target="_blank">
                            <i
                                class="icon-[fa--dribbble] bg-white text-pink-400 shadow-lg font-lg p-3 items-center justify-center align-center rounded-full outline-none focus:outline-none mr-2 inline-block text-center"></i>
                        </a>
                        <a href="https://www.github.com/hmuhdkamran" target="_blank">
                            <i
                                class="icon-[fa--github] bg-white text-gray-800 shadow-lg font-lg p-3 items-center justify-center align-center rounded-full outline-none focus:outline-none mr-2 inline-block text-center"></i>
                        </a>
                    </div>
                    <p class="text-sm mt-6 text-gray-500 font-semibold">Currently v1.1.0. Code<a
                            href="https://github.com/hmuhdkamran" class="text-gray-600" target="_blank">
                            <!-- -->licensed MIT</a>, docs<a href="https://creativecommons.org/licenses/by/4.0/"
                            targe="_blank" class="text-gray-600"> <!-- -->CC BY 4.0</a>.</p>
                </div>
                <div class="w-full md:w-6/12 px-4">
                    <div class="flex flex-wrap items-top mb-6">
                        <div class="w-full md:w-6/12 xl:w-4/12 pt-6 md:pt-0 md:px-4 ml-auto"><span
                                class="block uppercase text-gray-500 text-sm font-semibold mb-2">Useful Links</span>
                            <ul class="list-unstyled">
                                <li>
                                    <a class="text-gray-600 hover:text-gray-800 font-semibold block pb-2 text-sm"
                                        href="https://combinesoft.net" target="_blank">About Us</a>
                                </li>
                                <li>
                                    <a class="text-gray-600 hover:text-gray-800 font-semibold block pb-2 text-sm"
                                        href="https://combinesoft.net" target="_blank">Blog</a>
                                </li>
                                <li>
                                    <a class="text-gray-600 hover:text-gray-800 font-semibold block pb-2 text-sm"
                                        href="https://www.github.com/hmuhdkamran" target="_blank">Github</a>
                                </li>
                            </ul>
                        </div>
                        <div class="w-full md:w-6/12 xl:w-4/12 pt-6 md:pt-0 md:px-4 ml-auto"><span
                                class="block uppercase text-gray-500 text-sm font-semibold mb-2">Other
                                Resources</span>
                            <ul class="list-unstyled">
                                <li>
                                    <a class="text-gray-600 hover:text-gray-800 font-semibold block pb-2 text-sm"
                                        href="/" target="_blank">MIT License</a>
                                </li>
                                <li>
                                    <a class="text-gray-600 hover:text-gray-800 font-semibold block pb-2 text-sm"
                                        href="/" target="_blank">Terms &amp; Conditions</a>
                                </li>
                                <li>
                                    <a class="text-gray-600 hover:text-gray-800 font-semibold block pb-2 text-sm"
                                        href="/" target="_blank">Privacy Policy</a>
                                </li>
                                <li>
                                    <a class="text-gray-600 hover:text-gray-800 font-semibold block pb-2 text-sm"
                                        href="/" target="_blank">Contact Us</a>
                                </li>
                            </ul>
                        </div>
                    </div>
                </div>
            </div>
            <hr class="my-6 border-gray-300">
            <div class="flex flex-wrap items-center md:justify-between justify-center">
                <div class="w-full md:w-4/12 px-4 mx-auto text-center">
                    <div class="text-sm text-gray-500 font-semibold py-1">Copyright © <!-- -->2024<!-- --> C3K
                        Engine by<!-- --> <a href="https://combinesoft.net" class="text-gray-500 hover:text-gray-800"
                            target="_blank">COMBINE CHIPSoft</a>.
                    </div>
                </div>
            </div>
        </div>
    </footer>
</template>