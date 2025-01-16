import type { InjectionKey } from 'vue'

export interface Configuration {
  APP_API_URL: string
  APP_MICRO_URL: string
}

export const configKey: InjectionKey<Configuration> = Symbol('configKey')

export interface SingleSpaProps {
  name: string
  mountParcel: () => void
  singleSpa: {
    NOT_LOADED: 'NOT_LOADED'
    LOADING_SOURCE_CODE: 'LOADING_SOURCE_CODE'
    NOT_BOOTSTRAPPED: 'NOT_BOOTSTRAPPED'
    BOOTSTRAPPING: 'BOOTSTRAPPING'
    NOT_MOUNTED: 'NOT_MOUNTED'
    MOUNTING: 'MOUNTING'
    UPDATING: 'UPDATING'
    LOAD_ERROR: 'LOAD_ERROR'
    MOUNTED: 'MOUNTED'
    UNMOUNTING: 'UNMOUNTING'
    SKIP_BECAUSE_BROKEN: 'SKIP_BECAUSE_BROKEN'
    addErrorHandler: (handler: unknown) => void
    checkActivityFunctions: (location: Location) => void
    ensureJQuerySupport: (jQuery: unknown /* default to window.jQuery if has jQuery */) => void
    getAppNames: () => string[]
    getAppStatus: (appName: string) => string
    getMountedApps: () => string[]
    mountRootParcel: () => void
    navigateToUrl: (obj: unknown) => void
    pathToActiveWhen: (path: string, exactMatch: string) => void
    registerApplication: (
      appNameOrConfig: string,
      appOrLoadApp: unknown,
      activeWhen: unknown,
      customProps: unknown,
    ) => unknown
    removeErrorHandler: (handler: unknown) => void
    setBootstrapMaxTime: (time: unknown, dieOnTimeout: unknown, warningMillis: unknown) => unknown
    setMountMaxTime: (time: unknown, dieOnTimeout: unknown, warningMillis: unknown) => unknown
    setUnloadMaxTime: (time: unknown, dieOnTimeout: unknown, warningMillis: unknown) => unknown
    setUnmountMaxTime: (time: unknown, dieOnTimeout: unknown, warningMillis: unknown) => unknown
    start: (opts: unknown) => void
    triggerAppChange: () => void
    unloadApplication: (appName: string, opts: unknown) => unknown
    unregisterApplication: (appName: string) => void
  }
}

export const SingleSpaKey: InjectionKey<SingleSpaProps> = Symbol('SingleSpaProps')
