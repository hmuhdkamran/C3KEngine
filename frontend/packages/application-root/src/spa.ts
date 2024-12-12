import { ref, type Ref } from 'vue';
import { TokenHelper } from 'c3k-library';
import { AuthenticationService } from './services/authentication-service';

export const microApps: Ref<Array<any>> = ref([]);
export const microAppRoutes: Ref<Array<any>> = ref([]);

const setMicroApp = (value: string) => `c3k-${value.replace('/', '-')}`;

export async function initializeMicroApps() {
    const authService = new AuthenticationService();
    
    const products = await authService.allProducts();

    microApps.value = (products as Array<any>).map((product) => ({
        name: setMicroApp(product.Abbreviation),
        entry: `//${product.FrontendIp}:${product.FrontendPort}`,
        activeRule: setMicroApp(product.Abbreviation),
        container: '#child-viewport',
        props: {
            routerBase: setMicroApp(product.Abbreviation),
            getGlobalState: () => ({
                user: TokenHelper.parseUserToken(TokenHelper.getAccessToken()),
                toggleSidebar: false
            }),
        },
    }));

    // Generate routes for micro apps
    microAppRoutes.value = microApps.value.map((app) => ({
        path: `/${app.activeRule}/:pathMatch(.*)*`,
        name: app.name,
        component: () => import('@/layouts/micro-frontend.vue'),
    }));
}