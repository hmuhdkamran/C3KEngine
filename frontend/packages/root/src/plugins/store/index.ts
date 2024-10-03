import { useContext } from 'react';
import { PageContext, PageState } from './page';
import { DataContext, DataTableState } from './data';
import { ICommonContext, SystemContext } from './system';
import { DefaultUser } from '@/types/axios';

export const usePageContext = () => {
    const context = useContext(PageContext);
    if (!context) {
        return {
            pageTitle: '',
            breadcrumbItems: [],
            updatePageState: () => { }
        } as PageState;
    } else {
        return context;
    }
};

export const useDataContext = () => {
    const context = useContext(DataContext);
    if (!context) {
        return {
            totalRecords: 0,
            itemsPerPage: 0,
            currentPage: 0,
            searchQuery: '',
            totalPages: 0,
            setPage: () => { },
            updateTotalRecords: () => { },
            updateItemsPerPage: () => { },
            updateSearchQuery: () => { }
        } as DataTableState;
    } else {
        return context;
    }
};

export const useSystemContext = () => {
    const context = useContext(SystemContext);
    if (!context) {
        return {
            isLoading: false,
            user: DefaultUser,
            services: [],
            menuItems: [],
            sidebarMenu: [],
            updateLoading: () => { },
            updateUser: () => { },
            updateServices: () => { },
            updateMenuItems: () => { },
            updateSidebarMenu: () => { },
        } as ICommonContext;
    } else {
        return context;
    }
}