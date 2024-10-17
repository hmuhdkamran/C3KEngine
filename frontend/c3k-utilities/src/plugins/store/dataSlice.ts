import { createSlice, PayloadAction } from '@reduxjs/toolkit';
import { DataTableState } from '../../types/models';

const initialState: DataTableState = {
    totalRecords: 0,
    itemsPerPage: 20,
    currentPage: 1,
    searchQuery: '',
    totalPages: 0,
};

const dataSlice = createSlice({
    name: 'dataTable',
    initialState,
    reducers: {
        setPage(state: { totalPages: number; currentPage: any; }, action: PayloadAction<number>) {
            const page = action.payload;
            if (page >= 1 && page <= state.totalPages) {
                state.currentPage = page;
            }
        },
        updateTotalRecords(state: DataTableState, action: PayloadAction<number>) {
            state.totalRecords = action.payload;
            state.totalPages = Math.ceil(state.totalRecords / state.itemsPerPage) || 0;
        },
        updateItemsPerPage(state: DataTableState, action: PayloadAction<number>) {
            state.itemsPerPage = action.payload;
            state.totalPages = Math.ceil(state.totalRecords / state.itemsPerPage) || 0;
        },
        updateSearchQuery(state: DataTableState, action: PayloadAction<string>) {
            state.searchQuery = action.payload;
        },
    },
});

export const {
    setPage,
    updateTotalRecords,
    updateItemsPerPage,
    updateSearchQuery,
} = dataSlice.actions;

export default dataSlice.reducer;