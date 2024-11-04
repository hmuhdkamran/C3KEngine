import { registerMicroApps } from 'qiankun'

// let initState = { count: 0 };

// const actions = initGlobalState(store);

// actions.onGlobalStateChange((state: any) => {
//     initState = state;
//     store.count = initState.count;
// }, true);

// const getGlobalState = () => initState;

const microApps = [{
    name: 'c3k-auth-management',
    entry: '//localhost:8291',
    activeRule: 'c3k-auth-management'
}
];

const apps = microApps.map((item) => {
    return {
        ...item,
        container: "#child-viewport",
        props: {
            routerBase: item.activeRule,
            // getGlobalState
        },
    };
});

export default apps;