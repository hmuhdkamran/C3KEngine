import { createContext, FC, ReactNode, useState } from "react";

export interface DataTableState {
  totalRecords: number;
  itemsPerPage: number;
  currentPage: number;
  searchQuery: string;
  totalPages: number;
  setPage: (page: number) => void;
  updateTotalRecords: (records: number) => void;
  updateItemsPerPage: (count: number) => void;
  updateSearchQuery: (value: string) => void;
}

export const DataContext = createContext<DataTableState | undefined>(undefined);

export const DataProvider: FC<{ children: ReactNode }> = ({ children }) => {
  const [totalRecords, setTotalRecords] = useState(0);
  const [itemsPerPage, setItemsPerPage] = useState(20);
  const [currentPage, setCurrentPage] = useState(1);
  const [searchQuery, setSearchQuery] = useState("");

  const totalPages = Math.ceil(totalRecords / itemsPerPage) || 0;

  const setPage = (page: number) => {
    if (page >= 1 && page <= totalPages) {
      setCurrentPage(page);
    }
  };

  const updateTotalRecords = (records: number) => {
    setTotalRecords(records);
  };

  const updateItemsPerPage = (count: number) => {
    setItemsPerPage(count);
  };

  const updateSearchQuery = (value: string) => {
    setSearchQuery(value);
  };

  const contextValue = {
    totalRecords,
    itemsPerPage,
    currentPage,
    searchQuery,
    totalPages,
    setPage,
    updateTotalRecords,
    updateItemsPerPage,
    updateSearchQuery,
  };

  return (
    <DataContext.Provider value={contextValue}>{children}</DataContext.Provider>
  );
};
