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
        updateLoading(state: { isLoading: boolean; }, action: PayloadAction<boolean>) {
            state.isLoading = action.payload;
        },
        updateUser(state: { user: IUser; }, action: PayloadAction<IUser>) {
            state.user = action.payload;
        },
        updateServices(state: { services: IService[]; }, action: PayloadAction<IService[]>) {
            state.services = action.payload;
        },
        updateMenuItems(state: { menuItems: MenuItem[]; }, action: PayloadAction<MenuItem[]>) {
            state.menuItems = action.payload;
        },
        updateSidebarMenu(state: { sidebarMenu: MenuItem[]; }, action: PayloadAction<MenuItem[]>) {
            state.sidebarMenu = action.payload;
        },
        updatePageState(state: { pageState: PageState; }, action: PayloadAction<Partial<PageState>>) {
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