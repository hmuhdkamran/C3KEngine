import type { NavigationGuard, NavigationGuardNext, RouteLocationNormalized, RouteRecordRaw } from 'vue-router'
import type { IRouteGuardOptions } from './route-guard-option'
import type { IRouteMeta } from './route-meta'
import { TokenHelper } from '../helpers'
import type { IUser } from '../models'
import type { IClaimMeta } from "./route-meta";

export const routeCheck = (user: IUser, to: string): boolean => {
  const requiredPermissions = user.roles as IRouteMeta[] || []

  console.log(to);

  // Check if user is authenticated and has the required permissions
  return user.authenticated && requiredPermissions.every(() => /*can('read', to)*/ true)
};

export const verifyCheck = (user: IUser, meta: IClaimMeta): boolean => {
  if (user.authenticated && (meta.private || meta.claims)) {
    return !user.verified;
  } else {
    return false;
  }
};

export let ActivedRoutes: RouteRecordRaw[] = [];

export const RouteGuards = (options: IRouteGuardOptions): NavigationGuard => {
  return async (to: RouteLocationNormalized, _from: RouteLocationNormalized, next: NavigationGuardNext) => {
    ActivedRoutes = [];
    to.matched.forEach((record: any) => ActivedRoutes.push(record));

    const user: IUser = TokenHelper.parseUserToken(TokenHelper.getAccessToken());

    if (to.matched.some((_r) => routeCheck(user, to.name as string))) {
      if (user.authenticated && to.meta.claims) {
        next(options.forbiddenRouteName);
      } else {
        next(options.loginRouteName);
      }
    } else if (
      to.name !== options.verifyRouteName &&
      to.matched.some((r) => verifyCheck(user, r.meta as IClaimMeta))
    ) {
      next(options.loginRouteName);
    } else {
      next();
    }

    window.scrollTo(0, 0);
  };
};
