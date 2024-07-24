<script setup lang="ts">
import { ref, Ref } from "vue";
import { useRoute, useRouter } from "vue-router";
import { AuthenticationService } from "@/service/auth/authentication-service";

import logo from "@/assets/vue.svg"
import broken from "@/assets/login.svg"

interface ICredential {
    username: string
    password: string
}

const service: AuthenticationService = new AuthenticationService();

const route = useRoute()
const router = useRouter()

const email: Ref<string> = ref('admin@sefam.com')
const password: Ref<string> = ref('P@ssw0rd')

const login = () => {
    const credentials: ICredential = {
        username: email.value.toLowerCase(),
        password: password.value,
    }

    service.login(credentials)
        .then(response => {
            if (response) {
                // ability.update(JSON.parse(TokenHelper.getAbilities()))
                router.replace(route.query.to ? String(route.query.to) : '/app/dashboard')
            } else {
                console.error('Login failed, response is undefined.')
            }
        })
}

</script>

<template>
    <div class="body-bg min-h-screen pb-6 px-2 md:px-0">
        <div class="w-full flex flex-wrap">
            <!-- Login Section -->
            <div class="w-full md:w-1/2 flex flex-col">

                <div class="flex justify-center md:justify-start pt-12 md:pl-12 md:-mb-24">
                    <a href="/" class="text-white font-bold text-xl p-4">
                        <img :src="logo" alt="Logo" class="h-16 md:h-24">
                    </a>
                </div>

                <div class="flex flex-col justify-center md:justify-start my-auto pt-8 md:pt-0 px-8 md:px-24 lg:px-32">
                    <p class="text-center text-3xl">Welcome.</p>
                    <div class="flex flex-col pt-3 md:pt-8">
                        <div class="flex flex-col pt-4">
                            <label for="email" class="text-lg">Email</label>
                            <input type="email" id="email" placeholder="your@email.com" v-model="email" class="c3k-input">

                        </div>

                        <div class="flex flex-col pt-4">
                            <label for="password" class="text-lg">Password</label>
                            <input type="password" id="password" placeholder="Password" v-model="password" class="c3k-input">
                        </div>

                        <button @click="login" class="c3k-btn">Login</button>
                    </div>
                    <div class="text-center pt-12 pb-12">
                        <p>Don't have an account?
                            <RouterLink to="/auth/register" class="underline font-semibold"> Register here.</RouterLink>
                        </p>
                    </div>
                </div>

            </div>

            <!-- Image Section -->
            <div class="w-1/2 shadow-2xl">
                <img class="object-cover w-full h-screen hidden md:block" :src="broken">
            </div>
        </div>
    </div>
</template>

<route lang="yaml">
    meta:
      layout: blank
      action: read
  </route>
