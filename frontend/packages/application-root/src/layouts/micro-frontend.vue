<script setup lang="ts">
import { TokenStatus } from "c3k-library"
import { AuthenticationService } from "@/services/authentication-service";
import AuthHeader from "./components/authentication/Header.vue";
import AuthFooter from './components/authentication/Footer.vue';
import ChildDisplay from '@/components/ChildDisplay.vue';

const auth: AuthenticationService = new AuthenticationService();

</script>

<template>
    <div class="text-blueGray-700 antialiased">
        <TokenStatus :info-timeout="300" :warn-timeout="60" :error-timeout="10" :logout="true" :auth="auth" />
            <AuthHeader selectedCardTitle="Applications" />

            <ChildDisplay path="" class="mt-12" />

            <AuthFooter />
        </div>
</template>

<style>
.layout-wrapper.layout-blank {
    flex-direction: column;
}
</style>
