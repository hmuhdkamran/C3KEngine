import { registerMicroApps } from 'qiankun'

// let initState = { count: 0 };

// const actions = initGlobalState(store);

// actions.onGlobalStateChange((state: any) => {
//     initState = state;
//     store.count = initState.count;
// }, true);

// const getGlobalState = () => initState;

const microApps = [{
    name: 'c3k-user-management',
    entry: '//localhost:5002',
    activeRule: 'c3k-user-management'
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