<script setup lang="ts">
import { useSystemStore, VNodeRenderer } from 'c3-library'
import { RouterLink } from 'vue-router'

const config = useSystemStore()
</script>

<template>
  <nav
    class="fixed z-50 w-full top-0 flex flex-wrap items-center justify-between px-1 py-1 shadow-lg"
    :style="{ backgroundColor: config.application.titleColor }"
  >
    <div class="container px-4 mx-auto flex flex-wrap items-center justify-between">
      <div
        class="w-full relative flex justify-between lg:w-auto lg:static lg:block lg:justify-start"
      >
        <a
          class="text-sm font-bold leading-relaxed inline-block mr-4 py-1 whitespace-nowrap uppercase text-gray-700"
          href="/"
        >
          <VNodeRenderer :nodes="config.application.logo" />
          {{ config.application.name }}
        </a>
        <button
          class="cursor-pointer text-xl leading-none px-3 py-1 border border-solid border-transparent rounded bg-transparent block lg:hidden outline-none focus:outline-none"
          type="button"
        >
          <span class="fa-solid fa-bars"></span>
        </button>
      </div>
      <div class="lg:flex flex-grow items-center hidden">
        <ul class="flex flex-col lg:flex-row list-none lg:ml-auto gap-4">
          <li class="nav-item" v-for="item in config.application.socialMedia" :key="item.link">
            <a :href="item.link" target="_blank">
              <span :class="item.icon"></span>
            </a>
          </li>
          <li class="nav-item cursor-pointer">
            <RouterLink to="/authentication/login" class="btn-outline btn-primary px-4 py-1">
              Login
            </RouterLink>
          </li>
        </ul>
      </div>
    </div>
  </nav>
</template>
