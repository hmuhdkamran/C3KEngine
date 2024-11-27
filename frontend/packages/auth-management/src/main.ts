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
    mount(props: any) {
        render(props);
    },
    bootstrap() {
    },
    update(props: any) {
        render(props)
    },
    unmount(_props: any) {
        instance.unmount();
        instance._container.innerHTML = '';
    },
});

export async function mount(props: any) {
    render(props);
}

if (!qiankunWindow.__POWERED_BY_QIANKUN__) {
    render({ container: '' });
}
