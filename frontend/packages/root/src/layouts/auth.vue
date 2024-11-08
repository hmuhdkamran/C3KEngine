<script setup lang="ts">
import { TokenStatus } from "c3k-library"
import { AuthenticationService } from "@/service/authentication-service";
import AuthHeader from './components/auth/Header.vue';
import AuthFooter from './components/auth/Footer.vue';

const auth: AuthenticationService = new AuthenticationService();

</script>

<template>
    <div class="text-blueGray-700 antialiased">
        <TokenStatus :info-timeout="300" :warn-timeout="60" :error-timeout="10" :logout="true" :auth="auth" />
            <AuthHeader selectedCardTitle="Applications" />

            <RouterView v-slot="{ Component }">
                <Transition name="fade" mode="out-in">
                    <Component :is="Component" />
                </Transition>
            </RouterView>

            <AuthFooter />
        </div>
</template>

<style>
.layout-wrapper.layout-blank {
    flex-direction: column;
}
</style>
