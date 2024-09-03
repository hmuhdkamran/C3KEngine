<script setup lang="ts">
import { ref, computed } from 'vue';
import Card from '@/layouts/components/card.vue';
import Dashboardheader from '@/layouts/components/dashboardheader.vue';
import Sidebar from '@/layouts/components/sidebar.vue';

const isOpen = ref(false);
const showModuleDropdown = ref(false);
const showProfileDropdown = ref(false);

function toggleModule() {
    showModuleDropdown.value = !showModuleDropdown.value;
    isOpen.value = false;
    showProfileDropdown.value = false;
}

function toggleDropdown() {
    isOpen.value = !isOpen.value;
    showProfileDropdown.value = false;
    showModuleDropdown.value = false;
}

function toggleProfileDropdown() {
    showProfileDropdown.value = !showProfileDropdown.value;
    isOpen.value = false;
    showModuleDropdown.value = false;
}

const searchQuery = ref('');
const selectedCategory = ref('All');
const showFavorites = ref(false);

const cards = ref([
    {
        title: 'HRMS',
        description: 'Streamline HR with our software',
        status: 'Installed',
        buttonText: 'MODULE INFO',
        iconClass: 'icon-[fluent-mdl2--recruitment-management]',
        category: 'HRMS',
    },
    {
        title: 'Retail',
        description: 'Boost retail with streamlined software.',
        status: 'Activate',
        buttonText: 'LEARN MORE',
        iconClass: 'icon-[vaadin--shop]',
        category: 'Retail',
    },
    {
        title: 'Production',
        description: 'Looking forward to process improvements.',
        status: 'Installed',
        buttonText: 'LEARN MORE',
        iconClass: 'icon-[mdi--office-building-settings-outline]',
        category: 'Production',
    }
]);

const filteredCards = computed(() => {
    return cards.value.filter((card) => {
        const matchesCategory = selectedCategory.value === 'All' || card.category === selectedCategory.value;
        const matchesSearch = card.title.toLowerCase().includes(searchQuery.value.toLowerCase());
        return matchesCategory && matchesSearch;
    });
});

function filterCards() {

}

function filterByCategory(category: string) {
    selectedCategory.value = category;
}

function toggleFilters() {
    alert('Filters button clicked');
}

function groupByCategory() {
    alert('Group By button clicked');
}

function toggleFavorites() {
    showFavorites.value = !showFavorites.value;
    alert('Favorites button clicked');
}
</script>

