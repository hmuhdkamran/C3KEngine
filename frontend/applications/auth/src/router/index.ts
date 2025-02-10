import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router'
import { setupLayouts } from 'virtual:generated-layouts'
import generatedRoutes from 'virtual:generated-pages'
import i18n from '~/modules/i18n'

import { DefaultUser, IRouteGuardOptions, RouteGuards } from 'c3-library'

const generatedRoutesTyped = generatedRoutes as RouteRecordRaw[];

const routeGuardOptions: IRouteGuardOptions = {
  resolveUser: () => DefaultUser,
  forbiddenRouteName: 'not-authorized',
  loginRouteName: 'login',
  verifyRouteName: 'access-control',
  store: null,
}

const routes = setupLayouts(generatedRoutesTyped)

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
})

router.beforeEach((to, from, next) => {
  const { t } = i18n.global
  const baseTitle = t('title')
  const routeTitle = to.meta.title ? t(`menu.${to.meta.title}`) : null; // Avoid empty string

  document.title = routeTitle ? `${routeTitle} - ${baseTitle}` : baseTitle;
  next()
})

// router.beforeEach(RouteGuards(routeGuardOptions))

export default router
