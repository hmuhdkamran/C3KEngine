import Login from "./auth/login";
import RegisterForm from "./auth/register";

import DashboardIndex from "./dashboard/main";
import LandingPage from "./landing";

import UserModule from "./roles/user-module";

const routePath = {
    LOGIN: '/auth/login',
    REGISTER: '/auth/register',
    LANDING: '/',
    DASHBOARD: '/dashboard',
    USER_MODULE: '/roles/user-module'
};


export {
    Login,
    RegisterForm,
    DashboardIndex,
    LandingPage,
    UserModule,

    routePath
};