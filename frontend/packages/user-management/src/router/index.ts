import { setupLayouts } from 'virtual:generated-layouts'
import { createRouter, createWebHistory } from 'vue-router'
import routes from '~pages'
import { DefaultUser, RouteGuards } from 'c3k-library'

const options = {
  resolveUser: () => DefaultUser, // store.user,
  forbiddenRouteName: 'not-authorized',
  loginRouteName: 'login',
  verifyRouteName: 'access-control',
  store: null,
}

export const routeHash = (hash: string) => {
  const router = createRouter({
    history: createWebHistory(hash),
    routes: [
      ...setupLayouts(routes),
    ],
  });

  router.beforeEach(RouteGuards(options));

  return router;
}