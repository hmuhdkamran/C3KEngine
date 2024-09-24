import type { IUser } from '@/plugins/models'

export interface IRouteGuardOptions {
  resolveUser(): IUser | undefined
  forbiddenRouteName: string
  loginRouteName: string
  verifyRouteName: string
  store: any
}
