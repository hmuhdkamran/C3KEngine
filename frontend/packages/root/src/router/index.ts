import { setupLayouts } from 'virtual:generated-layouts';
import { createRouter, createWebHistory } from 'vue-router';
import routes from '~pages';
import { DefaultUser, RouteGuards } from 'c3k-library';

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
    {
      path: '/c3k-user-management/:pathMatch(.*)*',
      name: 'c3k-user-management',
      component: () => import('@/pages/app/micro.vue'),
    },
  ],
});

router.beforeEach(RouteGuards(options));

export default router;
