import { useContext } from 'react';
import { PageContext } from './page';
import { DataContext } from './data';
import { SystemContext } from './system';

export const usePageContext = () => {
    const context = useContext(PageContext);
    if (!context) {
      throw new Error("usePageContext must be used within a PageContextProvider");
    }
    return context;
  };

export const useDataContext = () => {
    const context = useContext(DataContext);
    if (!context) {
        throw new Error("usePageContext must be used within a DataContextProvider");
    } else {
        return context;
    }
};

export const useSystemContext = () => {
    const context = useContext(SystemContext);
    if (!context) {
        throw new Error("usePageContext must be used within a SystemContextProvider");
    } else {
        return context;
    }
}