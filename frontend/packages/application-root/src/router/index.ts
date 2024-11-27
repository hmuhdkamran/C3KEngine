import { setupLayouts } from 'virtual:generated-layouts';
import { createRouter, createWebHistory } from 'vue-router';
import routes from '~pages';
import { DefaultUser, RouteGuards } from 'c3k-library';
import { initializeMicroApps, microAppRoutes } from '@/spa';

const options = {
  resolveUser: () => DefaultUser,
  forbiddenRouteName: 'not-authorized',
  loginRouteName: 'login',
  verifyRouteName: 'access-control',
  store: null,
};

await initializeMicroApps();

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    ...setupLayouts(routes),
    ...microAppRoutes.value
  ],
});

router.beforeEach(RouteGuards(options));

export default router;
