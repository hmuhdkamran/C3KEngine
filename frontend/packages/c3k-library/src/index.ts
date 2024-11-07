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
export { default as Card } from './components/utilities/cardstyle.vue';

export { default as Notification } from './components/utilities/Notification.vue';
export * from "./components/utilities/useNotification";

export { default as ConfirmationDialog } from './components/common/ConfirmationDialog.vue';
export { default as Drawer } from './components/common/Drawer.vue';

export { default as Button } from './components/common/Button.vue';
export { default as ComboBox } from './components/form/ComboBox.vue';
export { default as TextInput } from './components/form/TextInput.vue';
// export { default as DatePicker } from './components/common/DatePicker.vue';

export { VNodeRenderer } from './components/extra/VNodeRenderer';
export { default as TokenStatus } from './components/extra/TokenStatus.vue';
export { default as SysLoader } from './components/extra/SysLoader.vue'