<template>
    <div class="h-screen mt-12 flex flex-col">
        <div class="bg-white py-2 px-4 flex items-center justify-between">
            <div class="text-lg font-semibold w-9/12">Apps</div>
            <div class="flex items-center w-full space-x-4">
                <div class="flex w-full mx-10 rounded bg-slate-100">
                    <input
                        class=" w-full border-black bg-transparent px-4 py-1 text-black outline-none focus:outline-none "
                        type="search" name="search" placeholder="Search..." />
                    <button type="submit" class="m-2 rounded bg-slate-100 px-4 py-2 text-black">
                        <svg class="fill-current h-6 w-6" xmlns="http://www.w3.org/2000/svg"
                            xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" id="Capa_1" x="0px" y="0px"
                            viewBox="0 0 56.966 56.966" style="enable-background:new 0 0 56.966 56.966;"
                            xml:space="preserve" width="512px" height="512px">
                            <path
                                d="M55.146,51.887L41.588,37.786c3.486-4.144,5.396-9.358,5.396-14.786c0-12.682-10.318-23-23-23s-23,10.318-23,23  s10.318,23,23,23c4.761,0,9.298-1.436,13.177-4.162l13.661,14.208c0.571,0.593,1.339,0.92,2.162,0.92  c0.779,0,1.518-0.297,2.079-0.837C56.255,54.982,56.293,53.08,55.146,51.887z M23.984,6c9.374,0,17,7.626,17,17s-7.626,17-17,17  s-17-7.626-17-17S14.61,6,23.984,6z" />
                        </svg>
                    </button>
                </div>

            </div>
        </div>

        <div class="flex flex-1">
           
            <div class="min-h-screen bg-gray-100">
  <div class="sidebar min-h-screen w-[3.35rem] overflow-hidden border-r hover:w-56 hover:bg-white hover:shadow-lg">
    <div class="flex h-screen flex-col justify-between pt-2 pb-6">
      <div>
        <div class="w-max p-2.5">
          <img src="@/assets/images/vue.svg" class="w-8" alt="">
        </div>
        <ul class="mt-6 space-y-2 tracking-wide">
          <li class="min-w-max">
            <a href="#" aria-label="dashboard" class="relative flex items-center space-x-4 bg-gradient-to-r bg-pink-900 to-cyan-400 px-4 py-3 text-white">
              <svg class="-ml-1 h-6 w-6" viewBox="0 0 24 24" fill="none">
                <path d="M6 8a2 2 0 0 1 2-2h1a2 2 0 0 1 2 2v1a2 2 0 0 1-2 2H8a2 2 0 0 1-2-2V8ZM6 15a2 2 0 0 1 2-2h1a2 2 0 0 1 2 2v1a2 2 0 0 1-2 2H8a2 2 0 0 1-2-2v-1Z" class="fill-current text-cyan-400 dark:fill-slate-600"></path>
                <path d="M13 8a2 2 0 0 1 2-2h1a2 2 0 0 1 2 2v1a2 2 0 0 1-2 2h-1a2 2 0 0 1-2-2V8Z" class="fill-current text-cyan-200 group-hover:text-cyan-300"></path>
                <path d="M13 15a2 2 0 0 1 2-2h1a2 2 0 0 1 2 2v1a2 2 0 0 1-2 2h-1a2 2 0 0 1-2-2v-1Z" class="fill-current group-hover:text-sky-300"></path>
              </svg>
              <span class="-mr-1 font-medium">Dashboard</span>
            </a>
          </li>
          <li class="min-w-max">
            <a href="#" class="bg group flex items-center space-x-4 rounded-full px-4 py-3 text-gray-600">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                <path class="fill-current text-gray-300 group-hover:text-pink-900" fill-rule="evenodd" d="M2 6a2 2 0 012-2h4l2 2h4a2 2 0 012 2v1H8a3 3 0 00-3 3v1.5a1.5 1.5 0 01-3 0V6z" clip-rule="evenodd" />
                <path class="fill-current text-gray-600 group-hover:text-pink-900" d="M6 12a2 2 0 012-2h8a2 2 0 012 2v2a2 2 0 01-2 2H2h2a2 2 0 002-2v-2z" />
              </svg>
              <span class="group-hover:text-gray-700">Categories</span>
            </a>
          </li>
          <li class="min-w-max">
            <a href="#" class="group flex items-center space-x-4 rounded-md px-4 py-3 text-gray-600">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                <path class="fill-current text-gray-600 group-hover:text-pink-900" fill-rule="evenodd" d="M2 5a2 2 0 012-2h8a2 2 0 012 2v10a2 2 0 002 2H4a2 2 0 01-2-2V5zm3 1h6v4H5V6zm6 6H5v2h6v-2z" clip-rule="evenodd" />
                <path class="fill-current text-gray-300 group-hover:text-pink-900" d="M15 7h1a2 2 0 012 2v5.5a1.5 1.5 0 01-3 0V7z" />
              </svg>
              <span class="group-hover:text-gray-700">Reports</span>
            </a>
          </li>
          <li class="min-w-max">
            <a href="#" class="group flex items-center space-x-4 rounded-md px-4 py-3 text-gray-600">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                <path class="fill-current text-gray-600 group-hover:text-pink-900" d="M2 10a8 8 0 018-8v8h8a8 8 0 11-16 0z" />
                <path class="fill-current text-gray-300 group-hover:text-pink-900" d="M12 2.252A8.014 8.014 0 0117.748 8H12V2.252z" />
              </svg>
              <span class="group-hover:text-gray-700">Other data</span>
            </a>
          </li>
          <li class="min-w-max">
            <a href="#" class="group flex items-center space-x-4 rounded-md px-4 py-3 text-gray-600">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                <path class="fill-current text-gray-300 group-hover:text-pink-900" d="M4 4a2 2 0 00-2 2v1h16V6a2 2 0 00-2-2H4z" />
                <path class="fill-current text-gray-600 group-hover:text-pink-900" fill-rule="evenodd" d="M18 9H2v5a2 2 0 002 2h12a2 2 0 002-2V9zM4 13a1 1 0 011-1h1a1 1 0 110 2H5a1 1 0 01-1-1zm5-1a1 1 0 100 2h1a1 1 0 100-2H9z" clip-rule="evenodd" />
              </svg>
              <span class="group-hover:text-gray-700">Finance</span>
            </a>
          </li>
        </ul>
      </div>
      <div class="w-max -mb-3">
        <a href="#" class="group flex items-center space-x-4 rounded-md px-4 py-3 text-gray-600">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 group-hover:fill-cyan-600" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M11.49 3.17c-.38-1.56-2.6-1.56-2.98 0a1.532 1.532 0 01-2.286.948c-1.372-.836-2.942.734-2.106 2.106.54.886.061 2.042-.947 2.287-1.561.379-1.561 2.6 0 2.978a1.532 1.532 0 01.947 2.287c-.836 1.372.734 2.942 2.106 2.106a1.532 1.532 0 012.287.947c.379 1.561 2.6 1.561 2.978 0a1.533 1.533 0 012.287-.947c1.372.836 2.942-.734 2.106-2.106a1.533 1.533 0 01.947-2.287c1.561-.379 1.561-2.6 0-2.978a1.532 1.532 0 01-.947-2.287c.836-1.372-.734-2.942-2.106-2.106a1.532 1.532 0 01-2.287-.947zM10 13a3 3 0 100-6 3 3 0 000 6z" clip-rule="evenodd" />
          </svg>
          <span class="group-hover:text-gray-700">Settings</span>
        </a>
      </div>
    </div>
  </div>
