<script setup lang="ts">
import avatar from '@/assets/images/avatar.jpg';
import logo from "@/assets/images/vue.svg";
import { ref, onMounted, onUnmounted } from 'vue';

const isOpen = ref(false);
const showMenuDropdown = ref(false);
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

const toggleMenuDropdown = () => {
    showMenuDropdown.value = !showMenuDropdown.value;
    showCartDropdown.value = false;
    showProfileDropdown.value = false;
};

const toggleCartDropdown = () => {
    showCartDropdown.value = !showCartDropdown.value;
    showMenuDropdown.value = false;
    showProfileDropdown.value = false;
};

const toggleProfileDropdown = () => {
    showProfileDropdown.value = !showProfileDropdown.value;
    showCartDropdown.value = false;
    showMenuDropdown.value = false;
};
</script>

<template>
    <div class="flex h-screen">
        <div class="bg-gray-200 sticky w-full">
            <div class="bg-white shadow-md p-4 flex justify-between items-center">
                <div class="flex items-center space-x-4">
                    <div class="relative">
                        <button @click="toggleMenuDropdown"
                            class="flex items-center p-2 text-gray-600 hover:text-gray-800 focus:outline-none">
                            <span class="icon-[fluent--navigation-unread-20-filled] h-7 w-7"></span>
                        </button>
                        <ul v-if="showMenuDropdown"
                            class="absolute left-0 mt-4 w-52 bg-white shadow-lg rounded-lg z-10">
                            <li><a href="#" class="block px-4 py-2 text-gray-800 hover:bg-gray-100">Homepage</a></li>
                            <li><a href="#" class="block px-4 py-2 text-gray-800 hover:bg-gray-100">Terms</a></li>
                            <li><a href="#" class="block px-4 py-2 text-gray-800 hover:bg-gray-100">About</a></li>
                        </ul>
                    </div>
                    <a href="/" class="text-xl font-semibold text-gray-800 flex items-center space-x-2">
                        <img :src="logo" alt="C3K Engine Logo" class="h-12 w-auto object-contain" />
                        <span>C3K Engine</span>
                    </a>
                </div>
                <div class="flex items-center space-x-4">
                    <div class="relative">
                        <button class="btn btn-ghost btn-circle">
                            <div class="flex items-center p-2 text-gray-600 hover:text-gray-800 focus:outline-none">
                                <span class="icon-[ic--baseline-search] h-7 w-7"></span>
                            </div>
                        </button>
                    </div>

                    <div  @click.stop="toggleDropdown" class="relative">
                        <button class="btn btn-ghost btn-circle">
                            <div class="flex items-center p-2 text-gray-600 hover:text-gray-800 focus:outline-none">
                                <span class="icon-[mdi--bell-outline] h-7 w-7"></span>
                                <span
                                    class="absolute top-0 right-0 block w-4 h-4 text-xs text-white bg-red-600 rounded-full flex items-center justify-center">4</span>
                            </div>
                        </button>
                        <div v-if="isOpen"
                            class="absolute right-0 mt-2 w-80 bg-white shadow-lg rounded-lg overflow-hidden z-50">
                            <div class="py-2">
                                <div class="px-4 py-3 border-b border-gray-200">
                                    <h3 class="text-lg font-semibold text-gray-800">Notifications</h3>
                                </div>
                                <div class="max-h-60 overflow-y-auto">
                                    <div class="px-4 py-2">
                                        <p class="text-sm text-gray-600">You have a new message from John Doe</p>
                                        <span class="text-xs text-gray-500">2 minutes ago</span>
                                    </div>
                                    <div class="px-4 py-2 border-t border-gray-200">
                                        <p class="text-sm text-gray-600">Your leave request has been approved</p>
                                        <span class="text-xs text-gray-500">1 hour ago</span>
                                    </div>
                                    <div class="px-4 py-2 border-t border-gray-200">
                                        <p class="text-sm text-gray-600">You have a meeting at 3 PM</p>
                                        <span class="text-xs text-gray-500">3 hours ago</span>
                                    </div>
                                    <div class="px-4 py-2 border-t border-gray-200">
                                        <p class="text-sm text-gray-600">Your password will expire in 5 days</p>
                                        <span class="text-xs text-gray-500">1 day ago</span>
                                    </div>
                                </div>
                            </div>
                            <div class="px-4 py-3 bg-gray-100 text-center">
                                <button class="text-sm text-blue-500 hover:underline">View All</button>
                            </div>
                        </div>
                    </div>

                    <div class="relative">
                        <button @click="toggleCartDropdown"
                            class="flex items-center p-2 text-gray-600 hover:text-gray-800 focus:outline-none">
                            <span class="icon-[mdi--cart-outline] h-7 w-7"></span>
                            <span
                                class="absolute top-0 right-0 block w-4 h-4 text-xs text-white bg-red-600 rounded-full flex items-center justify-center">4</span>
                        </button>
                        <div v-if="showCartDropdown"
                            class="absolute right-0 mt-2 w-52 bg-white shadow-lg rounded-lg z-10">
                            <div class="p-4">
                                <span class="block text-lg font-bold">8 Items</span>
                                <button
                                    class="mt-4 w-full px-4 py-2 text-white bg-violet-500 rounded hover:bg-violet-600 focus:outline-none">View
                                    cart</button>
                            </div>
                        </div>
                    </div>
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

            <div class="flex justify-center space-x-12 mt-12">
                <a href="/app/dashboard"
                    class="w-64 p-4 py-6 bg-white shadow-lg rounded-2xl transition transform hover:scale-95 duration-200">
                    <div class="flex flex-col items-center justify-center">
                        <div class="relative w-24 h-24 bg-violet-200 rounded-full">

                            <span
                                class="icon-[fluent-mdl2--recruitment-management] absolute w-8 h-8 text-violet-700 transform -translate-x-1/2 -translate-y-1/2 left-1/2 top-1/2"></span>

                        </div>
                        <p class="mt-4 mb-4 text-xl font-medium text-gray-800">
                            HRMS
                        </p>
                        <p class="px-2 text-center text-gray-400">
                            Our HRMS software streamlines your HR processes, making it easier to manage your workforce
                            effectively and efficiently.
                        </p>
                    </div>
                </a>

                <a href="/app/dashboard"
                    class="w-64 p-4 py-6 bg-white shadow-lg rounded-2xl transition transform hover:scale-95 duration-200">
                    <div class="flex flex-col items-center justify-center">
                        <div class="relative w-24 h-24 bg-violet-200 rounded-full">

                            <span
                                class="icon-[vaadin--shop] absolute w-8 h-8 text-violet-700 transform -translate-x-1/2 -translate-y-1/2 left-1/2 top-1/2"></span>

                        </div>
                        <p class="mt-4 mb-4 text-xl font-medium text-gray-800">
                            Retail
                        </p>
                        <p class="px-2 text-center text-gray-400">
                            Elevate your retail operations with our cutting-edge software, designed to streamline
                            processes and enhance customer satisfaction.
                        </p>
                    </div>
                </a>

                <a href="/app/dashboard"
                    class="w-64 p-4 py-6 bg-white shadow-lg rounded-2xl transition transform hover:scale-95 duration-200">
                    <div class="flex flex-col items-center justify-center">
                        <div class="relative w-24 h-24 bg-violet-200 rounded-full">

                            <span
                                class="icon-[clarity--network-settings-solid] absolute w-8 h-8 text-violet-700 transform -translate-x-1/2 -translate-y-1/2 left-1/2 top-1/2"></span>

                        </div>
                        <p class="mt-4 mb-4 text-xl font-medium text-gray-800">
                            Production
                        </p>
                        <p class="px-2 text-center text-gray-400">
                            I therefore look forward to further developments in this area so that we can streamline our
                            processes and improve overall production efficiency.

                        </p>
                    </div>
                </a>
            </div>

        </div>
    </div>
</template>

<route lang="yaml">
    meta:
      layout: auth
      action: read
  </route>
