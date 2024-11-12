<script setup lang="ts">
import { config } from 'c3k-library';

</script>
<template>
    <footer class="fixed text-xs w-auto justify-end bottom-0 right-0 bg-gray-50 py-2 px-4 flex items-center">
        <p class="text-gray-500">
            Copyright © 2024 {{ config.application }} by
            <a href="https://combinesoft.net" class="text-gray-500 hover:text-gray-800" target="_blank">
                COMBINE CHIPSoft
            </a>
        </p>
    </footer>
</template>