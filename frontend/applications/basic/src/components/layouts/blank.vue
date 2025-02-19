<script lang="ts" setup>

</script>

<template>
    <div class="text-blue-700 antialiased">
        <RouterView v-slot="{ Component }">
            <Transition name="fade" mode="out-in">
                <Component :is="Component" />
            </Transition>
        </RouterView>
    </div>
</template>
