<script setup lang="ts">
import { onMounted, ref, type Ref } from "vue";
import { useRoute, useRouter } from "vue-router";
import { AuthenticationService } from "@/services/authentication-service";
import { useNotification } from "c3k-library";
import logo from "@/assets/logo.svg";

const service: AuthenticationService = new AuthenticationService();

const route = useRoute();
const router = useRouter();
const { addNotification } = useNotification();

const email: Ref<string> = ref("admin@sefam.com");
const password: Ref<string> = ref("P@ssw0rd");
const remember: Ref<boolean> = ref(false);

const login = () => {
    const credentials = {
        username: email.value.toLowerCase(),
        password: password.value,
    };

    service
        .login(credentials)
        .then((response: any) => {
            if (response) {
                if (remember.value) {
                    localStorage.setItem("rememberMe", "true");
                    localStorage.setItem("email", email.value);
                    localStorage.setItem("password", password.value);
                } else {
                    localStorage.removeItem("rememberMe");
                    localStorage.removeItem("email");
                    localStorage.removeItem("password");
                }
                router.replace(route.query.to ? String(route.query.to) : "/dashboard");
            } else {
                addNotification(
                    "Login failed. Please try again.",
                    "error",
                    "top-right",
                    "Error",
                    3000
                );
            }
        })
        .catch((error: any) => {
            addNotification(
                `An error occurred during login. ${JSON.stringify(error)}.`,
                "error",
                "top-right",
                "Error",
                3000
            );
        });
};

onMounted(() => {
    if (localStorage.getItem("rememberMe") === "true") {
        email.value = localStorage.getItem("email") || "";
        password.value = localStorage.getItem("password") || "";
        remember.value = true;
    }
});

</script>

<template>
    <div class="flex flex-col md:flex-row min-h-screen bg-gray-50">
        <div
            class="hidden md:flex w-1/2 bg-gradient-to-r from-indigo-500 via-blue-00 to-sky-500 text-white items-center justify-center p-10 rounded-sm shadow-2xl relative overflow-hidden">
            <div class="absolute inset-0 flex flex-col justify-center items-center pointer-events-none animate-clouds">
                <div class="cloud1 w-60 h-20 bg-white opacity-40 rounded-full blur-xl absolute bottom-0 left-0"></div>
                <div class="cloud2 w-80 h-24 bg-white opacity-40 rounded-full blur-xl absolute bottom-0 right-0"></div>
                <div class="cloud3 w-70 h-18 bg-white opacity-40 rounded-full blur-xl absolute top-10 left-20"></div>
            </div>
            <div class="absolute inset-0 flex justify-center items-center pointer-events-none animate-particles">
                <div class="particle w-2 h-2 bg-white rounded-full opacity-60 absolute animate-particle1"></div>
                <div class="particle w-3 h-3 bg-white rounded-full opacity-60 absolute animate-particle2"></div>
                <div class="particle w-1.5 h-1.5 bg-white rounded-full opacity-60 absolute animate-particle3"></div>
                <div class="particle w-2.5 h-2.5 bg-white rounded-full opacity-60 absolute animate-particle4"></div>
            </div>
            <div class="text-center space-y-8 relative z-10">
                <h1
                    class="text-5xl font-extrabold text-transparent bg-clip-text bg-gradient-to-r from-white via-white to-blue-200 animate-slide-in">
                    Welcome Back!</h1>
                <p class="text-lg opacity-90 animate-slide-in delay-100">Manage your account and enjoy seamless access
                    to all features with ease.</p>
                <div class="relative">
                    <div class="absolute inset-0 bg-white opacity-20 rounded-xl blur-lg -z-10"></div>
                    <p class="text-sm font-light opacity-80 animate-slide-in delay-200">Get back to work with just one
                        click. Let's make things happen!</p>
                </div>
            </div>
        </div>

        <div class="flex flex-col justify-center w-full max-w-lg mx-auto p-6">
            <div class="text-center mb-12">
                <img :src="logo" alt="Logo" class="h-14 mx-auto" />
                <h2 class="text-2xl font-bold mt-4 text-gray-800">Welcome Back!</h2>
                <p class="text-sm text-gray-500 mt-1">Login to access your dashboard.</p>
            </div>
            <form class="space-y-5 mb-6 text-gray-700">
                <div>
                    <label for="email" class="block text-sm font-medium text-gray-700">Email</label>
                    <div class="relative mt-1">
                        <input id="email" type="email" v-model="email" placeholder="Enter your email"
                            class="block w-full px-9 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500" />
                        <span class="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400 icon-[mdi--email-outline]"></span>
                    </div>
                </div>
                <div>
                    <label for="password" class="block text-sm font-medium text-gray-700">Password</label>
                    <div class="relative mt-1">
                        <input id="password" type="password" v-model="password" placeholder="Enter your password"
                            class="block w-full px-9 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500" />
                        <span class="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400 icon-[mdi--lock-outline]"></span>
                    </div>
                </div>

                <div class="flex justify-between items-center">
                    <div class="flex items-center space-x-2">
                        <input type="checkbox" id="remember" v-model="remember"
                            class="h-4 w-4 rounded" />
                        <label for="remember" class="text-sm text-gray-600">Remember me</label>
                    </div>
                    <a href="/forgot-password" class="text-xs text-gray-500 hover:underline">Forgot your password?</a>
                </div>

                <button @click.prevent="login"
                    class="w-full py-2.5 bg-blue-500 text-white rounded-md text-sm font-medium hover:bg-blue-600 transition">
                    Login
                </button>
            </form>
            <div class="flex items-center justify-center my-6">
                <div class="h-px bg-gray-300 w-1/4"></div>
                <p class="mx-3 text-sm text-gray-500">or continue with</p>
                <div class="h-px bg-gray-300 w-1/4"></div>
            </div>

            <div class="flex justify-center space-x-6">
                <a href="#google" class="hover:text-red-500">
                    <span class="icon-[devicon--google] w-5 h-5"></span>
                </a>
                <a href="#facebook" class="hover:text-blue-600">
                    <span class="icon-[logos--facebook] w-5 h-5"></span>
                </a>
                <a href="#twitter" class=" hover:text-blue-400">
                    <span class="icon-[logos--twitter] w-5 h-5"></span>
                </a>
            </div>

            <div class="text-center mt-6 text-sm text-gray-600">
                <p>
                    Don’t have an account?
                    <RouterLink to="/auth/register" class="text-blue-400 font-medium hover:underline">
                        Sign up
                    </RouterLink>
                </p>
            </div>

            <div class="text-center mt-6 text-xs text-gray-500">
                <p>
                    By logging in, you agree to our
                    <a href="/terms" class="text-blue-400 hover:underline">Terms</a> and
                    <a href="/privacy" class="text-blue-400 hover:underline">Privacy Policy</a>.
                </p>
            </div>
        </div>
    </div>
