<script lang="ts" setup>
import Header from "./components/default/header.vue";
import Footer from "./components/default/footer.vue";

</script>

<template>
    <div class="text-blueGray-700 antialiased">
        <Header />
        <RouterView v-slot="{ Component }">
            <Transition name="fade" mode="out-in">
                <Component :is="Component" />
            </Transition>
        </RouterView>
        <Footer />
    </div>
</template>