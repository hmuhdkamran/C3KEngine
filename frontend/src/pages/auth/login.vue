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
                router.replace(route.query.to ? String(route.query.to) : '/app/main')
            } else {
                console.error('Login failed, response is undefined.')
            }
        })
}

</script>

<template>
    <!-- <div class="flex min-h-screen">
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
    </div> -->
    
    <div class="min-h-screen bg-gray-100 py-6 flex flex-col justify-center sm:py-12 relative h-full md:flex items-center p-10 
    overflow-hidden bg-violet-900 text-white bg-no-repeat bg-cover">
        <div class="absolute bg-gradient-to-b from-violet-500 to-purple-300 opacity-75 inset-0 z-0"></div>
        <ul class="circles">
            <li></li>
            <li></li>
            <li></li>
            <li></li>
            <li></li>
            <li></li>
            <li></li>
            <li></li>
            <li></li>
            <li></li>
        </ul>
        <div class="relative py-3 sm:max-w-xl sm:mx-auto w-full">
            <div class="absolute inset-0 bg-gradient-to-r from-violet-900 to-purple-900 shadow-lg transform 
            -skew-y-6 sm:skew-y-0 sm:-rotate-6 sm:rounded-3xl transition transform hover:scale-105 duration-500">
            </div>
            <div
                class="relative bg-white shadow-lg sm:rounded-3xl sm:p-5 transition transform hover:scale-105 duration-700">
                <div class="flex flex-col bg-white">
                    <div class="flex justify-center md:justify-start md:pl-6 md:-mb-12">
                        <a href="/" class="text-white font-bold text-xl p-2">
                            <img :src="logo" alt="Logo" class="h-8 md:h-24">
                        </a>
                    </div>
                    <div class="fflex flex-col justify-center md:justify-start px-2 my-auto md:pt-0 md:px-12">
                        <p class="text-center text-2xl text-gray-800">Welcome.</p>
                        <div class="flex flex-col pt-3 md:pt-8">
                            <div class="flex flex-col pt-4">
                                <label for="email" class="text-lg text-gray-800">Email</label>
                                <input type="email" id="email" placeholder="your@email.com" v-model="email"
                                    class="c3k-input">
                            </div>

                            <div class="flex flex-col pt-4">
                                <label for="password" class="text-lg text-gray-800">Password</label>
                                <input type="password" id="password" placeholder="Password" v-model="password"
                                    class="c3k-input">
                            </div>

                            <button @click="login" class="c3k-btn mt-6">Login</button>
                        </div>

                        <div class="text-center pt-12 pb-12">
                            <p class="text-gray-800">Don't have an account?
                                <a href="/auth/register" class="underline font-semibold text-violet-700">
                                    Register here.</a>
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
.circles {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    overflow: hidden;
}

.circles li {
    position: absolute;
    display: block;
    list-style: none;
    width: 20px;
    height: 20px;
    background: rgba(255, 255, 255, 0.301);
    animation: animate 25s linear infinite;
    bottom: -150px;
}

.circles li:nth-child(1) {
    left: 25%;
    width: 80px;
    height: 80px;
    animation-delay: 0s;
}

.circles li:nth-child(2) {
    left: 10%;
    width: 20px;
    height: 20px;
    animation-delay: 2s;
    animation-duration: 12s;
}

.circles li:nth-child(3) {
    left: 70%;
    width: 20px;
    height: 20px;
    animation-delay: 4s;
}

.circles li:nth-child(4) {
    left: 40%;
    width: 60px;
    height: 60px;
    animation-delay: 0s;
    animation-duration: 18s;
}

.circles li:nth-child(5) {
    left: 65%;
    width: 20px;
    height: 20px;
    animation-delay: 0s;
}

.circles li:nth-child(6) {
    left: 75%;
    width: 110px;
    height: 110px;
    animation-delay: 3s;
}

.circles li:nth-child(7) {
    left: 35%;
    width: 150px;
    height: 150px;
    animation-delay: 7s;
}

.circles li:nth-child(8) {
    left: 50%;
    width: 25px;
    height: 25px;
    animation-delay: 15s;
    animation-duration: 45s;
}

.circles li:nth-child(9) {
    left: 20%;
    width: 15px;
    height: 15px;
    animation-delay: 2s;
    animation-duration: 35s;
}

.circles li:nth-child(10) {
    left: 85%;
    width: 150px;
    height: 150px;
    animation-delay: 0s;
    animation-duration: 11s;
}

@keyframes animate {
    0% {
        transform: translateY(0) rotate(0deg);
        opacity: 1;
        border-radius: 0;
    }

    100% {
        transform: translateY(-1000px) rotate(720deg);
        opacity: 0;
        border-radius: 50%;
    }
}
</style>

<route lang="yaml">
    meta:
      layout: blank
      action: read
  </route>
