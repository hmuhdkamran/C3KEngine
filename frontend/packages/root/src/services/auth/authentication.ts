import Axios, { AxiosResponse } from "axios";
import { StoreService, TokenHelper, DefaultUser, IAccessToken, ICredential, IPayload, IUser } from "c3k-utilities";

export class AuthenticationService extends StoreService {
    login(credentials: ICredential): Promise<IUser> {
        const processResponse = (token: IPayload<string>): IUser => {
            if (token.data != null) {
                TokenHelper.setAccessToken(token.data);
                const parsed = TokenHelper.parseUserToken(token.data);
                return parsed;
            }
            return DefaultUser;
        };

        return this.exec<IPayload<string>>(Axios.post(`${GlobalConfig.uri.auth}`, credentials))
            .then(value => this.processPayload(value))
            .then(processResponse);
    }

    logout(): Promise<IUser> {
        const onSuccess = (res: AxiosResponse): IUser => {
            const payload: IPayload<IAccessToken> = res.data;

            if (payload.result.toLocaleLowerCase().startsWith("suc")) {
                const user: IUser = Object.assign({}, DefaultUser);

                TokenHelper.removeAccessToken();
                window.localStorage.removeItem('microsoft-auth');

                return user;
            } else {
                return DefaultUser;
            }
        };

        return Axios.put(`${GlobalConfig.uri.auth}logout`, null)
            .then(onSuccess);
    }
}
