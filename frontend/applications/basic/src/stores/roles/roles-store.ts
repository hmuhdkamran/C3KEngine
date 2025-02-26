import type { IRole } from '@/models'
import { createGenericPiniaStore } from 'c3-library'
import { acceptHMRUpdate } from 'pinia'

export const useRoleRolesStore = createGenericPiniaStore<IRole, 'RoleId'>(
  'RoleRoles',
  'auth/role/roles',
  'RoleId',
)

if (import.meta.hot) import.meta.hot.accept(acceptHMRUpdate(useRoleRolesStore, import.meta.hot))
