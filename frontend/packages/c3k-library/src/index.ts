import { createPinia } from 'pinia';
import { App } from 'vue';

export const installPinia = (app: App) => {
    if (!app._context.provides.pinia) {
        const pinia = createPinia();
        app.use(pinia);
    }
};

import './assets/styles/main.css';

export * from './plugins';

export { default as DefaultHeader } from './components/layouts/default/header.vue';
export { default as DefaultFooter } from './components/layouts/default/footer.vue';

export { default as Filter } from './components/layouts/gernal/filter.vue';

export { default as DataTable } from './components/data/DataTable.vue';
export { default as Pagination } from './components/data/Pagination.vue';

export { default as BaseIcon } from './components/utilities/BaseIcon.vue';
export { default as Card } from './components/utilities/Card.vue';

export { default as Notification } from '../../root/src/components/Notification.vue';
export * from "../../root/src/components/useNotification";