import type { NavigationGuard, NavigationGuardNext, RouteLocationNormalized, RouteRecordRaw } from 'vue-router';
import type { IRouteGuardOptions } from './route-guard-option';
import type { IRouteMeta } from './route-meta';
import { TokenHelper } from '../helpers';
import type { IUser } from '../models';
// import type { IClaimMeta } from "./route-meta";

export const routeCheck = (user: IUser, toRouteMeta: IRouteMeta | undefined): boolean => {
    if (!user.authenticated || !toRouteMeta || !toRouteMeta.RouteName) {
        return false; // Or true, depending on your default behavior for public routes
    }

    const userRoles = user.roles as IRouteMeta[] || [];

    // Check if the user has at least one role that matches the route's required role.
    return userRoles.some(userRole => userRole.RouteName === toRouteMeta.RouteName && userRole.Operation === toRouteMeta.Operation);
};

export const verifyCheck = (user: IUser, meta: any): boolean => {
    if (user.authenticated && (meta?.authRequired || meta?.claims)) {
        return !user.verified;
    } else {
        return false;
    }
};

export let ActivedRoutes: RouteRecordRaw[] = [];

export const RouteGuards = (options: IRouteGuardOptions): NavigationGuard => {
    return async (to: RouteLocationNormalized, _from: RouteLocationNormalized, next: NavigationGuardNext) => {
        ActivedRoutes = [];

        console.log(`Try to find: ${JSON.stringify(to.meta)}`)

        to.matched.forEach((record: any) => ActivedRoutes.push(record));

        const user: IUser = TokenHelper.parseUserToken(TokenHelper.getAccessToken());        

        const routeMeta: IRouteMeta | undefined = to.meta as any;

        if (routeMeta?.authRequired === true && !routeCheck(user, routeMeta)) {
            if (user.authenticated) {
                next(options.forbiddenRouteName);
            } else {
                next({ name: options.loginRouteName, query: { redirect: to.fullPath } });
            }
        } else if (
            to.name !== options.verifyRouteName &&
            to.matched.some((r) => verifyCheck(user, r.meta))
        ) {
            next(options.loginRouteName);
        } else {
            next();
        }

        window.scrollTo(0, 0);
    };
};
