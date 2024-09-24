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