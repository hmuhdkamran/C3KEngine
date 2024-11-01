import { createApp, App } from 'vue'
import { createPinia } from 'pinia'
import { routeHash } from "@/router";

import '@/assets/styles/main.css';
import 'c3k-library/style.css';

import './public-path'
import { renderWithQiankun, qiankunWindow } from 'vite-plugin-qiankun/dist/helper';

import Root from './App.vue'
import { installPinia } from 'c3k-library';

let instance: App;

const render = (props: any = {}): void => {
    const { container, setGlobalState, getGlobalState, name } = props;
    if (getGlobalState) {
        // store.count = getGlobalState().count;
    }
    
    const router = routeHash(qiankunWindow.__POWERED_BY_QIANKUN__ ? `/${name}` : '/')

    instance = createApp(Root, { setGlobalState });
    instance.use(router);
    instance.use(createPinia())

    installPinia(instance);

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
