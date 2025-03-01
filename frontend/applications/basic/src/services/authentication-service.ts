import {
  DefaultUser,
  GlobalConfig,
  StoreService,
  TokenHelper,
  type IPayload,
  type IRouteMeta,
  type ICredential,
  type IAccessToken,
  type IUser,
  useSystemStore,
  type ISignupUsers,
} from 'c3-library'

import Axios, { type AxiosResponse } from 'axios'

interface IPermission {
  action: string
  subject: string
}

export class AuthenticationService extends StoreService {
  convertToPermissions(roles: IRouteMeta[]) {
    const permission: Array<IPermission> = []

    roles.forEach((element: any) => {
      if (element !== undefined) {
        permission.push({ action: 'read', subject: element.RouteName })
      }
    })

    return permission
  }

  login(credentials: ICredential) {
    const processResponse = (token: IPayload<string>) => {
      if (token != null) {
        TokenHelper.setAccessToken(`${token}`)
        const store = useSystemStore()

        const parsed = TokenHelper.parseUserToken(`${token}`)
        store.updateUser(parsed)

        if (parsed.roles !== undefined) {
          // const userAbilities = this.convertToPermissions(
          //   parsed.roles
          // ) as SubjectRawRule<
          //   Actions,
          //   ExtractSubjectType<Subjects>,
          //   MongoQuery
          // >[];
          // TokenHelper.setAbilities(JSON.stringify(userAbilities));
        }

        return parsed
      }
    }

    return this.exec(Axios.post(`${GlobalConfig.uri.auth}/login`, credentials))
      .then((value: any) => this.processPayload(value))
      .then((value: any) => processResponse(value as IPayload<string>)) // Type assertion
  }

  logout() {
    const onSuccess = (res: AxiosResponse) => {
      const payload: IPayload<IAccessToken> = res.data

      if (payload.result.toLocaleLowerCase().startsWith('suc')) {
        const user: IUser = Object.assign({}, DefaultUser)

        TokenHelper.removeAccessToken()

        return true
      } else {
        return false
      }
    }

    const token = TokenHelper.getAccessToken()
    if (token) {
      const user = TokenHelper.parseUserToken(token)

      return this.post(
        `${GlobalConfig.uri.auth}/logout`,
        { username: user.username, password: '' },
        true,
      ).then((response: any) => onSuccess(response.data))
    }
    return false
  }

  async signup(entity: ISignupUsers): Promise<boolean> {
    try {
      const response = await this.post(`${GlobalConfig.uri.auth}/user_signup`, entity, true)
      return Boolean(response.data)
    } catch (error) {
      console.error('Signup failed:', error)
      return false
    }
  }

  allProducts() {
    const token = TokenHelper.getAccessToken()
    if (token) {
      const user = TokenHelper.parseUserToken(token)

      if (user.username) {
        return this.post(
          `${GlobalConfig.uri.auth}/user_products`,
          { username: '', product: '' },
          true,
        ).then((value: any) => this.processPayload(value))
      }
    }
  }

  userProducts() {
    const token = TokenHelper.getAccessToken()
    if (token) {
      const user = TokenHelper.parseUserToken(token)

      if (user.username) {
        return this.post(
          `${GlobalConfig.uri.auth}/user_products`,
          { username: user.username, product: '' },
          true,
        ).then((value: any) => this.processPayload(value))
      }
    }
  }

  userProductClaims(product: string) {
    const token = TokenHelper.getAccessToken()
    if (token) {
      const user = TokenHelper.parseUserToken(token)

      if (user.username) {
        return this.post(
          `${GlobalConfig.uri.auth}/user_products_claims`,
          { username: user.username, product: product },
          true,
        ).then((value: any) => this.processPayload(value))
      }
    }
  }
}
