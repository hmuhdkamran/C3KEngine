<script setup lang="ts">
import { ref, Ref } from "vue";
import { useRoute, useRouter } from "vue-router";
import { AuthenticationService } from "@/service/auth/authentication-service";

import logo from "@/assets/images/vue.svg"

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
    <div class="flex min-h-screen">
        <div
            class="relative w-1/2 bg-gradient-to-b from-violet-700 to-violet-600 flex flex-col justify-center items-center group overflow-hidden">

            <div
                class="bg-purple-300/20 absolute -start-6 -top-6 h-14 w-0 origin-top-left rotate-45 rounded-full transition-all delay-[25ms] duration-300 group-hover:w-72">
            </div>
            <div
                class="bg-purple-300/20 absolute -top-12 start-20 h-14 w-0 origin-top-left rotate-45 rounded-full transition-all delay-75 duration-300 group-hover:w-48">
            </div>
            <div
                class="bg-purple-300/20 absolute -start-7 top-24 h-14 w-0 origin-top-left rotate-45 rounded-full transition-all delay-150 duration-300 group-hover:w-40">
            </div>
            <div
                class="bg-purple-300/20 absolute -bottom-6 -end-6 h-14 w-0 origin-bottom-right rotate-45 rounded-full transition-all delay-150 duration-300 group-hover:w-72">
            </div>
            <div
                class="bg-purple-300/20 absolute -bottom-12 end-20 h-14 w-0 origin-bottom-right rotate-45 rounded-full transition-all delay-75 duration-300 group-hover:w-48">
            </div>
            <div
                class="bg-purple-300/20 absolute -end-7 bottom-24 h-14 w-0 origin-bottom-right rotate-45 rounded-full transition-all delay-[25ms] duration-300 group-hover:w-40">
            </div>
            <div class="mx-auto max-w-xs text-center">
                <h2 class="text-4xl text-white mb-3"> Have an
                    Account?</h2>
                <p class="text-md text-white mb-3">
                    No need to waste time on this page, let's take you back to your account</p>
                <button class="px-6 py-2 bg-white rounded-full w-full">Login to Account</button>
            </div>
        </div>

        <div class="w-full md:w-1/2 flex flex-col bg-white">
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
                        <input type="password" id="password" placeholder="Password" v-model="password"
                            class="c3k-input">
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
    </div>
</template>
<route lang="yaml">
    meta:
      layout: blank
      action: read
  </route>
