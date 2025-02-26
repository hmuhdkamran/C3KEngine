import type { IUserProductMaps } from '@/models'
import { createGenericPiniaStore } from 'c3-library'
import { acceptHMRUpdate } from 'pinia'

export const useRoleUserProductMapsStore = createGenericPiniaStore<
  IUserProductMaps,
  'UserProductMapId'
>('RoleUserProductMaps', 'auth/role/user_product_maps', 'UserProductMapId')

if (import.meta.hot)
  import.meta.hot.accept(acceptHMRUpdate(useRoleUserProductMapsStore, import.meta.hot))
