<script setup lang="ts">
import ChildDisplay from '@/components/ChildDisplay.vue';
import AuthFooter from '@/layouts/components/authentication/Footer.vue';

</script>

<template>
    <div class="text-blueGray-700 antialiased">
        <div class="bg-white mt-12 flex flex-col">
            <ChildDisplay path="" />
            <AuthFooter />
        </div>
    </div>
</template>

<route lang="yaml">
    meta:
      layout: authentication
      action: read
</route>