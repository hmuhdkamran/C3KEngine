<script setup lang="ts">
import logo from "@/assets/images/vue.svg"
</script>

<template>
    <div class="flex min-h-screen">
        <div
            class="relative w-1/2 flex flex-col sm:flex-row items-center md:items-start sm:justify-center md:justify-start flex-auto min-w-0 bg-white">
            <div
                class="relative w-1/2 h-full hidden md:flex flex-auto items-center justify-center p-10 overflow-hidden bg-violet-900 text-white bg-no-repeat bg-cover relative">
                <div class="absolute bg-gradient-to-b from-violet-600 to-violet-500 opacity-75 inset-0 z-0"></div>
                <div class="w-full  max-w-md z-10">
                    <div class="sm:text-4xl xl:text-5xl font-bold leading-tight mb-6">Get Started with Our HRMS Today
                    </div>
                    <div class="sm:text-sm xl:text-md text-gray-200 font-normal"> Our HRMS software offers comprehensive
                        solutions for managing your HR needs.
                        Join us today and streamline your HR processes with ease and efficiency.</div>
                </div>
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
            </div>

            <div class="w-full md:w-1/2 flex flex-col bg-white">

                <div class="flex justify-center md:justify-start pt-12 md:pl-12 md:-mb-3">
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

                        <input type="submit" value="Register" class="c3k-btn" />
                    </form>
                    <div class="text-center pt-12 pb-12">
                        <p>Already have an account?
                            <RouterLink to="/auth/login" class="underline font-semibold"> Log in here.</RouterLink>
                        </p>
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
    background: rgba(255, 255, 255, 0.2);
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
