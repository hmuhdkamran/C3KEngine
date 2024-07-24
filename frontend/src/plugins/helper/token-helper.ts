import { jwtDecode } from 'jwt-decode'
import { pick } from '.'
import type { IJwtToken, IUser } from '../models'
import { DefaultUser, GlobalConfig } from '../models'

const mapKeys = (obj: any, mapper: any) =>
  Object.entries(obj).reduce(
    (acc, [key, value]) => ({
      ...acc,
      [mapper(value, key)]: value,
    }),
    {},
  )

export class TokenHelper {
  public static getAccessToken(): string | null {
    return localStorage.getItem(GlobalConfig.auth.accessTokenKey)
  }

  public static setAccessToken(token: string): void {
    return localStorage.setItem(GlobalConfig.auth.accessTokenKey, token)
  }

  public static removeAccessToken(): void {
    return localStorage.removeItem(GlobalConfig.auth.accessTokenKey)
  }

  public static setAbilities(ability: string): void {
    return localStorage.setItem(GlobalConfig.auth.userAbility, ability)
  }

  public static getAbilities(): string {
    return localStorage.getItem(GlobalConfig.auth.userAbility) || '[]'
  }

  public static removeAbilities(): void {
    return localStorage.removeItem(GlobalConfig.auth.userAbility)
  }

  public static parseUserToken(token: string | null): IUser {
    const user: IUser = Object.assign({}, DefaultUser)

    if (token) {
      const decodedToken: IJwtToken = jwtDecode(token)
      const ns = GlobalConfig.claimsNamespace

      if (!user.authenticated)
        user.authenticated = true
      const name = decodedToken['http://schemas.xmlsoap.org/ws/2005/05/identity/claims/name']
      const email = decodedToken['http://schemas.xmlsoap.org/ws/2005/05/identity/claims/emailaddress'] || undefined
      const sid = decodedToken['http://schemas.xmlsoap.org/ws/2005/05/identity/claims/sid']
      const roles = decodedToken['http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role']

      const claimNames = Object.keys(decodedToken).filter(key => key.startsWith(ns)) as any
      const claims = <any>(mapKeys(pick(decodedToken, claimNames), (_value: any, key: any) => key.replace(ns, '')))

      user.claims = claims
      user.cultureName = claims.culturename
      user.displayName = name ? name[1] : ''
      user.name = user.email = email
      user.roles = Array.isArray(roles) ? roles : [roles]
      user.verified = true // claims.verified === "true" ? true : false;
      user.exp = new Date(decodedToken.exp * 1000)
      user.userId = sid
      user.username = decodedToken.sub
      user.timeZoneId = claims.timezoneid
    }

    return user
  }

  public static getBearerToken() {
    const token = localStorage.getItem(GlobalConfig.auth.accessTokenKey)

    return {
      Authorization: token ? `Bearer ${localStorage.getItem(GlobalConfig.auth.accessTokenKey)}` : null,
    }
  }

  public static isTokenCurrent(value: string | IUser) {
    let user: IUser | null = null

    if (typeof value === 'string')
      user = TokenHelper.parseUserToken(value)
    else
      user = value

    if (!user)
      return null
    else return user.exp && user.exp > new Date()
  }
}
