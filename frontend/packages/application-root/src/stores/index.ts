import type { UserProduct } from "@/models/user-products";
import { AuthenticationService } from "@/services/authentication-service";
import { DefaultUser, LocalStorageHelper, TokenHelper, type IMenuItem } from "c3k-library";
import type { MicroAppStateActions } from "qiankun";
import { reactive } from "vue";

const service: AuthenticationService = new AuthenticationService();

export const store = reactive({
    user: DefaultUser,
    userModules: [] as UserProduct[],
    sideBarMenu: [] as IMenuItem[],
    toggleSidebar: false as boolean,
    action: {} as MicroAppStateActions
});

export const updateUserWithModules = () => {
    store.user = TokenHelper.parseUserToken(TokenHelper.getAccessToken());

    var storeUserProducts = JSON.parse(LocalStorageHelper.get<string>('user-products') as string) as UserProduct[];
    if (storeUserProducts !== null) {
        store.userModules = storeUserProducts;
    } else {
        service.userProducts()?.then(result => {
            store.userModules = result as UserProduct[];
            LocalStorageHelper.set('user-products', JSON.stringify(store.userModules));
        });
    }
}