<script lang="ts">

</script>

<template>
    <RouterView v-slot="{ Component }">
        <Transition name="fade" mode="out-in">
            <Component :is="Component" />
        </Transition>
    </RouterView>
</template>

<style>
.layout-wrapper.layout-blank {
    flex-direction: column;
}
</style>
