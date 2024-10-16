import { IUriConfig } from "../../types/models";

const uri: IUriConfig = {};

// Environment-specific API path
const DEV_API = `${import.meta.env.VITE_DEV_API_PATH}:${import.meta.env.VITE_DEV_API_PORT}`;

// Utility to dynamically add properties to the uri object
const addProp = (obj: Record<string, string>, propName: string, value: string) => {
    const isProduction = process.env.NODE_ENV === 'production';

    Object.defineProperty(obj, propName, {
        enumerable: false,
        get: () => {
            return isProduction ? `//${window.location.host}${value}` : `//${DEV_API}${value}`;
        },
    });
};

// Adding properties to the uri object
addProp(uri as Record<string, string>, 'auth', '/api/auth');
addProp(uri as Record<string, string>, 'content', '/api/content/');
addProp(uri as Record<string, string>, 'site', '');
addProp(uri as Record<string, string>, 'services', '/api/');

// Configuration object with the uri and other settings
const config = {
    uri,
    claimsNamespace: 'https://1411c3k1911/claims/',
    auth: {
        accessTokenKey: 'AUTH-LOCAL',
        externalProviderKey: 'AUTH-EXTERNAL',
        userAbility: 'USER-ABILITY',
    },
    uopt: 'UOPT',
    xsrf: {
        cookieName: 'XSRF-TOKEN',
        headerName: 'X-XSRF-TOKEN',
    },
};

export default config;