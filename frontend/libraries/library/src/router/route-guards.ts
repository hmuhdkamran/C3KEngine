import type { NavigationGuard, NavigationGuardNext, RouteLocationNormalized, RouteRecordRaw } from 'vue-router';
import type { IRouteGuardOptions } from './route-guard-option';
import type { IRouteMeta } from './route-meta';
import { TokenHelper } from '../helpers';
import type { IUser } from '../models';

const getUserRoles = (user: IUser): IRouteMeta[] => {
    return (user.roles as IRouteMeta[]) || [];
};

export const routeCheck = (user: IUser, toRouteMeta: IRouteMeta | undefined): boolean => {
    if (!user.authenticated || !toRouteMeta?.name) {
        return false;
    }

    const userRoles = getUserRoles(user);

    return userRoles.some(userRole =>
        userRole.RouteName === toRouteMeta.RouteName && userRole.Operation === toRouteMeta.Operation
    );
};

export const verifyCheck = (user: IUser, meta: any): boolean => {
    const authRequired = meta?.authRequired === true;
    const hasClaims = meta?.claims !== undefined;

    if (user.authenticated && (authRequired || hasClaims)) {
        return !user.verified;
    }
    return false;
};

export let ActivedRoutes: RouteRecordRaw[] = [];

export const RouteGuards = (options: IRouteGuardOptions): NavigationGuard => {
    return async (to: RouteLocationNormalized, _from: RouteLocationNormalized, next: NavigationGuardNext) => {
        ActivedRoutes = [];

        to.matched.forEach((record: any) => ActivedRoutes.push(record));

        let user: IUser | null = null;
        try {
            const token = TokenHelper.getAccessToken();
            if (token) {
                user = TokenHelper.parseUserToken(token);
            } else {
                user = { authenticated: false } as IUser;
            }
        } catch (error) {
            user = { authenticated: false } as IUser;
        }

        const routeMeta: IRouteMeta | undefined = to.meta as any;

        if (routeMeta?.authRequired === true && !routeCheck(user || { authenticated: false } as IUser, routeMeta)) {
            if (user?.authenticated) {
                next(options.forbiddenRouteName);
            } else {
                next({ name: options.loginRouteName, query: { redirect: to.fullPath } });
            }
        } else if (
            to.name !== options.verifyRouteName &&
            to.matched.some((r) => verifyCheck(user || { authenticated: false } as IUser, r.meta))
        ) {
            console.log(`To: ${JSON.stringify(to)}, Options: ${JSON.stringify(options)}, IUser: ${JSON.stringify(user)}`);
            next(options.loginRouteName);
        } else {
            next();
        }

        window.scrollTo(0, 0);
    };
};
