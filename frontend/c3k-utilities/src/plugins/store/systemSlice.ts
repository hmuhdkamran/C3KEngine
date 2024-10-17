import { createSlice, PayloadAction } from '@reduxjs/toolkit';
import { DefaultUser, IUser } from '../../types/axios';
import { ICommonContext, IService, MenuItem, PageState } from '../../types/models';

const initialState: ICommonContext = {
    isLoading: false,
    user: DefaultUser,
    services: [],
    menuItems: [],
    sidebarMenu: [],
    pageState: {
        pageTitle: '',
        breadcrumbItems: [],
    },
};

const systemSlice = createSlice({
    name: 'system',
    initialState,
    reducers: {
        updateLoading(state: { isLoading: any; }, action: PayloadAction<boolean>) {
            state.isLoading = action.payload;
        },
        updateUser(state: { user: any; }, action: PayloadAction<IUser>) {
            state.user = action.payload;
        },
        updateServices(state: { services: any; }, action: PayloadAction<IService[]>) {
            state.services = action.payload;
        },
        updateMenuItems(state: { menuItems: any; }, action: PayloadAction<MenuItem[]>) {
            state.menuItems = action.payload;
        },
        updateSidebarMenu(state: { sidebarMenu: any; }, action: PayloadAction<MenuItem[]>) {
            state.sidebarMenu = action.payload;
        },
        updatePageState(state: { pageState: any; }, action: PayloadAction<Partial<PageState>>) {
            state.pageState = { ...state.pageState, ...action.payload };
        },
    },
});

export const {
    updateLoading,
    updateUser,
    updateServices,
    updateMenuItems,
    updateSidebarMenu,
    updatePageState,
} = systemSlice.actions;

export default systemSlice.reducer;