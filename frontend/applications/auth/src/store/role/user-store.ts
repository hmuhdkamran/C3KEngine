import { acceptHMRUpdate } from 'pinia'
import { createGenericPiniaStore } from 'c3-library';
import { IUser } from '~/models/roles/IUser';

export const useRoleUserStore = createGenericPiniaStore<IUser, 'UserId'>('RoleUser', 'auth/role/users', 'UserId');

if (import.meta.hot)
    import.meta.hot.accept(acceptHMRUpdate(useRoleUserStore, import.meta.hot))