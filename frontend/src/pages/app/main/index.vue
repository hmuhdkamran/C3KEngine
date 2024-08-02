<script setup lang="ts">
import avatar from '@/assets/images/avatar.jpg';
import logo from "@/assets/images/vue.svg";
import { ref, onMounted, onUnmounted } from 'vue';

const isOpen = ref(false);
const showModuleDropdown = ref(false);
const showCartDropdown = ref(false);
const showProfileDropdown = ref(false);

function toggleDropdown() {
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

const toggleModule = () => {
    showModuleDropdown.value = !showModuleDropdown.value;
    showCartDropdown.value = false;
    showProfileDropdown.value = false;
};


const toggleProfileDropdown = () => {
    showProfileDropdown.value = !showProfileDropdown.value;
    showCartDropdown.value = false;
    showModuleDropdown.value = false;
};

</script>

<template>
    <div class="mx-auto max-w-screen-xl px-4 sm:px-6 lg:px-8">
        <div class="flex h-16 items-center justify-between">
            <div class="md:flex md:items-center md:gap-12">
                <a href="/" class="text-xl font-semibold text-gray-800 flex items-center space-x-2">
                    <img :src="logo" alt="C3K Engine Logo" class="h-12 w-auto object-contain" />
                    <span>C3K Engine</span>
                </a>
            </div>
            <div class="hidden md:block">
                <nav aria-label="Site Nav">
                    <ul class="flex items-center gap-6 text-md">
                        <li class="relative">
                            <a @click="toggleModule" class="text-gray-500 transition hover:text-gray-500/75 cursor-pointer">Modules</a>
                            <ul v-if="showModuleDropdown"
                                class="absolute mt-2 w-48 bg-white shadow-lg rounded-lg z-10">
                                <li><a href="#" class="block px-4 py-2 text-gray-800 hover:bg-gray-100">HRMS</a></li>
                                <li><a href="#" class="block px-4 py-2 text-gray-800 hover:bg-gray-100">Retail</a></li>
                                <li><a href="#" class="block px-4 py-2 text-gray-800 hover:bg-gray-100">Production</a></li>
                            </ul>
                        </li>
                        <li>
                            <a class="text-gray-500 transition hover:text-gray-500/75" href="#">Services</a>
                        </li>
                        <li>
                            <a class="text-gray-500 transition hover:text-gray-500/75" href="#">News</a>
                        </li>
                        <li>
                            <a class="text-gray-500 transition hover:text-gray-500/75" href="#">Blog</a>
                        </li>
                    </ul>
                </nav>
            </div>
            <div class="flex items-center gap-3">
                <a href="#" class="group relative inline-block overflow-hidden rounded border border-violet-100 bg-violet-200  px-6 py-2 text-sm font-medium text-slate-800 
                    hover:text-violet-600 active:bg-violet-600 active:text-white">
                    <span
                        class="ease absolute left-0 top-0 h-0 w-0 border-t-2 border-violet-600 transition-all duration-200 group-hover:w-full"></span>
                    <span
                        class="ease absolute right-0 top-0 h-0 w-0 border-r-2 border-violet-600 transition-all duration-200 group-hover:h-full"></span>
                    <span
                        class="ease absolute bottom-0 right-0 h-0 w-0 border-b-2 border-violet-600 transition-all duration-200 group-hover:w-full"></span>
                    <span
                        class="ease absolute bottom-0 left-0 h-0 w-0 border-l-2 border-violet-600 transition-all duration-200 group-hover:h-full"></span>
                    View All Modules
                </a>
                <div class="relative">
                    <button @click="toggleProfileDropdown"
                        class="flex items-center p-2 text-gray-600 hover:text-gray-800 focus:outline-none">
                        <img :src="avatar" alt="Profile" class="w-10 h-10 rounded-full" />
                    </button>
                    <ul v-if="showProfileDropdown"
                        class="absolute right-0 mt-2 w-52 bg-white shadow-lg rounded-lg z-10">
                        <li><a href="#" class="block px-4 py-2 text-gray-800 hover:bg-gray-100">Profile</a></li>
                        <li><a href="#" class="block px-4 py-2 text-gray-800 hover:bg-gray-100">Settings</a></li>
                        <li><a href="#" class="block px-4 py-2 text-gray-800 hover:bg-gray-100">Logout</a></li>
                    </ul>
                </div>

            </div>
        </div>
    </div>
</template>

<route lang="yaml">
    meta:
      layout: auth
      action: read
  </route>