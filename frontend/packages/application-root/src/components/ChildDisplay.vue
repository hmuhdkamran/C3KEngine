<script setup lang="ts">
import { registerMicroApps, start, setDefaultMountApp, initGlobalState } from 'qiankun';
import { onMounted, ref, type Ref } from 'vue';
import { LocalStorageHelper, TokenHelper } from 'c3k-library';
import { store } from '@/stores';
import { AuthenticationService } from '@/services/authentication-service';

// Initialize authentication service
const repo = new AuthenticationService();

// Initialize global state with user data from the token
let initState = { user: TokenHelper.parseUserToken(TokenHelper.getAccessToken()) };

// Create actions for global state communication
const actions = initGlobalState(store);

// Listen for global state changes and update local store
actions.onGlobalStateChange((state: any) => {
    initState = state;
    store.user = initState.user;
}, true);

// Function to get the current global state
const getGlobalState = () => initState;

// MicroApps management with reactive ref
const microApps: Ref<Array<any>> = ref([]);

// Utility function to generate micro app names and active rules
const setMicroApp = (value: string) => `c3k-${value.replace('/', '-')}`;

// Vue onMounted lifecycle to initialize micro apps
onMounted(() => {
    // Fetch the user products and register corresponding micro apps
    repo.userProducts()?.then(products => {
        (products as Array<any>).forEach(element => {
            microApps.value.push({
                name: setMicroApp(element.Abbreviation), // Unique name for each micro app
                entry: `//${element.FrontendIp}:${element.FrontendPort}`, // Micro app entry point
                activeRule: setMicroApp(element.Abbreviation), // Active rule for routing
                container: "#child-viewport", // Target container in the main app
                props: {
                    routerBase: setMicroApp(element.Abbreviation), // Base route for the micro app
                    getGlobalState, // Function to share global state
                },
            });
        });

        // Register the dynamically fetched micro apps
        registerMicroApps(microApps.value, {
            beforeLoad: async (app) => console.log('%c[before load]', 'color: red', app.name),
            beforeMount: async (app) => console.log('%c[before mount]', 'color: blue', app.name),
            afterMount: async (app) => console.log('%c[after mount]', 'color: green', app.name),
            afterUnmount: async (app) => console.log('%c[after unmount]', 'color: gray', app.name),
        });

        // Start the Qiankun application
        start();
    });

    // Set the default application to mount
    const application = LocalStorageHelper.get<string>("application");
    if (application) {
        setDefaultMountApp(`/${application}`);
    }
});
</script>

<template>
    <!-- Container for child micro apps -->
    <div id="child-viewport"></div>
</template>

<style scoped>
/* Scoped styles for this component */
</style>
