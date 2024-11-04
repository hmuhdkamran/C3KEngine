<script setup lang="ts">
import { registerMicroApps, start, setDefaultMountApp } from 'qiankun';
import { apps } from '@/spa';
import { onMounted } from 'vue';

onMounted(() => {
    const application = localStorage.getItem("application");
    setDefaultMountApp(`/${application}`);

    registerMicroApps(apps, {
        beforeLoad: async (app) => console.log('before load', 'color: red', app.name),
        beforeMount: async (app) => console.log('before mount', app.name),
        afterMount: async (app) => console.log('after mount', app.name),
        afterUnmount: async (app) => console.log('after unmount', app.name),
    });

    start();
});
</script>

<template>
    <div id="child-viewport"></div>
</template>

<style scoped></style>
