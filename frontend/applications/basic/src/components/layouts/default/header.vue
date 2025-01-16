<script setup lang="ts">
import { config, VNodeRenderer } from 'c3-library';
</script>

<template>
    <nav class="fixed z-50 w-full bg-white top-0 flex flex-wrap items-center justify-between px-1 py-1 shadow-lg">
        <div class="container px-4 mx-auto flex flex-wrap items-center justify-between">
            <div class="w-full relative flex justify-between lg:w-auto lg:static lg:block lg:justify-start">
                <a class="text-sm font-bold leading-relaxed inline-block mr-4 py-1 whitespace-nowrap uppercase text-gray-700"
                    href="/">
                    <VNodeRenderer :nodes="config.logo" />{{ config.application }}</a>
                <button
                    class="cursor-pointer text-xl leading-none px-3 py-1 border border-solid border-transparent rounded bg-transparent block lg:hidden outline-none focus:outline-none"
                    type="button">
                    <span class="icon-[fa-solid--bars]"></span>
                </button>
            </div>
            <div class="lg:flex flex-grow items-center hidden">
                <ul class="flex flex-col lg:flex-row list-none lg:ml-auto gap-4">
                    <li class="nav-item" v-for="item in config.socialMedia">
                        <a :href="item.link" target="_blank">
                            <span :class="item.icon"></span>
                        </a>
                    </li>
                    <li class="nav-item cursor-pointer ">
                        <RouterLink to="/auth" class="btn-outline btn-primary px-4 py-1">
                            Login
                        </RouterLink>
                    </li>
                </ul>
            </div>
        </div>
    </nav>
</template>
