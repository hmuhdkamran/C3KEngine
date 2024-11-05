import { setupLayouts } from 'virtual:generated-layouts';
import { createRouter, createWebHistory } from 'vue-router';
import routes from '~pages';
import { DefaultUser, RouteGuards } from 'c3k-library';
// import { microAppRoutes } from '@/spa';

const options = {
  resolveUser: () => DefaultUser,
  forbiddenRouteName: 'not-authorized',
  loginRouteName: 'login',
  verifyRouteName: 'access-control',
  store: null,
};

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    ...setupLayouts(routes),
    // ...microAppRoutes
  ],
});

router.beforeEach(RouteGuards(options));

export default router;
