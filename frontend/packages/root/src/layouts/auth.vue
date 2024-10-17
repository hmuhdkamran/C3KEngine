<script lang="ts">

</script>

<template>
    <div class="text-blueGray-700 antialiased">
        <TokenStatus :info-timeout="300" :warn-timeout="60" :error-timeout="10" :logout="true" />
        <RouterView v-slot="{ Component }">
            <Transition name="fade" mode="out-in">
                <Component :is="Component" />
            </Transition>
        </RouterView>
    </div>
</template>

<style>
.layout-wrapper.layout-blank {
    flex-direction: column;
}
</style>
