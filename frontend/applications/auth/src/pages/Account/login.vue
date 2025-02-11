<script setup lang="ts">
import type { FormInst, FormRules } from 'naive-ui/es/form/src/interface'
import { storeToRefs } from 'pinia'

import type { LoginViewModel } from '~/models/Account'

import logo from "@/assets/images/logo.svg";

const { t } = useI18n()

const accountStore = useAccountStore()
const { isLoading } = storeToRefs(accountStore)
const loginInfo = ref<LoginViewModel>({ username: 'admin@sefam.com', password: 'P@ssw0rd' })
const loginFailed = ref(false)
const router = useRouter()
const formRef = ref<FormInst | null>(null)
async function login() {
  formRef.value?.validate(async (errors: any) => {
    if (!errors) {
      const loginSucceed = await accountStore.login(loginInfo.value)
      if (loginSucceed) {
        useNotifyStore().success(t('login.successMessage'))
        setTimeout(() => router.push('/dashboard'), 500)
      }
      else {
        useNotifyStore().error(t('login.failedMessage'))
        loginFailed.value = true
        setTimeout(() => {
          loginFailed.value = false
        }, 2000)
      }
    }
  })
}

const rules: FormRules = {
  username: [
    {
      required: true,
      trigger: ['blur', 'change'],
      message: t('login.validations.userNameRequired'),
    },
  ],
  password: [
    {
      required: true,
      trigger: ['blur', 'change'],
      message: t('login.validations.passwordRequired'),
    },
  ],
}
</script>

<route lang="yaml">
meta:
  title: login
  layout: blank
  authRequired: false
</route>

<template>
  <div class="flex flex-col md:flex-row min-h-screen bg-gray-50 dark:bg-slate-800">
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
          {{ t('login.welcome') }}</h1>
        <p class="text-lg opacity-90 animate-slide-in delay-100">{{ t('generic.tagline1') }}</p>
        <div class="relative">
          <div class="absolute inset-0 bg-white opacity-20 rounded-xl blur-lg -z-10"></div>
          <p class="text-sm font-light opacity-80 animate-slide-in delay-200">{{ t('generic.tagline2') }}</p>
        </div>
      </div>
    </div>

    <div class="flex flex-col justify-center w-full max-w-lg mx-auto p-6">
      <div class="mt-3 flex justify-between items-center">
        <LanguageSelect />
        <ThemeSwitch class="mr-2" />
      </div>
      <div class="text-center mb-12">
        <img :src="logo" alt="Logo" class="h-14 mx-auto" />
        <h2 class="text-2xl font-bold mt-4 text-gray-800">{{ t('login.welcome') }}</h2>
        <p class="text-sm text-gray-500 mt-1">{{ t('login.message') }}</p>
      </div>
      <n-form ref="formRef" :model="loginInfo" :rules="rules" @submit.prevent="login()">
        <n-form-item class="mb-1" path="username" :label="t('login.username')">
          <n-input id="name" v-model:value="loginInfo.username" autofocus :placeholder="t('login.username')" />
        </n-form-item>
        <n-form-item class="mb-1" path="password" :label="t('login.password')">
          <n-input id="name" v-model:value="loginInfo.password" type="password" show-password-on="mousedown"
            :placeholder="t('login.password')" />
        </n-form-item>

        <div class="flex align-items-center justify-between mb-2">
          <RouterLink to="/Account/ForgotPassword" class="no-underline ml-2 text-blue-500 text-right cursor-pointer">
            {{ t('login.forgetPassword') }}
          </RouterLink>
        </div>
        <n-button attr-type="submit" size="large" :block="true" type="primary" :loading="isLoading">
          {{ t('login.loginButton') }}
        </n-button>
      </n-form>
      <div class="flex items-center justify-center my-6">
        <div class="h-px bg-gray-300 w-1/4"></div>
        <p class="mx-3 text-sm text-gray-500">or continue with</p>
        <div class="h-px bg-gray-300 w-1/4"></div>
      </div>

      <div class="flex justify-center space-x-6 text-blue-500">
        <a href="#google" class="hover:text-sky-500">
          <span class="fa-brands fa-google"></span>
        </a>
        <a href="#facebook" class="hover:text-sky-500">
          <span class="fa-brands fa-facebook"></span>
        </a>
        <a href="#twitter" class=" hover:text-sky-500">
          <span class="fa-brands fa-twitter"></span>
        </a>
      </div>

      <div class="text-center mt-6 text-sm text-gray-600">
        <p>
          {{ t('login.haveNotAccount') }}
          <RouterLink to="/account/register" class="text-blue-400 font-medium hover:underline">
            {{ t('login.createAccount') }}
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

