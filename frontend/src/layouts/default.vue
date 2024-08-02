<script lang="ts" setup>
import Header from "./components/header.vue";
import Footer from "./components/footer.vue";
import ModuleHeader from "./components/moduleheader.vue";

</script>

<template>
    <div class="text-blueGray-700 antialiased">
        <ModuleHeader />
        <Header />
        <RouterView v-slot="{ Component }">
            <Transition name="fade" mode="out-in">
                <Component :is="Component" />
            </Transition>
        </RouterView>
        <Footer />
    </div>
</template>