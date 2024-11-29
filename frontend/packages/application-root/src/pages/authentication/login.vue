<script setup lang="ts">
import { ref, type Ref } from "vue";
import { useRoute, useRouter } from "vue-router";
import { AuthenticationService } from "@/services/authentication-service";
import { useNotification } from 'c3k-library';

import logo from "@/assets/logo.svg"

const service: AuthenticationService = new AuthenticationService();

const route = useRoute()
const router = useRouter()
const { addNotification } = useNotification();

const email: Ref<string> = ref('')
const password: Ref<string> = ref('')

const login = () => {
    const credentials = {
        username: email.value.toLowerCase(),
        password: password.value,
    }

    service.login(credentials)
        .then((response: any) => {
            if (response) {
                router.replace(route.query.to ? String(route.query.to) : '/dashboard');
            } else {
                addNotification('Login failed. Please try again.', 'error', 'top-right', 'Error', 3000);
            }
        })
        .catch((error: any) => {
            addNotification(`An error occurred during login. ${JSON.stringify(error)}.`, 'error', 'top-right', 'Error', 3000);
        });

}

</script>

<template>
    <div class="min-h-screen py-6 flex flex-col justify-center sm:py-12 relative h-full md:flex items-center p-10 
    overflow-hidden bg-violet-900 text-white bg-no-repeat bg-cover">
        <div class="absolute bg-gradient-to-b from-violet-500 to-purple-300 opacity-75 inset-0 z-0"></div>
        <ul class="circles">
            <li v-for="n in 30" :key="n"></li>
        </ul>
        <div class="relative py-3 sm:max-w-xl sm:mx-auto w-full">
            <div class="absolute inset-0 bg-gradient-to-r from-violet-900 to-purple-900 shadow-lg 
            -skew-y-6 sm:skew-y-0 sm:-rotate-6 rounded-md transition transform hover:scale-105 duration-500">
            </div>
                <div class="flex flex-col bg-white box shadow-lg p-1 rounded-md transition transform hover:scale-105 duration-500">
                    <div class="flex justify-center md:justify-start md:pl-6 md:-mb-12">
                        <a href="/" class="text-white font-bold text-xl p-2">
                            <img :src="logo" alt="Logo" class="h-8 md:h-24">
                        </a>
                    </div>
                    <div class="flex flex-col justify-center md:justify-start px-2 my-auto md:pt-0 md:px-12">
                        <p class="text-center text-2xl text-gray-800">Welcome.</p>
                        <div class="flex flex-col pt-3 md:pt-8">
                            <div class="flex flex-col pt-4">
                                <label for="email" class="text-lg text-gray-800">Email</label>
                                <input type="email" id="email" placeholder="your@email.com" v-model="email"
                                    class="input-bottom">
                            </div>

                            <div class="flex flex-col pt-4">
                                <label for="password" class="text-lg text-gray-800">Password</label>
                                <input type="password" id="password" placeholder="Password" v-model="password"
                                    class="input-bottom">
                            </div>

                            <button @click="login" class="btn-gradient mt-6">Login</button>
                        </div>

                        <div class="text-center pt-12 pb-12">
                            <p class="text-gray-800">Don't have an account?
                                <RouterLink to="/authentication/register" class="underline font-semibold text-violet-700">
                                    Register here.
                                </RouterLink>
                            </p>
                        </div>
                    </div>
                </div>
        </div>
    </div>
</template>

<style scoped lang="scss">
@use "sass:map";

@property --border-angle {
    inherits: false;
    initial-value: 0deg;
    syntax: '<angle>';
}

$circle-sizes: (
    1: (left: 25%, width: 80px, height: 80px, delay: 0s),
    2: (left: 10%, width: 20px, height: 20px, delay: 2s, duration: 12s),
    3: (left: 70%, width: 20px, height: 20px, delay: 4s),
    4: (left: 40%, width: 60px, height: 60px, delay: 0s, duration: 18s),
    5: (left: 65%, width: 20px, height: 20px, delay: 0s),
    6: (left: 75%, width: 110px, height: 110px, delay: 3s),
    7: (left: 35%, width: 150px, height: 150px, delay: 7s),
    8: (left: 50%, width: 25px, height: 25px, delay: 15s, duration: 45s),
    9: (left: 20%, width: 15px, height: 15px, delay: 2s, duration: 35s),
    10: (left: 85%, width: 150px, height: 150px, delay: 0s, duration: 11s)
);

.circles {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    overflow: hidden;

    li {
        position: absolute;
        display: block;
        list-style: none;
        width: 20px;
        height: 20px;
        background: rgba(255, 255, 255, 0.301);
        animation: animate 25s linear infinite;
        bottom: -150px;

        // Loop through $circle-sizes map to apply individual styles
        @each $index, $props in $circle-sizes {
            &:nth-child(#{$index}) {
                left: map.get($props, left);
                width: map.get($props, width);
                height: map.get($props, height);
                animation-delay: map.get($props, delay);

                // Optional duration override
                @if map.has-key($props, duration) {
                    animation-duration: map.get($props, duration);
                }
            }
        }
    }
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

.box {
  display: flex;
  padding: 12px;
  border: 3px solid #0000;
  background: linear-gradient(#ffffff, #ffffff) padding-box, linear-gradient(
        var(--angle),
        #ffffff,
        #fbff00,
        #07f362,
        #ff00f2
      ) border-box;
  animation: 6s rotate linear infinite;
}

@keyframes rotate {
  to {
    --angle: 360deg;
  }
}

@property --angle {
  syntax: "<angle>";
  initial-value: 0deg;
  inherits: false;
}

</style>

<route lang="yaml">
    meta:
      layout: blank
      action: read
  </route>
