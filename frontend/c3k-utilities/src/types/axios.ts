import 'axios';
declare module 'axios' {
    interface AxiosRequestConfig {
        skipIntercept?: boolean;
        skipShowTips?: boolean;
    }
}

export interface IAccessToken {
    access_token: string
    expires_on: number
}

export interface ICredential {
    username: string
    password: string
}

export interface IClaimMeta {
    private?: boolean
    claims?: string[]
    any?: boolean
}

export interface IRouteMeta {
    RouteName: string,
    Operation: string
}

export interface IPayload<T> {
    data: T
    result: string,
    description: string
}

export interface IUser {
    authenticated: boolean,
    claims: object,
    cultureName?: string,
    createdOn?: Date,
    displayName?: string,
    email?: string,
    enabled?: boolean,
    name?: string,
    roles: IRouteMeta[],
    username: string,
    verified?: boolean,
    exp: Date,
    userId?: string,
    timeZoneId?: string,
    password_changed?: boolean,
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

export interface IStoreService {
    exec: <T>(cb: Promise<unknown>) => Promise<IPayload<T>>
}
