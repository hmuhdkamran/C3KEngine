import type { IUserRoleMap } from '@/models'
import { createGenericPiniaStore } from 'c3-library'
import { acceptHMRUpdate } from 'pinia'

export const useRoleUserRoleMapStore = createGenericPiniaStore<IUserRoleMap, 'UserRoleMapId'>(
  'RoleUserRoleMaps',
  'auth/role/user_role_maps',
  'UserRoleMapId',
)

if (import.meta.hot)
  import.meta.hot.accept(acceptHMRUpdate(useRoleUserRoleMapStore, import.meta.hot))