</template>

<style scoped>
@keyframes clouds {
    0% {
        transform: translateX(-50%);
    }

    100% {
        transform: translateX(100%);
    }
}

.animate-clouds {
    animation: clouds 40s linear infinite;
}

.cloud1 {
    animation-duration: 60s;
}

.cloud2 {
    animation-duration: 90s;
}

.cloud3 {
    animation-duration: 120s;
}

@keyframes particle1 {
    0% {
        transform: translateX(0) translateY(0);
        opacity: 0.5;
    }

    50% {
        transform: translateX(300px) translateY(200px);
        opacity: 1;
    }

    100% {
        transform: translateX(600px) translateY(0);
        opacity: 0.5;
    }
}

@keyframes particle2 {
    0% {
        transform: translateX(0) translateY(0);
        opacity: 0.5;
    }

    50% {
        transform: translateX(-200px) translateY(300px);
        opacity: 1;
    }

    100% {
        transform: translateX(200px) translateY(400px);
        opacity: 0.5;
    }
}

@keyframes particle3 {
    0% {
        transform: translateX(0) translateY(0);
        opacity: 0.5;
    }

    50% {
        transform: translateX(150px) translateY(-250px);
        opacity: 1;
    }

    100% {
        transform: translateX(-150px) translateY(200px);
        opacity: 0.5;
    }
}

@keyframes particle4 {
    0% {
        transform: translateX(0) translateY(0);
        opacity: 0.5;
    }

    50% {
        transform: translateX(-400px) translateY(100px);
        opacity: 1;
    }

    100% {
        transform: translateX(300px) translateY(300px);
        opacity: 0.5;
    }
}

.animate-particles {
    animation: particle1 5s ease-in-out infinite, particle2 6s ease-in-out infinite, particle3 8s ease-in-out infinite, particle4 7s ease-in-out infinite;
}

@keyframes slide-in {
    0% {
        opacity: 0;
        transform: translateY(30px);
    }

    100% {
        opacity: 1;
        transform: translateY(0);
    }
}

.animate-slide-in {
    animation: slide-in 1s ease-out forwards;
}

.delay-100 {
    animation-delay: 0.1s;
}

.delay-200 {
    animation-delay: 0.2s;
}
</style>
