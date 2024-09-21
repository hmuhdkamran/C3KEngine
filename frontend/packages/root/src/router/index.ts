import { setupLayouts } from 'virtual:generated-layouts'
import { createRouter, createWebHistory } from 'vue-router'
import routes from '~pages'

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
      // ℹ️ We are redirecting to different pages based on role.
      // NOTE: Role is just for UI purposes. ACL is based on abilities.
      ...setupLayouts(routes),
    ],
  });

  export default router;