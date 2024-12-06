import { createApp, type App } from 'vue'
import { createPinia } from 'pinia'
import {routeHash} from "@/router";

import './assets/styles/main.css';

import Root from './App.vue'
import { installPinia, useSystemStore } from 'c3k-library';

import './public-path'
import { renderWithQiankun, qiankunWindow } from 'vite-plugin-qiankun/dist/helper';

let instance: App;

const render = (props: any = {}): void => {
    const { container, setGlobalState, getGlobalState, name } = props;

    const router = routeHash(qiankunWindow.__POWERED_BY_QIANKUN__ ? `/${name}` : '/')

    instance = createApp(Root, { setGlobalState });
    instance.use(router);
    instance.use(createPinia())

    installPinia(instance);

    const store = useSystemStore();

    if (getGlobalState) {
        store.updateUser(getGlobalState().user);        
    }

    instance.mount(container ? container.querySelector('#app') : '#app');
}

renderWithQiankun({
    async bootstrap() {
        console.log('[c3k-api-auth] Bootstrap started');
        return Promise.resolve();
    },
    async mount(props) {
        console.log('[c3k-api-auth] Mounting app');
        render(props);
        return Promise.resolve();
    },
    async update(props: any) {
        console.log('[c3k-api-auth] Updating app');
        render(props);
        return Promise.resolve();
    },
    async unmount(props) {
        console.log('[c3k-api-auth] Unmounting app');
        instance.unmount();
        instance._container.innerHTML = '';
        return Promise.resolve();
    },
});

export async function mount(props: any) {
    render(props);
    return Promise.resolve();
}

if (!qiankunWindow.__POWERED_BY_QIANKUN__) {
    render({ container: '' });
}
