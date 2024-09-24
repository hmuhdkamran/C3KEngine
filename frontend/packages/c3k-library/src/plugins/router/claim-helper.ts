import type { IClaimsHelper } from './claims-helpers'
import type { IUser } from '@/plugins/models'

export class ClaimHelper implements IClaimsHelper {
  satisfies(user: IUser, claims: string[]): boolean {
    let satisfied = false
    if (!user.roles)
      return satisfied

    user.roles.forEach(assigned => {
      claims.forEach(checking => {
        if (assigned.RouteName === checking)
          satisfied = true
      })
    })

    console.log(`satisfies ${user.roles} AND ${claims} ARE ${satisfied}`)

    return satisfied
  }

  satisfiesAny(user: IUser, claims: string[]): boolean {
    let satisfied = false
    if (!user.claims)
      return satisfied

    user.roles.forEach(assigned => {
      claims.forEach(checking => {
        if (assigned.RouteName === checking)
          satisfied = true
      })
    })

    console.log(`satisfiesAny ${user.roles} AND ${claims} ARE ${satisfied}`)

    return satisfied
  }
}
