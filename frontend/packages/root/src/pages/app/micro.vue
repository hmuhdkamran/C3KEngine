<script setup lang="ts">
import { TokenStatus } from "c3k-library"
import { AuthenticationService } from '@/service/authentication-service';

import ChildDisplay from '@/components/base/ChildDisplay.vue';
import AuthHeader from '@/layouts/components/auth/Header.vue';
import AuthFooter from '@/layouts/components/auth/Footer.vue';

const auth: AuthenticationService = new AuthenticationService();

</script>

<template>
    <div class="text-blueGray-700 antialiased">
        <TokenStatus :info-timeout="300" :warn-timeout="60" :error-timeout="10" :logout="true" :auth="auth" />
        <div class="bg-white mt-12 flex flex-col">
            <AuthHeader selectedCardTitle="Applications" />

            <ChildDisplay path="" />

            <AuthFooter />
        </div>
    </div>
</template>

<route lang="yaml">
    meta:
      layout: auth
      action: read
</route>