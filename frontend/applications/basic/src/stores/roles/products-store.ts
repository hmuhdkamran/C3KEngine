import type { IProducts } from '@/models'
import { createGenericPiniaStore } from 'c3-library'
import { acceptHMRUpdate } from 'pinia'

export const useRoleProductsStore = createGenericPiniaStore<IProducts, 'ProductId'>(
  'RoleProducts',
  'auth/role/products',
  'ProductId',
)

if (import.meta.hot) import.meta.hot.accept(acceptHMRUpdate(useRoleProductsStore, import.meta.hot))
