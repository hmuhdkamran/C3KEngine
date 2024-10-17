import { jwtDecode } from 'jwt-decode';
import { pick } from './pick';
import GlobalConfig from '../Api/config';
import { DefaultUser, IJwtToken, IUser } from '../../types/axios';

const mapKeys = <T, K extends string | number | symbol>(
    obj: Record<K, T>,
    mapper: (value: T, key: K) => K
): Record<K, T> => {
    return Object.entries(obj).reduce<Record<K, T>>(
        (acc, [key, value]) => {
            const typedKey = key as K; // Cast key to K
            const typedValue = value as T; // Cast value to T
            return {
                ...acc,
                [mapper(typedValue, typedKey)]: typedValue, // Use the typed values
            };
        },
        {} as Record<K, T>
    );
};


export class TokenHelper {
    public static getAccessToken(): string | null {
        return localStorage.getItem(GlobalConfig.auth.accessTokenKey);
    }

    public static setAccessToken(token: string): void {
        localStorage.setItem(GlobalConfig.auth.accessTokenKey, token);
    }

    public static removeAccessToken(): void {
        localStorage.removeItem(GlobalConfig.auth.accessTokenKey);
    }

    public static parseUserToken(token: string | null): IUser {
        const user: IUser = { ...DefaultUser };

        if (token) {
            const decodedToken: IJwtToken = jwtDecode(token);
            const ns = GlobalConfig.claimsNamespace;

            user.authenticated = true;

            const name = decodedToken['http://schemas.xmlsoap.org/ws/2005/05/identity/claims/name'];
            const email = decodedToken['http://schemas.xmlsoap.org/ws/2005/05/identity/claims/emailaddress'] || undefined;
            const sid = decodedToken['http://schemas.xmlsoap.org/ws/2005/05/identity/claims/sid'];
            const roles = decodedToken['http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role'];

            const claimNames = Object.keys(decodedToken).filter(key => key.startsWith(ns)) as Array<keyof typeof decodedToken>;
            const claims = mapKeys(
                pick(decodedToken, claimNames),
                (_value, key: keyof IJwtToken) => key.replace(ns, '') as keyof IJwtToken
            );

            user.claims = claims;
            // user.cultureName = claims.culturename;
            user.displayName = name ? name[1] : '';
            user.name = user.email = email;
            user.roles = Array.isArray(roles) ? roles : [roles];
            // user.verified = claims.verified === "true" ? true : false;
            user.exp = decodedToken.exp ? new Date(decodedToken.exp * 1000).toISOString() : null;
            user.userId = sid;
            user.username = decodedToken.sub;
            // user.timeZoneId = claims.timezoneid;
        }

        return user;
    }

    public static getBearerToken() {
        const token = localStorage.getItem(GlobalConfig.auth.accessTokenKey);
        return {
            Authorization: token ? `Bearer ${token}` : null,
        };
    }

    public static isTokenCurrent(value: string | IUser): boolean | null {
        let user: IUser | null = null;

        if (typeof value === 'string') {
            user = TokenHelper.parseUserToken(value);
        } else {
            user = value;
        }

        if (!user) return null;

        // Check if exp is in ISO string format or is a valid Date
        const expirationDate = typeof user.exp === 'string' ? new Date(user.exp) : null;

        // Return whether the token is current (not expired)
        return expirationDate ? expirationDate > new Date() : null;
    }
}
