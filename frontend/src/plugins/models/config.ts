interface IUriConfig {
  auth?: string
  content?: string
  site?: string
  services?: string
}

const uri: IUriConfig = {}

const DEV_API = `${import.meta.env.VITE_DEV_API_PATH}:${import.meta.env.VITE_DEV_API_PORT}`

const addProp = (obj: {}, propName: string, value: string) => {
  const isProduction = process.env.NODE_ENV === 'production'

  Object.defineProperty(obj, propName, {
    enumerable: false,
    get: () => {
      return isProduction ? `//${window.location.host}${value}` : `//${DEV_API}${value}`
    },
  })
}

addProp(uri, 'auth', '/api/auth')
addProp(uri, 'content', '/api/content/')
addProp(uri, 'site', '')
addProp(uri, 'services', '/api/')

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
}

export default config
