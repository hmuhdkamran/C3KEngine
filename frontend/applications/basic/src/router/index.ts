import { createRouter, createWebHistory, type RouteRecordRaw } from 'vue-router'

import { setupLayouts } from 'virtual:generated-layouts'
import generatedRoutes from 'virtual:generated-pages'
import { DefaultUser, RouteGuards, type IRouteGuardOptions } from 'c3-library'

const routeGuardOptions: IRouteGuardOptions = {
  resolveUser: () => DefaultUser,
  forbiddenRouteName: 'not-authorized',
  loginRouteName: 'account/login',
  verifyRouteName: 'access-control',
  store: null,
}

const generatedRoutesTyped = generatedRoutes as RouteRecordRaw[];
const routes = setupLayouts(generatedRoutesTyped)

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL + 'basic'),
  routes,
})

router.beforeEach(RouteGuards(routeGuardOptions))

export default router
