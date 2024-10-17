import { ReactNode } from "react";
import { useNavigate } from "react-router-dom";
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
    navigate: ReturnType<typeof useNavigate>;
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

export interface DataTableState {
    totalRecords: number;
    itemsPerPage: number;
    currentPage: number;
    searchQuery: string;
    totalPages: number;
  }

export interface BreadcrumbItem {
    title: string;
    route: string;
    icon: string;
}

export interface BreadcrumbProps {
    homeClick: () => void;
}
export interface HeaderAreaProps {
    goToMain: () => void;
    children?: ReactNode;
}

export interface PageState {
    pageTitle: string;
    breadcrumbItems: BreadcrumbItem[];
}

export interface PageFilterProps {
    children?: ReactNode;
}

export interface ICommonContext {
    isLoading: boolean;
    user: IUser;
    pageState: PageState,
    services: IService[];
    menuItems: MenuItem[];
    sidebarMenu: MenuItem[];
}

export interface IUriConfig {
    auth?: string;
    content?: string;
    site?: string;
    services?: string;
}
