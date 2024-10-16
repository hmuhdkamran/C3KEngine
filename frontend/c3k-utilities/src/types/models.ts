import { ReactNode } from "react";
import { IUser } from "./axios";

export interface IService {
    name: string;
    icon: string;
    description: string;
    route: string;
    category?: string;
}

export interface ServiceDropdownProps {
    services: IService[];
    selectedCardTitle?: string;
    showDropdown: boolean;
    toggleDropdown: () => void;
    getCategories: () => Promise<string[]>;
}

export interface SidebarProps {
    showSidebarDropdown: boolean;
    onToggleSidebar: () => void;
}

export interface CardProps {
    title?: string;
    description?: string;
    status?: string;
    buttonText?: string;
    iconClass?: string;
    containerClass?: string;
    showHeader?: boolean;
    showFooter?: boolean;
    children?: React.ReactNode;
    headerSlot?: React.ReactNode;
    footerSlot?: React.ReactNode;
    onClick?: () => void; // Added the onClick prop
    borderColor?: string;
}

export interface MenuItem {
    name: string;
    path?: string;
    icon?: string;
    children?: MenuItem[];
}

export interface MenuProps {
    items: MenuItem[];
}

export interface ConfirmDialogProps {
    message: string;
    onConfirm: () => void;
    onCancel: () => void;
};

export interface DropdownProps {
    showDropdown: boolean;
    toggleDropdown: () => void;
}

export interface BreadcrumbItem {
    title: string;
    route: string;
    icon: string;
}

export interface BreadcrumbProps {
    items: BreadcrumbItem[];
    homeClick: () => void;
}
export interface HeaderAreaProps {
    pageHeading: string;
    goToMain: () => void;
    children?: ReactNode;
}

export interface PageFilterProps {
    children?: ReactNode;
}

export interface ICommonContext {
    isLoading: boolean;
    user: IUser;
    services: IService[];
    menuItems: MenuItem[];
    sidebarMenu: MenuItem[];
    updateLoading: (loading: boolean) => void;
    updateUser: (user: IUser) => void;
    updateServices: (values: IService[]) => void;
    updateMenuItems: (values: MenuItem[]) => void;
    updateSidebarMenu: (values: MenuItem[]) => void;
}

export interface IUriConfig {
    auth?: string;
    content?: string;
    site?: string;
    services?: string;
}
