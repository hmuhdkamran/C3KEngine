import type { IUser } from '../models'

export interface IRouteGuardOptions {
  resolveUser(): IUser | undefined
  forbiddenRouteName: string
  loginRouteName: string
  verifyRouteName: string
  store: any
}
