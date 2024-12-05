import { DefaultUser, type IMenuItem } from "c3k-library";
import { reactive } from "vue";

export const store = reactive({
    user: DefaultUser,
    userModules: [],
    sideBarMneu: [] as IMenuItem[],
});