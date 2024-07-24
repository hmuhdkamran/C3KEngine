<script setup lang="ts">
import logo from "@/assets/vue.svg"
import broken from "@/assets/login.svg"
</script>

<template>
    <div class="body-bg min-h-screen pb-6 px-2 md:px-0">
        <div class="w-full flex flex-wrap">

            <!-- Register Section -->
            <div class="w-full md:w-1/2 flex flex-col">

                <div class="flex justify-center md:justify-start pt-12 md:pl-12 md:-mb-12">
                    <a href="/" class="text-white font-bold text-xl p-4" alt="Logo">
                        <img :src="logo" alt="Logo" class="h-16 md:h-24">
                    </a>
                </div>

                <div class="flex flex-col justify-center md:justify-start my-auto pt-8 md:pt-0 px-8 md:px-24 lg:px-32">
                    <p class="text-center text-3xl">Join Us.</p>
                    <form class="flex flex-col pt-3 md:pt-8">
                        <div class="flex flex-col pt-4">
                            <label for="name" class="text-lg">Name</label>
                            <input type="text" id="name" placeholder="John Smith" class="c3k-input" />
                        </div>

                        <div class="flex flex-col pt-4">
                            <label for="email" class="text-lg">Email</label>
                            <input type="email" id="email" placeholder="your@email.com" class="c3k-input" />
                        </div>

                        <div class="flex flex-col pt-4">
                            <label for="password" class="text-lg">Password</label>
                            <input type="password" id="password" placeholder="Password" class="c3k-input" />
                        </div>

                        <div class="flex flex-col pt-4">
                            <label for="confirm-password" class="text-lg">Confirm Password</label>
                            <input type="password" id="confirm-password" placeholder="Password" class="c3k-input" />
                        </div>

                        <input type="submit" value="Register"
                            class="c3k-btn" />
                    </form>
                    <div class="text-center pt-12 pb-12">
                        <p>Already have an account?
                            <RouterLink to="/auth/login" class="underline font-semibold"> Log in here.</RouterLink>
                        </p>
                    </div>
                </div>

            </div>

            <!-- Image Section -->
            <div class="w-1/2 shadow-2xl">
                <img class="object-cover w-full h-screen hidden md:block" :src="broken" alt="Background" />
            </div>
        </div>
    </div>
</template>

<route lang="yaml">
    meta:
      layout: blank
      action: read
  </route>
