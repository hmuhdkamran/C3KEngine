<script lang="ts">
import { RouterLink } from 'vue-router';

</script>

<template>
    <div class="flex h-screen">
        <!-- Sidebar -->
        <aside
            class="sidebar w-64 text-gray-500 fixed z-30 inset-y-0 left-0 transition duration-500 overflow-y-auto lg:static lg:inset-0">
            <div class="py-5 flex px-3 text-white font-black bg-gray-900">
                <div class="flex items-center">
                    <span class="icon-[token-branded--eng] text-xl mr-2"></span>
                    <span class="font-bold text-lg">C3K Engine</span>
                </div>
                <span
                    class="icon-[material-symbols-light--text-select-move-back-word-rounded] block h-6 w-6 ml-auto text-gray-700 hover:text-green-400 transition-colors ease-in-out duration-100 cursor-pointer"></span>
            </div>
            <div class="sidebar-header bg-gray-800 py-5 px-4">Header</div>
            <nav class="sidebar_nav bg-gray-900 text-xs">
                <div class="sidebar_nav bg-gray-900 text-xs">
                    <RouterLink to="/"
                        class="flex items-center py-2 px-4 bg-black text-green-400 transition-colors ease-in-out duration-100"
                        href="/">
                        <span class="icon-[codicon--dashboard]"></span>
                        <span class="mx-3">Dashboard</span>
                        <span class="icon-[codicon--chevron-up] block h-4 w-4 ml-auto"></span>
                    </RouterLink>
                </div>
            </nav>
        </aside>

        <!-- Main Content -->
        <div class="bg-gray-900 sticky w-full">
            <!-- Header -->
            <div class="flex space-x-2 font-medium justify-items-center align-middle p-3">
                <div class="transition-all ease-in-out duration-300 bg-gray-800 text-white px-2 py-2.5 rounded-md text-sm">Dashboard</div>
                <div class="flex items-center space-x-4">
                    <button class="text-gray-700 hover:text-gray-900">Profile</button>
                    <button class="text-gray-700 hover:text-gray-900">Logout</button>
                </div>
            </div>

            <!-- Content Area -->
            <main class="flex-1 p-4">
                <div class="bg-white p-4 rounded shadow">
                    <h1 class="text-2xl font-bold mb-4">Welcome to the Admin Panel</h1>
                    <p class="text-gray-700">This is the main content area. Customize it as per your needs.</p>
                </div>
            </main>
        </div>
    </div>
</template>

<style>
::-webkit-scrollbar {
    width: 20px;
}

::-webkit-scrollbar-track {
    background-color: transparent;
}

::-webkit-scrollbar-thumb {
    background-color: rgb(52, 211, 153);
    border-radius: 20px;
    border: 6px solid transparent;
    background-clip: content-box;
}

::-webkit-scrollbar-thumb:hover {
    background-color: #2b004e;
}

.custom-scroll,
.sidebar,
.main_content {
    scrollbar-width: thin;
    scrollbar-color: rgba(52, 211, 153, 0.432) #C2D2E4;
}
</style>

<route lang="yaml">
    meta:
      layout: auth
      action: read
  </route>
