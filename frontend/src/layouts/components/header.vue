<script setup lang="ts">
import logo from "@/assets/vue.svg"
</script>

<template>
    <nav class="fixed z-50 w-full bg-white top-0 flex flex-wrap items-center justify-between px-2 py-3 shadow-lg">
        <div class="container px-4 mx-auto flex flex-wrap items-center justify-between">
            <div class="w-full relative flex justify-between lg:w-auto lg:static lg:block lg:justify-start">
                <a class="text-sm font-bold leading-relaxed inline-block mr-4 py-2 whitespace-nowrap uppercase text-gray-700"
                    href="/learning-lab/tailwind-starter-kit/presentation">
                    <img :src="logo" class="inline-block" />
                    C3K Engine</a>
                <button
                    class="cursor-pointer text-xl leading-none px-3 py-1 border border-solid border-transparent rounded bg-transparent block lg:hidden outline-none focus:outline-none"
                    type="button">
                    <span class="icon-[fa-solid--bars]"></span>
                </button>
            </div>
            <div class="lg:flex flex-grow items-center hidden" id="example-navbar-danger">
                <ul class="flex flex-col lg:flex-row list-none lg:ml-auto">
                    <li class="nav-item">
                        <a href="https://www.facebook.com/hmuhdkamran"
                            class="px-3 py-2 flex items-center text-xs uppercase font-bold text-gray-700 hover:text-gray-500"
                            target="_blank">
                            <span class="icon-[fa--facebook-square] text-lg leading-lg text-gray-400"></span>
                        </a>
                    </li>
                    <li class="nav-item">
                        <a href="https://twitter.com/hmuhdkamran"
                            class="px-3 py-2 flex items-center text-xs uppercase font-bold text-gray-700 hover:text-gray-500"
                            target="_blank">
                            <span class="icon-[fa--twitter] text-lg leading-lg text-gray-400"></span>
                        </a>
                    </li>
                    <li class="nav-item">
                        <a href="https://github.com/hmuhdkamran"
                            class="px-3 py-2 flex items-center text-xs uppercase font-bold text-gray-700 hover:text-gray-500"
                            target="_blank">
                            <span class="icon-[fa--github] text-lg leading-lg text-gray-400"></span>
                        </a>
                    </li>
                    <li class="nav-item cursor-pointer ">
                        <RouterLink to="/auth/login" class="px-3 py-2 flex items-center text-xs uppercase font-bold text-gray-700 hover:text-gray-500">
                            <span class="icon-[fa--user] text-lg leading-lg text-gray-400 mr-2"></span>
                                Login
                        </RouterLink>
                    </li>
                </ul>
            </div>
        </div>
    </nav>
</template>