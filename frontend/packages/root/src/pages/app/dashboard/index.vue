<script setup lang="ts">
import logo from '@/assets/images/vue.svg'

import { ref, onMounted, onUnmounted } from 'vue';
const isOpen = ref(false);
const isDropdownOpen = ref(false)

const toggleDropdown = () => {
    isDropdownOpen.value = !isDropdownOpen.value
    isOpen.value = !isOpen.value;
}
function toggleDropdown1() {
    isOpen.value = !isOpen.value;
}

function handleClickOutside(event: MouseEvent) {
    const dropdown = document.querySelector('.relative');
    if (dropdown && !dropdown.contains(event.target as Node)) {
        isOpen.value = false;
    }
}

onMounted(() => {
    document.addEventListener('click', handleClickOutside);
});

onUnmounted(() => {
    document.removeEventListener('click', handleClickOutside);
});

</script>

<template>
    <div class="flex h-screen">
        <div class="absolute w-full bg-violet-500 dark:hidden min-h-75">

            <div class="bg-violet-500 p-4 flex justify-between items-center">
                <div class="flex items-center space-x-4"></div>

                <div class="flex items-center mt-2 grow sm:mt-0 sm:mr-6 md:mr-0 lg:flex lg:basis-auto" id="navbar">
                    <div class="flex items-center md:ml-auto md:pr-4">
                        <div class="relative flex flex-wrap items-stretch w-full transition-all rounded-lg ease">
                            <span
                                class="text-sm ease leading-5.6 absolute z-50 -ml-px flex h-full items-center whitespace-nowrap rounded-lg rounded-tr-none rounded-br-none border border-r-0 border-transparent bg-transparent py-2 px-2.5 text-center font-normal text-slate-500 transition-all">

                                <span class="icon-[ic--baseline-search] h-5 w-5" aria-hidden="true"></span>
                            </span>
                            <input type="text"
                                class="pl-9 text-sm focus:shadow-primary-outline dark:bg-slate-850 dark:placeholder:text-white/80 dark:text-white/80 ease w-1/100 leading-5.6 relative -ml-px block min-w-0 flex-auto rounded-lg border border-solid border-gray-300 bg-white bg-clip-padding py-2 pr-3 text-gray-700 transition-all placeholder:text-gray-500 focus:border-blue-500 focus:outline-none focus:transition-shadow"
                                placeholder="Type here...">
                        </div>
                    </div>
                    <div class="relative"><button class="btn btn-ghost btn-circle">
                            <div class="flex items-center p-2 text-white hover:text-white focus:outline-none">
                                <span class="icon-[gridicons--user] h-7 w-7" aria-hidden="true"></span>
                            </div>
                        </button></div>

                    <div class="relative"><button class="btn btn-ghost btn-circle">
                        </button></div>
                    <div class="relative"><button class="btn btn-ghost btn-circle">
                            <div class="flex items-center p-2 text-white hover:text-white focus:outline-none">
                                <span class="icon-[mdi--bell-outline] h-7 w-7"></span><span
                                    class="absolute top-0 right-0 block w-4 h-4 text-xs text-white bg-red-600 rounded-full flex items-center justify-center">4</span>
                            </div>
                        </button></div>
                    <div class="relative"><button
                            class="flex items-center p-2 text-gray-600 hover:text-gray-800 focus:outline-none"><img
                                src="/src/assets/images/avatar.jpg" alt="Profile"
                                class="w-10 h-10 rounded-full"></button></div>
                </div>
            </div>

            <div class="w-full px-6 py-6 mx-auto">
                <div class="flex justify-end -mx-16 mr-2">

                    <div class="px-3 mb-6 md:mb-0 md:w-1/5">
                        <div
                            class="relative flex flex-col min-w-0 break-words bg-white shadow-xl dark:bg-slate-850 dark:shadow-dark-xl rounded-2xl bg-clip-border">
                            <div class="flex-auto p-4">
                                <div class="flex flex-row -mx-3">
                                    <div class="flex-none w-2/3 max-w-full px-3">
                                        <div>
                                            <p
                                                class="mb-0 font-sans text-sm font-semibold leading-normal uppercase dark:text-white dark:opacity-60">
                                                Today's Money</p>
                                            <h5 class="mb-2 font-bold dark:text-white">$53,000</h5>
                                            <p class="mb-0 dark:text-white dark:opacity-60">
                                                <span
                                                    class="text-sm font-bold leading-normal text-emerald-500">+55%</span>
                                                since yesterday
                                            </p>
                                        </div>
                                    </div>
                                    <div class="px-3 text-right basis-1/3">
                                        <div
                                            class="inline-block w-12 h-12 text-center rounded-circle bg-gradient-to-tl from-blue-500 to-violet-500">
                                            <span class="icon-[wpf--coins] text-lg relative top-3.5 text-white"></span>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="px-3 mb-6 md:mb-0 md:w-1/5">
                        <div
                            class="relative flex flex-col min-w-0 break-words bg-white shadow-xl dark:bg-slate-850 dark:shadow-dark-xl rounded-2xl bg-clip-border">
                            <div class="flex-auto p-4">
                                <div class="flex flex-row -mx-3">
                                    <div class="flex-none w-2/3 max-w-full px-3">
                                        <div>
                                            <p
                                                class="mb-0 font-sans text-sm font-semibold leading-normal uppercase dark:text-white dark:opacity-60">
                                                Today's Users</p>
                                            <h5 class="mb-2 font-bold dark:text-white">2,300</h5>
                                            <p class="mb-0 dark:text-white dark:opacity-60">
                                                <span
                                                    class="text-sm font-bold leading-normal text-emerald-500">+3%</span>
                                                since last week
                                            </p>
                                        </div>
                                    </div>
                                    <div class="px-3 text-right basis-1/3">
                                        <div
                                            class="inline-block w-12 h-12 text-center rounded-circle bg-gradient-to-tl from-red-600 to-orange-600">
                                            <span
                                                class="icon-[mage--users-fill] text-lg relative top-3.5 text-white"></span>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="px-3 mb-6 md:mb-0 md:w-1/5">
                        <div
                            class="relative flex flex-col min-w-0 break-words bg-white shadow-xl dark:bg-slate-850 dark:shadow-dark-xl rounded-2xl bg-clip-border">
                            <div class="flex-auto p-4">
                                <div class="flex flex-row -mx-3">
                                    <div class="flex-none w-2/3 max-w-full px-3">
                                        <div>
                                            <p
                                                class="mb-0 font-sans text-sm font-semibold leading-normal uppercase dark:text-white dark:opacity-60">
                                                New Clients</p>
                                            <h5 class="mb-2 font-bold dark:text-white">+3,462</h5>
                                            <p class="mb-0 dark:text-white dark:opacity-60">
                                                <span class="text-sm font-bold leading-normal text-red-600">-2%</span>
                                                since last quarter
                                            </p>
                                        </div>
                                    </div>
                                    <div class="px-3 text-right basis-1/3">
                                        <div
                                            class="inline-block w-12 h-12 text-center rounded-circle bg-gradient-to-tl from-emerald-500 to-teal-400">
                                            <span
                                                class="icon-[bxs--user-pin] text-lg relative top-3.5 text-white"></span>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="px-3 mb-6 md:mb-0 md:w-1/5">
                        <div
                            class="relative flex flex-col min-w-0 break-words bg-white shadow-xl dark:bg-slate-850 dark:shadow-dark-xl rounded-2xl bg-clip-border">
                            <div class="flex-auto p-4">
                                <div class="flex flex-row -mx-3">
                                    <div class="flex-none w-2/3 max-w-full px-3">
                                        <div>
                                            <p
                                                class="mb-0 font-sans text-sm font-semibold leading-normal uppercase dark:text-white dark:opacity-60">
                                                Sales</p>
                                            <h5 class="mb-2 font-bold dark:text-white">$103,430</h5>
                                            <p class="mb-0 dark:text-white dark:opacity-60">
                                                <span
                                                    class="text-sm font-bold leading-normal text-emerald-500">+5%</span>
                                                than last month
                                            </p>
                                        </div>
                                    </div>
                                    <div class="px-3 text-right basis-1/3">
                                        <div
                                            class="inline-block w-12 h-12 text-center rounded-circle bg-gradient-to-tl from-orange-500 to-yellow-500">
                                            <span
                                                class="icon-[icon-park-solid--sales-report] text-lg relative top-3.5 text-white"></span>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>

                </div>
            </div>

        </div>

        <aside
            class="fixed inset-y-0 left-0 flex-wrap items-center justify-between block w-full p-0 my-4 transition-transform duration-200 ease-in-out -translate-x-full bg-white border-0 shadow-none xl:ml-6 dark:bg-slate-850 z-50 rounded-2xl xl:translate-x-0 overflow-y-auto max-w-64"
            id="sidenav-main">
            <div class="relative h-20">
                <a href="/" class=" px-8 py-6 text-xl font-semibold text-gray-800 flex items-center space-x-2">
                    <img :src="logo" alt="C3K Engine Logo" class="h-12 w-auto object-contain" />
                    <span class="ml-1 text-xl font-semibold transition-all duration-200 ease-in-out">C3K Engine</span>
                </a>
            </div>

            <hr class="my-0 bg-transparent bg-gradient-to-r from-transparent via-black/40 to-transparent 
        dark:bg-gradient-to-r dark:from-transparent dark:via-white dark:to-transparent">

            <div class="px-3 py-4 overflow-y-auto rounded ">
                <ul class="space-y-2">
                    <li>
                        <a href="#"
                            class="flex items-center p-2 text-base font-normal text-gray-900 rounded-lg dark:text-white hover:bg-violet-100 dark:hover:bg-violet-700">
                            <svg class="w-6 h-6 text-violet-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white"
                                fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                                <path d="M2 10a8 8 0 018-8v8h8a8 8 0 11-16 0z"></path>
                                <path d="M12 2.252A8.014 8.014 0 0117.748 8H12V2.252z"></path>
                            </svg>
                            <span class="ml-3">Dashboard</span>
                        </a>
                    </li>
                    <li>
                        <button type="button" @click="toggleDropdown"
                            class="flex items-center w-full p-2 text-base font-normal text-gray-900 transition duration-75 rounded-lg group hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700"
                            aria-controls="dropdown-example">
                            <svg class="flex-shrink-0 w-6 h-6 text-violet-500 transition duration-75 group-hover:text-gray-900 dark:text-gray-400 dark:group-hover:text-white"
                                fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                                <path fill-rule="evenodd"
                                    d="M10 2a4 4 0 00-4 4v1H5a1 1 0 00-.994.89l-1 9A1 1 0 004 18h12a1 1 0 00.994-1.11l-1-9A1 1 0 0015 7h-1V6a4 4 0 00-4-4zm2 5V6a2 2 0 10-4 0v1h4zm-6 3a1 1 0 112 0 1 1 0 01-2 0zm7-1a1 1 0 100 2 1 1 0 000-2z"
                                    clip-rule="evenodd"></path>
                            </svg>
                            <span class="flex-1 ml-3 text-left whitespace-nowrap" sidebar-toggle-item>E-commerce</span>
                            <svg sidebar-toggle-item class="w-6 h-6" fill="currentColor" viewBox="0 0 20 20"
                                xmlns="http://www.w3.org/2000/svg">
                                <path fill-rule="evenodd"
                                    d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z"
                                    clip-rule="evenodd"></path>
                            </svg>
                        </button>
                        <ul :class="{ hidden: !isDropdownOpen, block: isDropdownOpen }" class="py-2 space-y-2">
                            <li>
                                <a href="#"
                                    class="flex items-center w-full p-2 text-base font-normal text-gray-900 transition duration-75 rounded-lg group hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700 pl-11">
                                    Products
                                </a>
                            </li>
                            <li>
                                <a href="#"
                                    class="flex items-center w-full p-2 text-base font-normal text-gray-900 transition duration-75 rounded-lg group hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700 pl-11">
                                    Billing
                                </a>
                            </li>
                            <li>
                                <a href="#"
                                    class="flex items-center w-full p-2 text-base font-normal text-gray-900 transition duration-75 rounded-lg group hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700 pl-11">
                                    Invoice
                                </a>
                            </li>
                        </ul>
                    </li>
                    <li>
                        <a href="#"
                            class="flex items-center p-2 text-base font-normal text-gray-900 rounded-lg dark:text-white hover:bg-gray-100 dark:hover:bg-gray-700">
                            <svg class="flex-shrink-0 w-6 h-6 text-violet-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white"
                                fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                                <path
                                    d="M5 3a2 2 0 00-2 2v2a2 2 0 002 2h2a2 2 0 002-2V5a2 2 0 00-2-2H5zM5 11a2 2 0 00-2 2v2a2 2 0 002 2h2a2 2 0 002-2v-2a2 2 0 00-2-2H5zM11 5a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V5zM11 13a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z">
                                </path>
                            </svg>
                            <span class="flex-1 ml-3 whitespace-nowrap">Categories</span>
                            <span
                                class="inline-flex items-center justify-center px-2 ml-3 text-sm font-medium text-gray-800 bg-gray-200 rounded-full dark:bg-gray-700 dark:text-gray-300">Pro</span>
                        </a>
                    </li>
                    <hr class="my-0 bg-transparent bg-gradient-to-r from-transparent via-black/40 to-transparent 
        dark:bg-gradient-to-r dark:from-transparent dark:via-white dark:to-transparent">
                    <li>
                        <a href="#"
                            class="flex items-center p-2 text-base font-normal text-gray-900 rounded-lg dark:text-white hover:bg-gray-100 dark:hover:bg-gray-700">
                            <svg class="flex-shrink-0 w-6 h-6 text-violet-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white"
                                fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                                <path
                                    d="M8.707 7.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l2-2a1 1 0 00-1.414-1.414L11 7.586V3a1 1 0 10-2 0v4.586l-.293-.293z">
                                </path>
                                <path
                                    d="M3 5a2 2 0 012-2h1a1 1 0 010 2H5v7h2l1 2h4l1-2h2V5h-1a1 1 0 110-2h1a2 2 0 012 2v10a2 2 0 01-2 2H5a2 2 0 01-2-2V5z">
                                </path>
                            </svg>
                            <span class="flex-1 ml-3 whitespace-nowrap">Inbox</span>
                            <span
                                class="inline-flex items-center justify-center w-3 h-3 p-3 ml-3 text-sm font-medium text-violet-600 bg-violet-200 rounded-full dark:bg-blue-900 dark:text-blue-200">3</span>
                        </a>
                    </li>
                    <li>
                        <a href="#"
                            class="flex items-center p-2 text-base font-normal text-gray-900 rounded-lg dark:text-white hover:bg-gray-100 dark:hover:bg-gray-700">
                            <svg class="flex-shrink-0 w-6 h-6 text-violet-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white"
                                fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                                <path fill-rule="evenodd" d="M10 9a3 3 0 100-6 3 3 0 000 6zm-7 9a7 7 0 1114 0H3z"
                                    clip-rule="evenodd"></path>
                            </svg>
                            <span class="flex-1 ml-3 whitespace-nowrap">Users</span>
                        </a>
                    </li>
                    <li>
                        <a href="#"
                            class="flex items-center p-2 text-base font-normal text-gray-900 rounded-lg dark:text-white hover:bg-gray-100 dark:hover:bg-gray-700">
                            <svg class="flex-shrink-0 w-6 h-6 text-violet-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white"
                                fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                                <path fill-rule="evenodd"
                                    d="M10 2a4 4 0 00-4 4v1H5a1 1 0 00-.994.89l-1 9A1 1 0 004 18h12a1 1 0 00.994-1.11l-1-9A1 1 0 0015 7h-1V6a4 4 0 00-4-4zm2 5V6a2 2 0 10-4 0v1h4zm-6 3a1 1 0 112 0 1 1 0 01-2 0zm7-1a1 1 0 100 2 1 1 0 000-2z"
                                    clip-rule="evenodd"></path>
                            </svg>
                            <span class="flex-1 ml-3 whitespace-nowrap">Products</span>
                        </a>
                    </li>
                    <li>
                        <a href="#"
                            class="flex items-center p-2 text-base font-normal text-gray-900 rounded-lg dark:text-white hover:bg-gray-100 dark:hover:bg-gray-700">
                            <svg class="flex-shrink-0 w-6 h-6 text-violet-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white"
                                fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                                <path fill-rule="evenodd"
                                    d="M3 3a1 1 0 00-1 1v12a1 1 0 102 0V4a1 1 0 00-1-1zm10.293 9.293a1 1 0 001.414 1.414l3-3a1 1 0 000-1.414l-3-3a1 1 0 10-1.414 1.414L14.586 9H7a1 1 0 100 2h7.586l-1.293 1.293z"
                                    clip-rule="evenodd"></path>
                            </svg>
                            <span class="flex-1 ml-3 whitespace-nowrap">Sign In</span>
                        </a>
                    </li>
                    <li>
                        <a href="#"
                            class="flex items-center p-2 text-base font-normal text-gray-900 rounded-lg dark:text-white hover:bg-gray-100 dark:hover:bg-gray-700">
                            <svg class="flex-shrink-0 w-6 h-6 text-violet-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white"
                                fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                                <path fill-rule="evenodd"
                                    d="M5 4a3 3 0 00-3 3v6a3 3 0 003 3h10a3 3 0 003-3V7a3 3 0 00-3-3H5zm-1 9v-1h5v2H5a1 1 0 01-1-1zm7 1h4a1 1 0 001-1v-1h-5v2zm0-4h5V8h-5v2zM9 8H4v2h5V8z"
                                    clip-rule="evenodd"></path>
                            </svg>
                            <span class="flex-1 ml-3 whitespace-nowrap">Sign Up</span>
                        </a>
                    </li>
                    <hr class="my-0 bg-transparent bg-gradient-to-r from-transparent via-black/40 to-transparent 
        dark:bg-gradient-to-r dark:from-transparent dark:via-white dark:to-transparent">

                    <div class="py-12">
                        <p
                            class="invisible hidden text-gray-800 text-red-500 text-red-600 text-blue-500 bg-gray-500/30 bg-cyan-500/30 bg-emerald-500/30 bg-orange-500/30 bg-red-500/30 after:bg-gradient-to-tl after:from-zinc-800 after:to-zinc-700 dark:bg-gradient-to-tl dark:from-slate-750 dark:to-gray-850 after:from-blue-700 after:to-cyan-500 after:from-orange-500 after:to-yellow-500 after:from-green-600 after:to-lime-400 after:from-red-600 after:to-orange-600 after:from-slate-600 after:to-slate-300 text-emerald-500 text-cyan-500 text-slate-400">
                        </p>
                        <div class="relative flex flex-col min-w-0 break-words bg-transparent border-0 shadow-none rounded-2xl bg-clip-border"
                            sidenav-card="">
                            <div class="flex-auto w-full p-4 pt-0 text-center">
                                <div class="transition-all duration-200 ease-nav-brand">
                                    <h6 class="mb-0 dark:text-white text-slate-700">Need help?</h6>
                                    <p class="mb-0 text-xs font-semibold leading-tight dark:text-white dark:opacity-60">
                                        Please check our docs
                                    </p>
                                </div>
                            </div>
                        </div>
                        <a href="#" target="_blank"
                            class="inline-block w-full px-8 py-2 mb-4 text-sm font-bold leading-normal text-center text-white capitalize transition-all ease-in rounded-lg shadow-md bg-violet-600 bg-150 hover:shadow-xs hover:-translate-y-px">
                            Documentation
                        </a>
                    </div>
                </ul>
            </div>
        </aside>

        

        
    </div>

