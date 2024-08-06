<script setup lang="ts">
import avatar from '@/assets/images/avatar.jpg';
import logo from "@/assets/images/vue.svg";
import { ref, } from 'vue';

const isOpen = ref(false);
const showModuleDropdown = ref(false);
const showProfileDropdown = ref(false);

function toggleDropdown() {
    isOpen.value = !isOpen.value;
    showProfileDropdown.value = false;
    showModuleDropdown.value = false;
}

const toggleModule = () => {
    showModuleDropdown.value = !showModuleDropdown.value;
    isOpen.value = false;
    showProfileDropdown.value = false;
};


const toggleProfileDropdown = () => {
    showProfileDropdown.value = !showProfileDropdown.value;
    isOpen.value = false;
    showModuleDropdown.value = false;
};

</script>

<template>
    <div class="fixed top-0 left-0 w-full px-4 sm:px-6 lg:px-8 flex h-12 items-center bg-violet-500 justify-between">
        <div class="md:flex md:items-center">
            <div class="relative">
                <button @click="toggleModule"
                    class="flex items-center p-2 text-gray-600 hover:text-gray-800 focus:outline-none">
                    <span class="icon-[fluent--navigation-unread-20-filled] text-white h-7 w-7"></span>
                </button>
            </div>

            <transition name="slide">
                <div v-if="showModuleDropdown" class="fixed inset-y-0 left-0 w-64 bg-white shadow-lg z-50">
                    <div class="flex flex-col h-full style-basic">
                        <div class="flex justify-end p-4">
                            <button @click="toggleModule" class="text-gray-600 hover:text-gray-800 focus:outline-none">
                                <span class="icon-[fluent--dismiss-20-filled] h-5 w-5"></span>
                            </button>
                        </div>
                        <a href="/" class="text-xl font-semibold text-gray-800 flex items-center space-x-2 ml-4">
                            <img :src="logo" alt="C3K Engine Logo" class="h-12 w-auto object-contain" />
                            <span>C3K Engine</span>
                        </a>
                        <hr>
                        <div class="px-3 py-4 overflow-y-auto rounded ">
                            <ul class="py-6 space-y-2 ">
                                <li><a href="dashboard" class="block rounded-lg px-4 py-2 text-gray-800 hover:bg-violet-100">
                                        <span
                                            class="icon-[fluent-mdl2--recruitment-management] bg-violet-600 mx-3 w-5 h-5"></span>
                                        HRMS</a></li>
                                <li><a href="dashboard" class="block rounded-lg px-4 py-2 text-gray-800 hover:bg-violet-100">
                                        <span class="icon-[vaadin--shop] bg-violet-600 mx-3 w-5 h-5"></span>
                                        Retail</a></li>
                                <li><a href="dashboard" class="block rounded-lg px-4 py-2 text-gray-800 hover:bg-violet-100">
                                        <span class="icon-[mdi--office-building-settings-outline] bg-violet-600 mx-3 w-5 h-5"></span>
                                        Production</a></li>
                            </ul>
                        </div>
                    </div>
                </div>
            </transition>
            <a href="/" class="text-xl font-semibold text-gray-800 flex items-center space-x-2 ml-4">
                <img :src="logo" alt="C3K Engine Logo" class="h-12 w-auto object-contain" />
                <span class="text-white">C3K Engine</span>
            </a>
        </div>
        <transition name="backdrop">
            <div v-if="showModuleDropdown" class="fixed inset-0 bg-gray-800 bg-opacity-20 backdrop-blur-sm z-40"
                @click="toggleModule"></div>
        </transition>
        <div class="flex items-center">
            <div class="relative">
                <button class="btn btn-ghost btn-circle">
                    <div class="flex items-center p-2 text-gray-600 hover:text-gray-800 focus:outline-none">
                        <span class="icon-[ic--baseline-search] text-white h-6 w-6"></span>
                    </div>
                </button>
            </div>
            <div @click.stop="toggleDropdown" class="relative">
                <button class="btn btn-ghost btn-circle">
                    <div class="flex items-center p-2 text-gray-600 hover:text-gray-800 focus:outline-none">
                        <span class="icon-[mdi--bell-outline] text-white h-6 w-6"></span>
                        <span
                            class="absolute top-0 right-0 block w-4 h-4 text-xs text-white bg-red-600 rounded-full flex items-center justify-center">4</span>
                    </div>
                </button>
                <transition name="dropdown">
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
                </transition>
            </div>

            <div class="relative">
                <button @click="toggleProfileDropdown"
                    class="flex items-center p-2 text-gray-600 hover:text-gray-800 focus:outline-none gap-1">
                    <img :src="avatar" alt="Profile" class="w-9 h-9 rounded-full border-2 border-gray-200" />
                    <span class="text-white">Mitchell Admin</span>
                    <span class="icon-[ri--arrow-drop-down-line] text-white h-7 w-7"></span>
                </button>
                <transition name="dropdown">
                    <ul v-if="showProfileDropdown" class="absolute right-0 mt-2 w-52 bg-white shadow-lg rounded-lg z-10">
                        <li><a href="#" class="block px-4 py-2 text-gray-800 hover:bg-gray-100">Profile</a></li>
                        <li><a href="#" class="block px-4 py-2 text-gray-800 hover:bg-gray-100">Settings</a></li>
                        <li><a href="#" class="block px-4 py-2 text-gray-800 hover:bg-gray-100">Logout</a></li>
                    </ul>
                </transition>
            </div>
        </div>
    </div>
</template>

<style scoped>
.slide-enter-active,
.slide-leave-active {
    transition: transform 0.4s ease;
}

.slide-enter-from,
.slide-leave-to {
    transform: translateX(-100%);
}

.backdrop-enter-active,
.backdrop-leave-active {
    transition: opacity 0.3s ease;
}

.backdrop-enter,
.backdrop-leave-to {
    opacity: 0;
}

.dropdown-enter-active,
.dropdown-leave-active {
    transition: opacity 0.3s ease;
}

.dropdown-enter-from,
.dropdown-leave-to {
    opacity: 0;
}
</style>
<route lang="yaml">
      meta:
        layout: auth
        action: read
</route>