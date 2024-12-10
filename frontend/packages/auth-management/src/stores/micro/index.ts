import { DefaultUser, type IMenuItem } from "c3k-library";
import { reactive } from "vue";

export const store = reactive({
    user: DefaultUser,
    userModules: [],
    sideBarMenu: [] as IMenuItem[],
    toggleSidebar: false as boolean,
    userClaims: [] as any[]
});