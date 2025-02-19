import { LocalStorageHelper } from "../helpers";

/**
 * Return if user is logged in
 * This is completely up to you and how you want to store the token in your frontend application
 * e.g. If you are using cookies to store the application please update this function
 */
export const isUserLoggedIn = () => !!(LocalStorageHelper.get<string>('userData') && LocalStorageHelper.get<string>('accessToken'))
