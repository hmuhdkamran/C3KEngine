import { defineStore } from "pinia";

export interface DataTableState {
    totalRecords: number,
    itemsPerPage: number,
    currentPage: number,
    searchQuery: string
};

export const useTableStore = defineStore('table-store', {
    state: (): DataTableState => ({
        totalRecords: 0,
        itemsPerPage: 20,
        currentPage: 1,
        searchQuery: '',
    }),
    getters: {
        totalPages: (state) => {
            return Math.ceil(state.totalRecords / state.itemsPerPage) || 0;
        }
    },
    actions: {
        setPage(page: number) {
            if (page >= 1 && page <= this.totalPages) {
                this.currentPage = page;
            }
        },
        updateTotalRecords(records: number) {
            this.totalRecords = records;
        },
        updateItemsPerPage(count: number) {
            this.itemsPerPage = count;
        }
    }
});
