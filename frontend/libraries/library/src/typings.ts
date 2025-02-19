import type { VNode } from "vue"
import type { RouteLocationRaw } from 'vue-router'

declare type ATagTargetAttrValues = '_blank' | '_self' | '_parent' | '_top' | 'framename'
declare type ATagRelAttrValues =
    | 'alternate'
    | 'author'
    | 'bookmark'
    | 'external'
    | 'help'
    | 'license'
    | 'next'
    | 'nofollow'
    | 'noopener'
    | 'noreferrer'
    | 'prev'
    | 'search'
    | 'tag'

export interface I18nLanguage {
    label: string
    i18nLang: string
};

interface ISocialMedia {
    name: string,
    link: string,
    icon: string
};

export interface IConfiguration {
    name: string;
    logo: VNode;
    enableI18n: boolean;
    language: string;
    isRtl: boolean;
    titleColor: string;
    primaryColor: string;
    backgroundColor: string;
    sidebarColor: string;
    socialMedia: ISocialMedia[]
};

interface AclProperties {
    action: string
    subject: string
}

export interface NavLinkProps {
    to?: RouteLocationRaw | string | null
    href?: string
    target?: ATagTargetAttrValues
    rel?: ATagRelAttrValues
}

export interface NavLink extends NavLinkProps, Partial<AclProperties> {
    title: string
    icon?: unknown
    badgeContent?: string
    badgeClass?: string
    disable?: boolean
}

export interface NavGroup extends Partial<AclProperties> {
    title: string
    icon?: unknown
    badgeContent?: string
    badgeClass?: string
    children: (NavLink | NavGroup)[]
    disable?: boolean
}

export interface IMenuItem {
    name: string;
    title: string;
    icon: string;
    route: string;
    open: boolean;
    children?: IMenuItem[];
};