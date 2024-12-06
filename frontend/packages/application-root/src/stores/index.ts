import type { UserProduct } from "@/models/user-products";
import { AuthenticationService } from "@/services/authentication-service";
import { DefaultUser, LocalStorageHelper, TokenHelper, type IMenuItem } from "c3k-library";
import { reactive } from "vue";

const service: AuthenticationService = new AuthenticationService();

export const store = reactive({
    user: DefaultUser,
    userModules: [] as UserProduct[],
    sideBarMneu: [] as IMenuItem[],
});

export const updateUserWithModules = () => {
    store.user = TokenHelper.parseUserToken(TokenHelper.getAccessToken());

    var storeUserProducts = LocalStorageHelper.get<UserProduct[]>('user-products') as UserProduct[];
    if(storeUserProducts !== null) {
        store.userModules = storeUserProducts;
    } else {
        service.userProducts()?.then(result => {
            store.userModules = result as UserProduct[];
            LocalStorageHelper.set('user-products', JSON.stringify(store.userModules));
        });
    }    
}