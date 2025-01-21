<script setup lang="ts">
import Header from './Header.vue';
import Sidebar from './Sidebar.vue';
import MainContent from './MainContent.vue';
</script>

<template>
    <div class="flex flex-col h-screen">
        <Header />
        <div class="flex flex-grow">
            <Sidebar />
            <MainContent />
        </div>
    </div>
</template>