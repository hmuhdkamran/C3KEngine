import { Actions, Subjects } from '@/plugins/casl/AppAbility'
import { TokenHelper } from '@/plugins/helper/token-helper'
import { DefaultUser, GlobalConfig, IAccessToken, ICredential, IPayload, IUser, PayloadMessageTypes } from '@/plugins/models'
import { StoreService } from '@/plugins/service/service'
import { IRouteMeta } from '@/router/route-meta'
import { ExtractSubjectType, MongoQuery, SubjectRawRule } from '@casl/ability'
import Axios, { AxiosResponse } from 'axios'

interface IPermission {
  action: string
  subject: string
}

export class AuthenticationService extends StoreService {
  convertToPermissions(roles: IRouteMeta[]) {
    const permission: Array<IPermission> = []

    roles.forEach((element: any) => {
      if (element !== undefined)
        permission.push({ action: 'read', subject: element.RouteName })
    })

    return permission
  }

  login(credentials: ICredential) {
    const processResponse = (token: IPayload<string>) => {
      if (token.data != null) {
        TokenHelper.setAccessToken(token.data)
        const parsed = TokenHelper.parseUserToken(token.data)

        if (parsed.roles !== undefined) {
          const userAbilities = this.convertToPermissions(parsed.roles) as SubjectRawRule<Actions, ExtractSubjectType<Subjects>, MongoQuery>[]

          TokenHelper.setAbilities(JSON.stringify(userAbilities))
        }

        return parsed
      }
    }

    return this.exec<IPayload<string>>(Axios.post(`${GlobalConfig.uri.auth}`, credentials))
      .then(value => this.processPayload(value))
      .then(processResponse)
  }

  logout() {
    const onSuccess = (res: AxiosResponse) => {
      const payload: IPayload<IAccessToken> = res.data

      if (payload.result.toLocaleLowerCase().startsWith("suc")) {
        const user: IUser = Object.assign({}, DefaultUser)

        TokenHelper.removeAccessToken()
        window.localStorage.removeItem('microsoft-auth')

        return user
      } else {
        return DefaultUser
      }
    }

    return Axios.put(`${GlobalConfig.uri.auth}logout`, null)
      .then(onSuccess)
  }


  changepassword(email: string, oldPassword: string, newPassword: string, confirmPassword: string) {
    return this.exec<any>(Axios.post(`${GlobalConfig.uri.auth}changepassword`, { username: email, oldPassword, newPassword, confirmPassword }))
      .then(value => this.processPayload(value))
      .catch(error => console.error(error))
  }

  resetpassword(userId: string, newPassword: string, confirmPassword: string) {
    return this.exec<any>(Axios.post(`${GlobalConfig.uri.auth}resetpassword`, { userId, newPassword, confirmPassword }))
      .then(value => this.processPayload(value))
      .catch(error => console.error(error))
  }
}