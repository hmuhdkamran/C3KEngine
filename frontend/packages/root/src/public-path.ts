import { qiankunWindow } from 'vite-plugin-qiankun/dist/helper';

if (qiankunWindow.__POWERED_BY_QIANKUN__) {
    console.log(qiankunWindow.__INJECTED_PUBLIC_PATH_BY_QIANKUN__, 'If sub-application is used then go here')
}