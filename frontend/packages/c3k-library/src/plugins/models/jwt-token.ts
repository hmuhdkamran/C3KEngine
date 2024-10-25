import type {IRouteMeta} from "../router/route-meta";

export interface IJwtToken {
  aud: string
  exp: number
  'http://schemas.xmlsoap.org/ws/2005/05/identity/claims/emailaddress': string
  'http://schemas.xmlsoap.org/ws/2005/05/identity/claims/name': string[]
  'http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role': IRouteMeta[]
  'http://schemas.xmlsoap.org/ws/2005/05/identity/claims/sid': string
  iss: string
  nbf: number
  sub: string
}