</div>
            <div class="flex-row">
                <div class="grid gird-col-2 gap-4  sm:grid-col-2 md:grid-cols-4 lg:grid-cols-4 xl:grid-cols-6 mr-auto">
                    <a href=""
                        class="p-4 transition-colors duration-200 bg-white border border-gray-200  dark:hover:bg-gray-800 dark:bg-gray-900 dark:border-gray-700 hover:bg-gray-100">
                        <div class="flex items-center gap-x-3">
                            <img src="https://www.creative-tim.com/twcomponents/storage/avatars/TpXC86kLAsEjEFhN5zBp9Q8a8mzO4aigKXwfc5zm.jpg"
                                alt="Creative Tim Profile on Tailwind Components Website"
                                class="object-cover w-12 h-12 rounded-full">
                            <div class="flex-1">
                                <div class="flex items-center justify-between">
                                    <h2 class="font-semibold tracking-wide text-gray-800 dark:text-white">Creative Tim
                                    </h2>

                                </div>
                                <p class="mt-1 text-sm tracking-wide text-gray-600 dark:text-gray-400">196 Components
                                </p>
                            </div>
                        </div>
                    </a> <a href=""
                        class="p-4 transition-colors duration-200 bg-white border border-gray-200  dark:hover:bg-gray-800 dark:bg-gray-900 dark:border-gray-700 hover:bg-gray-100">
                        <div class="flex items-center gap-x-3"><img
                                src="https://www.gravatar.com/avatar/6dc5e4950b260687309cb98b1e54a539"
                                alt="banny Profile on Tailwind Components Website"
                                class="object-cover w-12 h-12 rounded-full">
                            <div class="flex-1">
                                <div class="flex items-center justify-between">
                                    <h2 class="font-semibold tracking-wide text-gray-800 dark:text-white">banny</h2>
                                    <p class="mt-1 text-sm font-medium tracking-wide text-gray-600 dark:text-gray-400">
                                        #2</p>
                                </div>
                                <p class="mt-1 text-sm tracking-wide text-gray-600 dark:text-gray-400">123 Components
                                </p>
                            </div>
                        </div>
                    </a> <a href=""
                        class="p-4 transition-colors duration-200 bg-white border border-gray-200  dark:hover:bg-gray-800 dark:bg-gray-900 dark:border-gray-700 hover:bg-gray-100">
                        <div class="flex items-center gap-x-3"><img
                                src="https://www.creative-tim.com/twcomponents/storage/avatars/WkI7PkGHABCVvWWVtUHrC0XaDCmoUKDN5t5mpAwQ.jpg"
                                alt="Shehab coding Profile on Tailwind Components Website"
                                class="object-cover w-12 h-12 rounded-full">
                            <div class="flex-1">
                                <div class="flex items-center justify-between">
                                    <h2 class="font-semibold tracking-wide text-gray-800 dark:text-white">Shehab coding
                                    </h2>
                                    <p class="mt-1 text-sm font-medium tracking-wide text-gray-600 dark:text-gray-400">
                                        #3</p>
                                </div>
                                <p class="mt-1 text-sm tracking-wide text-gray-600 dark:text-gray-400">97 Components</p>
                            </div>
                        </div>
                    </a> <a href=""
                        class="p-4 transition-colors duration-200 bg-white border border-gray-200 dark:hover:bg-gray-800 dark:bg-gray-900 dark:border-gray-700 hover:bg-gray-100">
                        <div class="flex items-center gap-x-3"><img
                                src="https://www.creative-tim.com/twcomponents/storage/avatars/ckQ2aybMQQJgUPv1HOe9XbpNmQmltLlWyy4CSVuJ.jpg"
                                alt="khatabwedaa Profile on Tailwind Components Website"
                                class="object-cover w-12 h-12 rounded-full">
                            <div class="flex-1">
                                <div class="flex items-center justify-between">
                                    <h2 class="font-semibold tracking-wide text-gray-800 dark:text-white">khatabwedaa
                                    </h2>
                                    <p class="mt-1 text-sm font-medium tracking-wide text-gray-600 dark:text-gray-400">
                                        #4</p>
                                </div>
                                <p class="mt-1 text-sm tracking-wide text-gray-600 dark:text-gray-400">92 Components</p>
                            </div>
                        </div>
                    </a> <a href=""
                        class="p-4 transition-colors duration-200 bg-white border border-gray-200 dark:hover:bg-gray-800 dark:bg-gray-900 dark:border-gray-700 hover:bg-gray-100">
                        <div class="flex items-center gap-x-3"><img
                                src="https://www.creative-tim.com/twcomponents/storage/avatars/ILXgUJYPjEo0P9gYmGhFUpUfK0VaR4l7yAfAUMuS.png"
                                alt="Harrishash Profile on Tailwind Components Website"
                                class="object-cover w-12 h-12 rounded-full">
                            <div class="flex-1">
                                <div class="flex items-center justify-between">
                                    <h2 class="font-semibold tracking-wide text-gray-800 dark:text-white">Harrishash
                                    </h2>
                                    <p class="mt-1 text-sm font-medium tracking-wide text-gray-600 dark:text-gray-400">
                                        #5</p>
                                </div>
                                <p class="mt-1 text-sm tracking-wide text-gray-600 dark:text-gray-400">81 Components</p>
                            </div>
                        </div>
                    </a> <a href=""
                        class="p-4 transition-colors duration-200 bg-white border border-gray-200 dark:hover:bg-gray-800 dark:bg-gray-900 dark:border-gray-700 hover:bg-gray-100">
                        <div class="flex items-center gap-x-3"><img
                                src="https://www.creative-tim.com/twcomponents/storage/avatars/21ZisJofZDGSjmtcFGoLN7QhbCE6Np1rzwjqUZIN.jpg"
                                alt="Material Tailwind Profile on Tailwind Components Website"
                                class="object-cover w-12 h-12 rounded-full">
                            <div class="flex-1">
                                <div class="flex items-center justify-between">
                                    <h2 class="font-semibold tracking-wide text-gray-800 dark:text-white">Material
                                        Tailwind</h2>
                                    <p class="mt-1 text-sm font-medium tracking-wide text-gray-600 dark:text-gray-400">
                                        #6</p>
                                </div>
                                <p class="mt-1 text-sm tracking-wide text-gray-600 dark:text-gray-400">81 Components</p>
                            </div>
                        </div>
                    </a> <a href=""
                        class="p-4 transition-colors duration-200 bg-white border border-gray-200 dark:hover:bg-gray-800 dark:bg-gray-900 dark:border-gray-700 hover:bg-gray-100">
                        <div class="flex items-center gap-x-3"><img
                                src="https://www.creative-tim.com/twcomponents/storage/avatars/hvDqTaSchLjQZhxZu4Fksqia6vYMKwdv1IScC0vD.jpg"
                                alt="zoltanszogyenyi Profile on Tailwind Components Website"
                                class="object-cover w-12 h-12 rounded-full">
                            <div class="flex-1">
                                <div class="flex items-center justify-between">
                                    <h2 class="font-semibold tracking-wide text-gray-800 dark:text-white">
                                        zoltanszogyenyi</h2>
                                    <p class="mt-1 text-sm font-medium tracking-wide text-gray-600 dark:text-gray-400">
                                        #7</p>
                                </div>
                                <p class="mt-1 text-sm tracking-wide text-gray-600 dark:text-gray-400">65 Components</p>
                            </div>
                        </div>
                    </a> <a href=" "
                        class="p-4 transition-colors duration-200 bg-white border border-gray-200 dark:hover:bg-gray-800 dark:bg-gray-900 dark:border-gray-700 hover:bg-gray-100">
                        <div class="flex items-center gap-x-3"><img
                                src="https://www.creative-tim.com/twcomponents/storage/avatars/HOh38rDP25DJDFRZh4FIZFZ1pNLJTM9D0vdbD3K4.jpg"
                                alt="Yousef Najib Profile on Tailwind Components Website"
                                class="object-cover w-12 h-12 rounded-full">
                            <div class="flex-1">
                                <div class="flex items-center justify-between">
                                    <h2 class="font-semibold tracking-wide text-gray-800 dark:text-white">Yousef Najib
                                    </h2>
                                    <p class="mt-1 text-sm font-medium tracking-wide text-gray-600 dark:text-gray-400">
                                        #8</p>
                                </div>
                                <p class="mt-1 text-sm tracking-wide text-gray-600 dark:text-gray-400">56 Components</p>
                            </div>
                        </div>
                    </a>
                    <a href=" "
                        class="p-4 transition-colors duration-200 bg-white border border-gray-200 dark:hover:bg-gray-800 dark:bg-gray-900 dark:border-gray-700 hover:bg-gray-100">
                        <div class="flex items-center gap-x-3"><img
                                src="https://www.creative-tim.com/twcomponents/storage/avatars/HOh38rDP25DJDFRZh4FIZFZ1pNLJTM9D0vdbD3K4.jpg"
                                alt="Yousef Najib Profile on Tailwind Components Website"
                                class="object-cover w-12 h-12 rounded-full">
                            <div class="flex-1">
                                <div class="flex items-center justify-between">
                                    <h2 class="font-semibold tracking-wide text-gray-800 dark:text-white">Yousef Najib
                                    </h2>
                                    <p class="mt-1 text-sm font-medium tracking-wide text-gray-600 dark:text-gray-400">
                                        #8</p>
                                </div>
                                <p class="mt-1 text-sm tracking-wide text-gray-600 dark:text-gray-400">56 Components</p>
                            </div>
                        </div>
                    </a>
                    <a href=" "
                        class="p-4 transition-colors duration-200 bg-white border border-gray-200 dark:hover:bg-gray-800 dark:bg-gray-900 dark:border-gray-700 hover:bg-gray-100">
                        <div class="flex items-center gap-x-3"><img
                                src="https://www.creative-tim.com/twcomponents/storage/avatars/HOh38rDP25DJDFRZh4FIZFZ1pNLJTM9D0vdbD3K4.jpg"
                                alt="Yousef Najib Profile on Tailwind Components Website"
                                class="object-cover w-12 h-12 rounded-full">
                            <div class="flex-1">
                                <div class="flex items-center justify-between">
                                    <h2 class="font-semibold tracking-wide text-gray-800 dark:text-white">Yousef Najib
                                    </h2>
                                    <p class="mt-1 text-sm font-medium tracking-wide text-gray-600 dark:text-gray-400">
                                        #8</p>
                                </div>
                                <p class="mt-1 text-sm tracking-wide text-gray-600 dark:text-gray-400">56 Components</p>
                            </div>
                        </div>
                    </a>
                    <a href=""
                        class="p-4 transition-colors duration-200 bg-white border border-gray-200 dark:hover:bg-gray-800 dark:bg-gray-900 dark:border-gray-700 hover:bg-gray-100">
                        <div class="flex items-center gap-x-3"><img
                                src="https://www.creative-tim.com/twcomponents/storage/avatars/HOh38rDP25DJDFRZh4FIZFZ1pNLJTM9D0vdbD3K4.jpg"
                                alt="Yousef Najib Profile on Tailwind Components Website"
                                class="object-cover w-12 h-12 rounded-full">
                            <div class="flex-1">
                                <div class="flex items-center justify-between">
                                    <h2 class="font-semibold tracking-wide text-gray-800 dark:text-white">Yousef Najib
                                    </h2>
                                    <p class="mt-1 text-sm font-medium tracking-wide text-gray-600 dark:text-gray-400">
                                        #8</p>
                                </div>
                                <p class="mt-1 text-sm tracking-wide text-gray-600 dark:text-gray-400">56 Components</p>
                            </div>
                        </div>
                    </a>
                    <a href=""
                        class="p-4 transition-colors duration-200 bg-white border border-gray-200 dark:hover:bg-gray-800 dark:bg-gray-900 dark:border-gray-700 hover:bg-gray-100">
                        <div class="flex items-center gap-x-3"><img
                                src="https://www.creative-tim.com/twcomponents/storage/avatars/HOh38rDP25DJDFRZh4FIZFZ1pNLJTM9D0vdbD3K4.jpg"
                                alt="Yousef Najib Profile on Tailwind Components Website"
                                class="object-cover w-12 h-12 rounded-full">
                            <div class="flex-1">
                                <div class="flex items-center justify-between">
                                    <h2 class="font-semibold tracking-wide text-gray-800 dark:text-white">Yousef Najib
                                    </h2>
                                    <p class="mt-1 text-sm font-medium tracking-wide text-gray-600 dark:text-gray-400">
                                        #8</p>
                                </div>
                                <p class="mt-1 text-sm tracking-wide text-gray-600 dark:text-gray-400">56 Components</p>
                            </div>
                        </div>
                    </a>
                    <a href=""
                        class="p-4 transition-colors duration-200 bg-white border border-gray-200 dark:hover:bg-gray-800 dark:bg-gray-900 dark:border-gray-700 hover:bg-gray-100">
                        <div class="flex items-center gap-x-3"><img
                                src="https://www.creative-tim.com/twcomponents/storage/avatars/HOh38rDP25DJDFRZh4FIZFZ1pNLJTM9D0vdbD3K4.jpg"
                                alt="Yousef Najib Profile on Tailwind Components Website"
                                class="object-cover w-12 h-12 rounded-full">
                            <div class="flex-1">
                                <div class="flex items-center justify-between">
                                    <h2 class="font-semibold tracking-wide text-gray-800 dark:text-white">Yousef Najib
                                    </h2>
                                    <p class="mt-1 text-sm font-medium tracking-wide text-gray-600 dark:text-gray-400">
                                        #8</p>
                                </div>
                                <p class="mt-1 text-sm tracking-wide text-gray-600 dark:text-gray-400">56 Components</p>
                            </div>
                        </div>
                    </a>
                    <a href=""
                        class="p-4 transition-colors duration-200 bg-white border border-gray-200 dark:hover:bg-gray-800 dark:bg-gray-900 dark:border-gray-700 hover:bg-gray-100">
                        <div class="flex items-center gap-x-3"><img
                                src="https://www.creative-tim.com/twcomponents/storage/avatars/HOh38rDP25DJDFRZh4FIZFZ1pNLJTM9D0vdbD3K4.jpg"
                                alt="Yousef Najib Profile on Tailwind Components Website"
                                class="object-cover w-12 h-12 rounded-full">
                            <div class="flex-1">
                                <div class="flex items-center justify-between">
                                    <h2 class="font-semibold tracking-wide text-gray-800 dark:text-white">Yousef Najib
                                    </h2>
                                    <p class="mt-1 text-sm font-medium tracking-wide text-gray-600 dark:text-gray-400">
                                        #8</p>
                                </div>
                                <p class="mt-1 text-sm tracking-wide text-gray-600 dark:text-gray-400">56 Components</p>
                            </div>
                        </div>
                    </a>
                    <a href=""
                        class="p-4 transition-colors duration-200 bg-white border border-gray-200 dark:hover:bg-gray-800 dark:bg-gray-900 dark:border-gray-700 hover:bg-gray-100">
                        <div class="flex items-center gap-x-3"><img
                                src="https://www.creative-tim.com/twcomponents/storage/avatars/HOh38rDP25DJDFRZh4FIZFZ1pNLJTM9D0vdbD3K4.jpg"
                                alt="Yousef Najib Profile on Tailwind Components Website"
                                class="object-cover w-12 h-12 rounded-full">
                            <div class="flex-1">
                                <div class="flex items-center justify-between">
                                    <h2 class="font-semibold tracking-wide text-gray-800 dark:text-white">Yousef Najib
                                    </h2>
                                    <p class="mt-1 text-sm font-medium tracking-wide text-gray-600 dark:text-gray-400">
                                        #8</p>
                                </div>
                                <p class="mt-1 text-sm tracking-wide text-gray-600 dark:text-gray-400">56 Components</p>
                            </div>
                        </div>
                    </a>
                    <a href=""
                        class="p-4 transition-colors duration-200 bg-white border border-gray-200 dark:hover:bg-gray-800 dark:bg-gray-900 dark:border-gray-700 hover:bg-gray-100">
                        <div class="flex items-center gap-x-3"><img
                                src="https://www.creative-tim.com/twcomponents/storage/avatars/HOh38rDP25DJDFRZh4FIZFZ1pNLJTM9D0vdbD3K4.jpg"
                                alt="Yousef Najib Profile on Tailwind Components Website"
                                class="object-cover w-12 h-12 rounded-full">
                            <div class="flex-1">
                                <div class="flex items-center justify-between">
                                    <h2 class="font-semibold tracking-wide text-gray-800 dark:text-white">Yousef Najib
                                    </h2>
                                    <p class="mt-1 text-sm font-medium tracking-wide text-gray-600 dark:text-gray-400">
                                        #8</p>
                                </div>
                                <p class="mt-1 text-sm tracking-wide text-gray-600 dark:text-gray-400">56 Components</p>
                            </div>
                        </div>
                    </a>
                    <a href=""
                        class="p-4 transition-colors duration-200 bg-white border border-gray-200 dark:hover:bg-gray-800 dark:bg-gray-900 dark:border-gray-700 hover:bg-gray-100">
                        <div class="flex items-center gap-x-3"><img
                                src="https://www.creative-tim.com/twcomponents/storage/avatars/HOh38rDP25DJDFRZh4FIZFZ1pNLJTM9D0vdbD3K4.jpg"
                                alt="Yousef Najib Profile on Tailwind Components Website"
                                class="object-cover w-12 h-12 rounded-full">
                            <div class="flex-1">
                                <div class="flex items-center justify-between">
                                    <h2 class="font-semibold tracking-wide text-gray-800 dark:text-white">Yousef Najib
                                    </h2>
                                    <p class="mt-1 text-sm font-medium tracking-wide text-gray-600 dark:text-gray-400">
                                        #8</p>
                                </div>
                                <p class="mt-1 text-sm tracking-wide text-gray-600 dark:text-gray-400">56 Components</p>
                            </div>
                        </div>
                    </a>
                    <a href=""
                        class="p-4 transition-colors duration-200 bg-white border border-gray-200 dark:hover:bg-gray-800 dark:bg-gray-900 dark:border-gray-700 hover:bg-gray-100">
                        <div class="flex items-center gap-x-3"><img
                                src="https://www.creative-tim.com/twcomponents/storage/avatars/HOh38rDP25DJDFRZh4FIZFZ1pNLJTM9D0vdbD3K4.jpg"
                                alt="Yousef Najib Profile on Tailwind Components Website"
                                class="object-cover w-12 h-12 rounded-full">
                            <div class="flex-1">
                                <div class="flex items-center justify-between">
                                    <h2 class="font-semibold tracking-wide text-gray-800 dark:text-white">Yousef Najib
                                    </h2>
                                    <p class="mt-1 text-sm font-medium tracking-wide text-gray-600 dark:text-gray-400">
                                        #8</p>
                                </div>
                                <p class="mt-1 text-sm tracking-wide text-gray-600 dark:text-gray-400">56 Components</p>
                            </div>
                        </div>
                    </a>
                    <a href=""
                        class="p-4 transition-colors duration-200 bg-white border border-gray-200 dark:hover:bg-gray-800 dark:bg-gray-900 dark:border-gray-700 hover:bg-gray-100">
                        <div class="flex items-center gap-x-3"><img
                                src="https://www.creative-tim.com/twcomponents/storage/avatars/HOh38rDP25DJDFRZh4FIZFZ1pNLJTM9D0vdbD3K4.jpg"
                                alt="Yousef Najib Profile on Tailwind Components Website"
                                class="object-cover w-12 h-12 rounded-full">
                            <div class="flex-1">
                                <div class="flex items-center justify-between">
                                    <h2 class="font-semibold tracking-wide text-gray-800 dark:text-white">Yousef Najib
                                    </h2>
                                    <p class="mt-1 text-sm font-medium tracking-wide text-gray-600 dark:text-gray-400">
                                        #8</p>
                                </div>
                                <p class="mt-1 text-sm tracking-wide text-gray-600 dark:text-gray-400">56 Components</p>
                            </div>
                        </div>
                    </a>
                    <a href=""
                        class="p-4 transition-colors duration-200 bg-white border border-gray-200 dark:hover:bg-gray-800 dark:bg-gray-900 dark:border-gray-700 hover:bg-gray-100">
                        <div class="flex items-center gap-x-3"><img
                                src="https://www.creative-tim.com/twcomponents/storage/avatars/HOh38rDP25DJDFRZh4FIZFZ1pNLJTM9D0vdbD3K4.jpg"
                                alt="Yousef Najib Profile on Tailwind Components Website"
                                class="object-cover w-12 h-12 rounded-full">
                            <div class="flex-1">
                                <div class="flex items-center justify-between">
                                    <h2 class="font-semibold tracking-wide text-gray-800 dark:text-white">Yousef Najib
                                    </h2>
                                    <p class="mt-1 text-sm font-medium tracking-wide text-gray-600 dark:text-gray-400">
                                        #8</p>
                                </div>
                                <p class="mt-1 text-sm tracking-wide text-gray-600 dark:text-gray-400">56 Components</p>
                            </div>
                        </div>
                    </a>
                    <a href=""
                        class="p-4 transition-colors duration-200 bg-white border border-gray-200 dark:hover:bg-gray-800 dark:bg-gray-900 dark:border-gray-700 hover:bg-gray-100">
                        <div class="flex items-center gap-x-3"><img
                                src="https://www.creative-tim.com/twcomponents/storage/avatars/HOh38rDP25DJDFRZh4FIZFZ1pNLJTM9D0vdbD3K4.jpg"
                                alt="Yousef Najib Profile on Tailwind Components Website"
                                class="object-cover w-12 h-12 rounded-full">
                            <div class="flex-1">
                                <div class="flex items-center justify-between">
                                    <h2 class="font-semibold tracking-wide text-gray-800 dark:text-white">Yousef Najib
                                    </h2>
                                    <p class="mt-1 text-sm font-medium tracking-wide text-gray-600 dark:text-gray-400">
                                        #8</p>
                                </div>
                                <p class="mt-1 text-sm tracking-wide text-gray-600 dark:text-gray-400">56 Components</p>
                            </div>
                        </div>
                    </a>
                    <a href=""
                        class="p-4 transition-colors duration-200 bg-white border border-gray-200 dark:hover:bg-gray-800 dark:bg-gray-900 dark:border-gray-700 hover:bg-gray-100">
                        <div class="flex items-center gap-x-3"><img
                                src="https://www.creative-tim.com/twcomponents/storage/avatars/HOh38rDP25DJDFRZh4FIZFZ1pNLJTM9D0vdbD3K4.jpg"
                                alt="Yousef Najib Profile on Tailwind Components Website"
                                class="object-cover w-12 h-12 rounded-full">
                            <div class="flex-1">
                                <div class="flex items-center justify-between">
                                    <h2 class="font-semibold tracking-wide text-gray-800 dark:text-white">Yousef Najib
                                    </h2>
                                    <p class="mt-1 text-sm font-medium tracking-wide text-gray-600 dark:text-gray-400">
                                        #8</p>
                                </div>
                                <p class="mt-1 text-sm tracking-wide text-gray-600 dark:text-gray-400">56 Components</p>
                            </div>
                        </div>
                    </a>
                    <a href=""
                        class="p-4 transition-colors duration-200 bg-white border border-gray-200 dark:hover:bg-gray-800 dark:bg-gray-900 dark:border-gray-700 hover:bg-gray-100">
                        <div class="flex items-center gap-x-3"><img
                                src="https://www.creative-tim.com/twcomponents/storage/avatars/HOh38rDP25DJDFRZh4FIZFZ1pNLJTM9D0vdbD3K4.jpg"
                                alt="Yousef Najib Profile on Tailwind Components Website"
                                class="object-cover w-12 h-12 rounded-full">
                            <div class="flex-1">
                                <div class="flex items-center justify-between">
                                    <h2 class="font-semibold tracking-wide text-gray-800 dark:text-white">Yousef Najib
                                    </h2>
                                    <p class="mt-1 text-sm font-medium tracking-wide text-gray-600 dark:text-gray-400">
                                        #8</p>
                                </div>
                                <p class="mt-1 text-sm tracking-wide text-gray-600 dark:text-gray-400">56 Components</p>
                            </div>
                        </div>
                    </a>
                    <a href=""
                        class="p-4 transition-colors duration-200 bg-white border border-gray-200 dark:hover:bg-gray-800 dark:bg-gray-900 dark:border-gray-700 hover:bg-gray-100">
                        <div class="flex items-center gap-x-3"><img
                                src="https://www.creative-tim.com/twcomponents/storage/avatars/HOh38rDP25DJDFRZh4FIZFZ1pNLJTM9D0vdbD3K4.jpg"
                                alt="Yousef Najib Profile on Tailwind Components Website"
                                class="object-cover w-12 h-12 rounded-full">
                            <div class="flex-1">
                                <div class="flex items-center justify-between">
                                    <h2 class="font-semibold tracking-wide text-gray-800 dark:text-white">Yousef Najib
                                    </h2>
                                    <p class="mt-1 text-sm font-medium tracking-wide text-gray-600 dark:text-gray-400">
                                        #8</p>
                                </div>
                                <p class="mt-1 text-sm tracking-wide text-gray-600 dark:text-gray-400">56 Components</p>
                            </div>
                        </div>
                    </a>
                    <a href=""
                        class="p-4 transition-colors duration-200 bg-white border border-gray-200 dark:hover:bg-gray-800 dark:bg-gray-900 dark:border-gray-700 hover:bg-gray-100">
                        <div class="flex items-center gap-x-3"><img
                                src="https://www.creative-tim.com/twcomponents/storage/avatars/HOh38rDP25DJDFRZh4FIZFZ1pNLJTM9D0vdbD3K4.jpg"
                                alt="Yousef Najib Profile on Tailwind Components Website"
                                class="object-cover w-12 h-12 rounded-full">
                            <div class="flex-1">
                                <div class="flex items-center justify-between">
                                    <h2 class="font-semibold tracking-wide text-gray-800 dark:text-white">Yousef Najib
                                    </h2>
                                    <p class="mt-1 text-sm font-medium tracking-wide text-gray-600 dark:text-gray-400">
                                        #8</p>
                                </div>
                                <p class="mt-1 text-sm tracking-wide text-gray-600 dark:text-gray-400">56 Components</p>
                            </div>
                        </div>
                    </a>
                    <a href=""
                        class="p-4 transition-colors duration-200 bg-white border border-gray-200 dark:hover:bg-gray-800 dark:bg-gray-900 dark:border-gray-700 hover:bg-gray-100">
                        <div class="flex items-center gap-x-3"><img
                                src="https://www.creative-tim.com/twcomponents/storage/avatars/HOh38rDP25DJDFRZh4FIZFZ1pNLJTM9D0vdbD3K4.jpg"
                                alt="Yousef Najib Profile on Tailwind Components Website"
                                class="object-cover w-12 h-12 rounded-full">
                            <div class="flex-1">
                                <div class="flex items-center justify-between">
                                    <h2 class="font-semibold tracking-wide text-gray-800 dark:text-white">Yousef Najib
                                    </h2>
                                    <p class="mt-1 text-sm font-medium tracking-wide text-gray-600 dark:text-gray-400">
                                        #8</p>
                                </div>
                                <p class="mt-1 text-sm tracking-wide text-gray-600 dark:text-gray-400">56 Components</p>
                            </div>
                        </div>
                    </a>
                    <a href=""
                        class="p-4 transition-colors duration-200 bg-white border border-gray-200 dark:hover:bg-gray-800 dark:bg-gray-900 dark:border-gray-700 hover:bg-gray-100">
                        <div class="flex items-center gap-x-3"><img
                                src="https://www.creative-tim.com/twcomponents/storage/avatars/HOh38rDP25DJDFRZh4FIZFZ1pNLJTM9D0vdbD3K4.jpg"
                                alt="Yousef Najib Profile on Tailwind Components Website"
                                class="object-cover w-12 h-12 rounded-full">
                            <div class="flex-1">
                                <div class="flex items-center justify-between">
                                    <h2 class="font-semibold tracking-wide text-gray-800 dark:text-white">Yousef Najib
                                    </h2>
                                    <p class="mt-1 text-sm font-medium tracking-wide text-gray-600 dark:text-gray-400">
                                        #8</p>
                                </div>
                                <p class="mt-1 text-sm tracking-wide text-gray-600 dark:text-gray-400">56 Components</p>
                            </div>
                        </div>
                    </a>
                    <a href=""
                        class="p-4 transition-colors duration-200 bg-white border border-gray-200 dark:hover:bg-gray-800 dark:bg-gray-900 dark:border-gray-700 hover:bg-gray-100">
                        <div class="flex items-center gap-x-3"><img
                                src="https://www.creative-tim.com/twcomponents/storage/avatars/HOh38rDP25DJDFRZh4FIZFZ1pNLJTM9D0vdbD3K4.jpg"
                                alt="Yousef Najib Profile on Tailwind Components Website"
                                class="object-cover w-12 h-12 rounded-full">
                            <div class="flex-1">
                                <div class="flex items-center justify-between">
                                    <h2 class="font-semibold tracking-wide text-gray-800 dark:text-white">Yousef Najib
                                    </h2>
                                    <p class="mt-1 text-sm font-medium tracking-wide text-gray-600 dark:text-gray-400">
                                        #8</p>
                                </div>
                                <p class="mt-1 text-sm tracking-wide text-gray-600 dark:text-gray-400">56 Components</p>
                            </div>
                        </div>
                    </a>
                    <a href=""
                        class="p-4 transition-colors duration-200 bg-white border border-gray-200 dark:hover:bg-gray-800 dark:bg-gray-900 dark:border-gray-700 hover:bg-gray-100">
                        <div class="flex items-center gap-x-3"><img
                                src="https://www.creative-tim.com/twcomponents/storage/avatars/HOh38rDP25DJDFRZh4FIZFZ1pNLJTM9D0vdbD3K4.jpg"
                                alt="Yousef Najib Profile on Tailwind Components Website"
                                class="object-cover w-12 h-12 rounded-full">
                            <div class="flex-1">
                                <div class="flex items-center justify-between">
                                    <h2 class="font-semibold tracking-wide text-gray-800 dark:text-white">Yousef Najib
                                    </h2>
                                    <p class="mt-1 text-sm font-medium tracking-wide text-gray-600 dark:text-gray-400">
                                        #8</p>
                                </div>
                                <p class="mt-1 text-sm tracking-wide text-gray-600 dark:text-gray-400">56 Components</p>
                            </div>
                        </div>
                    </a>
                    <a href=" "
                        class="p-4 transition-colors duration-200 bg-white border border-gray-200 dark:hover:bg-gray-800 dark:bg-gray-900 dark:border-gray-700 hover:bg-gray-100">
                        <div class="flex items-center gap-x-3"><img
                                src="https://www.creative-tim.com/twcomponents/storage/avatars/HOh38rDP25DJDFRZh4FIZFZ1pNLJTM9D0vdbD3K4.jpg"
                                alt="Yousef Najib Profile on Tailwind Components Website"
                                class="object-cover w-12 h-12 rounded-full">
                            <div class="flex-1">
                                <div class="flex items-center justify-between">
                                    <h2 class="font-semibold tracking-wide text-gray-800 dark:text-white">Yousef Najib
                                    </h2>
                                    <p class="mt-1 text-sm font-medium tracking-wide text-gray-600 dark:text-gray-400">
                                        #8</p>
                                </div>
                                <p class="mt-1 text-sm tracking-wide text-gray-600 dark:text-gray-400">56 Components</p>
                            </div>
                        </div>
                    </a>
                </div>
            </div>
        </div>


    </div>
    <Dashboardheader />

</template>

<route lang="yaml">
    meta:
      layout: auth
      action: read
</route>