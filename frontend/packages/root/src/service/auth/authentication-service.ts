import { TokenHelper } from '@/plugins/helper/token-helper'
import { GlobalConfig, ICredential, IPayload } from '@/plugins/models'
import { StoreService } from '@/plugins/service/service'
import Axios from 'axios'

export class AuthenticationService extends StoreService {
    login(credentials: ICredential) {
        const processResponse = (token: IPayload<string>) => {
            if (token.data != null) {
              TokenHelper.setAccessToken(token.data)
              const parsed = TokenHelper.parseUserToken(token.data)

            //   if (parsed.roles !== undefined) {
            //     const userAbilities = this.convertToPermissions(parsed.roles) as SubjectRawRule<Actions, ExtractSubjectType<Subjects>, MongoQuery>[]
      
            //     TokenHelper.setAbilities(JSON.stringify(userAbilities))
            //   }
      
              return parsed 
            }
          }
      
          return this.exec<IPayload<string>>(Axios.post(`${GlobalConfig.uri.auth}`, credentials))
            .then(value => this.processPayload(value))
            .then(processResponse)
    }
}