</template>

<style>
.min-h-75 {
    min-height: 18.75rem;
}

.ps {
    overflow: hidden !important;
    overflow-anchor: none;
    -ms-overflow-style: none;
    touch-action: auto;
    -ms-touch-action: auto;
}

@media (min-width: 1200px) {
    .xl\:translate-x-0 {
        --tw-translate-x: 0px;
        transform: translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y));
    }
}

@media (min-width: 1200px) {
    .xl\:ml-6 {
        margin-left: 1.5rem;
    }
}

.ease-in-out {
    transition-timing-function: cubic-bezier(0.42, 0, 0.58, 1);
}

.duration-200 {
    transition-duration: 200ms;
}


.p-0 {
    padding: 0px;
}

.bg-white {
    --tw-bg-opacity: 1;
    background-color: rgb(255 255 255 / var(--tw-bg-opacity));
}

.border-0 {
    border-width: 0px;
}

.rounded-2xl {
    border-radius: 1rem;
}

.overflow-y-auto {
    overflow-y: auto;
}

.justify-between {
    justify-content: space-between;
}

.items-center {
    align-items: center;
}

.flex-wrap {
    flex-wrap: wrap;
}

.max-w-64 {
    max-width: 16rem;
}

.w-full {
    width: 100%;
}

.block {
    display: block;
}

.my-4 {
    margin-top: 1rem;
    margin-bottom: 1rem;
}

.z-990 {
    z-index: 990;
}

.left-0 {
    left: 0px;
}

.inset-y-0 {
    top: 0px;
    bottom: 0px;
}

.fixed {
    position: fixed;
}

</style>

<route lang="yaml">
    meta:
      layout: auth
      action: read
  </route>
