import { createRouter, createWebHistory } from 'vue-router'

import DefaultLayout from '@/components/layouts/default/index.vue'
import BlankLayout from '@/components/layouts/blank/index.vue'

import LandingPage from '@/views/LandingPage.vue'
import LoginPage from '@/views/authentication/login.vue'
import RegisterPage from '@/views/authentication/register.vue'
import Dashboard from '@/views/dashboard/index.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: DefaultLayout,
      children: [
        {
          path: '',
          name: 'landing',
          component: LandingPage,
        },
      ],
    },
    {
      path: '/auth',
      name: 'auth',
      component: BlankLayout,
      children: [
        {
          path: '',
          name: 'login',
          component: LoginPage,
        },
        {
          path: 'register',
          name: 'register',
          component: RegisterPage,
        },
      ],
    },
    {
      path: '/dashboard',
      name: 'dashboard',
      component: Dashboard,
    },
  ],
})

export default router
