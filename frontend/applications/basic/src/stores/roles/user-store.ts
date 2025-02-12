import type { IUser } from "@/models";
import { createGenericPiniaStore } from "c3-library";
import { acceptHMRUpdate } from "pinia";

export const useRoleUserStore = createGenericPiniaStore<IUser, 'UserId'>('RoleUser', 'auth/role/users', 'UserId');

if (import.meta.hot)
    import.meta.hot.accept(acceptHMRUpdate(useRoleUserStore, import.meta.hot))