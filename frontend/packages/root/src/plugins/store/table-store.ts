import { defineStore } from "pinia";

export interface DataTableState {
    totalPages: number,
    currentPage: number,
    searchQuery: string
};

export const useTableStore = defineStore('table-store', {
    state: (): DataTableState => ({
        totalPages: 0,
        currentPage: 0,
        searchQuery: '',
    }),
    actions: {
        goToPage(page: number) {
            if (page >= 1 && page <= this.totalPages) {
                this.currentPage = page;
            }
        }
    }
});