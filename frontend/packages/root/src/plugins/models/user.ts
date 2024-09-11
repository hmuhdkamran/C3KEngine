import { IRouteMeta } from "./route-meta"

export interface IUser {
  authenticated: boolean
  claims: {}
  cultureName?: string
  createdOn?: Date
  displayName?: string
  email?: string
  enabled?: boolean
  name?: string
  roles: IRouteMeta[],
  username: string
  verified?: boolean
  exp: Date
  userId?: string
  timeZoneId?: string
  password_changed?: boolean
}

export const DefaultUser = <IUser><unknown>{
  authenticated: false,
  claims: {},
  cultureName: 'en',
  displayName: null,
  email: null,
  name: null,
  username: null,
  roles: [],
  timeZoneId: 'Asia/Karachi',
  verified: false,
  password_changed: false,
}
