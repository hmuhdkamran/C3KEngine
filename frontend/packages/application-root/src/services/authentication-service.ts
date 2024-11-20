import {
  DefaultUser,
  GlobalConfig,
  StoreService,
  TokenHelper,
  type IPayload,
  type IRouteMeta,
  type ICredential,
  type Actions,
  type Subjects,
  type IAccessToken,
  type IUser,
} from "c3k-library";

import type { ExtractSubjectType, MongoQuery, SubjectRawRule } from "@casl/ability";
import Axios, { type AxiosResponse } from "axios";

interface IPermission {
  action: string;
  subject: string;
}

export class AuthenticationService extends StoreService {
  convertToPermissions(roles: IRouteMeta[]) {
    const permission: Array<IPermission> = [];

    roles.forEach((element: any) => {
      if (element !== undefined) {
        permission.push({ action: "read", subject: element.RouteName });
      }
    });

    return permission;
  }

  login(credentials: ICredential) {
    const processResponse = (token: IPayload<string>) => {
      if (token.data != null) {
        TokenHelper.setAccessToken(token.data);
        const parsed = TokenHelper.parseUserToken(token.data);

        if (parsed.roles !== undefined) {
          const userAbilities = this.convertToPermissions(
            parsed.roles
          ) as SubjectRawRule<
            Actions,
            ExtractSubjectType<Subjects>,
            MongoQuery
          >[];
          TokenHelper.setAbilities(JSON.stringify(userAbilities));
        }

        return parsed;
      }
    };

    return this.exec(Axios.post(`${GlobalConfig.uri.auth}/login`, credentials))
      .then((value: any) => this.processPayload(value))
      .then((value: any) => processResponse(value as IPayload<string>)); // Type assertion
  }

  logout() {
    const onSuccess = (res: AxiosResponse) => {
      const payload: IPayload<IAccessToken> = res.data;

      if (payload.result.toLocaleLowerCase().startsWith("suc")) {
        const user: IUser = Object.assign({}, DefaultUser);

        TokenHelper.removeAccessToken();
        window.localStorage.removeItem("microsoft-auth");

        return user;
      } else {
        return DefaultUser;
      }
    };

    return Axios.put(`${GlobalConfig.uri.auth}logout`, null).then(onSuccess);
  }
}
