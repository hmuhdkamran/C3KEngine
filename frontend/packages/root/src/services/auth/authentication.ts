import { TokenHelper } from "@/plugins/helper/token-helper";
import { StoreService } from "@/plugins/service/service";
import { DefaultUser, IAccessToken, ICredential, IPayload, IUser } from "@/types/axios";
import Axios, { AxiosResponse } from "axios";
import GlobalConfig from '@/plugins/Api/config';

export class AuthenticationService extends StoreService {
    login(credentials: ICredential) {
        const processResponse = (token: IPayload<string>) => {
            if (token.data != null) {
                TokenHelper.setAccessToken(token.data)
                const parsed = TokenHelper.parseUserToken(token.data);

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
}