<style lang='scss'>
.banner {
  background-image: url('~/assets/images/login_banner.jpg');
  background-size: cover;
  background-position: center center;
  height: 150px;
  border-radius: 4px 4px 0 0;
}

.login-box {
  max-width: 380px;

  .failed {
    animation: shake 0.82s cubic-bezier(0.36, 0.07, 0.19, 0.97) both;
    transform: translate3d(0, 0, 0);
    backface-visibility: hidden;
    perspective: 1000px;
  }
}

input:-webkit-autofill,
input:-webkit-autofill:hover,
input:-webkit-autofill:focus,
textarea:-webkit-autofill,
textarea:-webkit-autofill:hover,
textarea:-webkit-autofill:focus,
select:-webkit-autofill,
select:-webkit-autofill:hover,
select:-webkit-autofill:focus {
  -webkit-text-fill-color: #000;
  -webkit-box-shadow: 0 0 0 1000px #eff0f1 inset;
  transition: background-color 5000s ease-in-out 0s;
}

@keyframes shake {

  10%,
  90% {
    transform: translate3d(-1px, 0, 0);
  }

  20%,
  80% {
    transform: translate3d(2px, 0, 0);
  }

  30%,
  50%,
  70% {
    transform: translate3d(-4px, 0, 0);
  }

  40%,
  60% {
    transform: translate3d(4px, 0, 0);
  }
}

.separator {
  border-bottom: solid 1px #ececec;
  margin: 1rem 0;

  .title {
    margin-top: -10px;
    color: var(--border);
    font-size: 0.8rem;
    position: absolute;
    left: calc(50% - 10px);
    padding: 0 0.3rem;
  }
}

// Variables
$cloud-base-duration: 40s;
$cloud-stagger-duration: 30s;
$base-cloud-animation-duration: 60s;
$slide-in-duration: 1s;

// Keyframes
@keyframes clouds {
  0% {
    transform: translateX(-50%);
  }

  100% {
    transform: translateX(100%);
  }
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

// Cloud animations
.animate-clouds {
  animation: clouds $cloud-base-duration linear infinite;
}

@for $i from 1 through 3 {
  .cloud#{$i} {
    animation-duration: $base-cloud-animation-duration + ($cloud-stagger-duration * ($i - 1));
  }
}

@mixin multiple-animations($d1, $d2, $d3, $d4) {
  animation:
    particle1 $d1 ease-in-out infinite,
    particle2 $d2 ease-in-out infinite,
    particle3 $d3 ease-in-out infinite,
    particle4 $d4 ease-in-out infinite;
}

// Particle animations
.animate-particles {
  @include multiple-animations(5s, 6s, 8s, 7s);
}

// Slide-in animation
.animate-slide-in {
  animation: slide-in $slide-in-duration ease-out forwards;
}

// Delay classes
@for $i from 1 through 2 {
  .delay-#{$i}00 {
    animation-delay: #{$i / 10}s;
    /* 0.1s, 0.2s, etc. */
  }
}

// Helper mixin for multiple animations (optional)
@mixin multiple-animations($d1: 0s, $d2: 0s, $d3: 0s, $d4: 0s) {
  @content;
}
</style>
