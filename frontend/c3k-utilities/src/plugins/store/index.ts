import { useContext } from 'react';
import { PageContext } from './page';
import { DataContext } from './data';
import { SystemContext } from './system';

export const useSystemContext = () => {
    const context = useContext(SystemContext);
    console.log("useSystemContext called", context);
    if (!context) {
        throw new Error("useSystemContext must be used within a SystemProvider");
    }
    return context;
}

export const usePageContext = () => {
    const context = useContext(PageContext);
    console.log("usePageContext called", context);
    if (!context) {
        throw new Error("usePageContext must be used within a PageProvider");
    }
    return context;
};

export const useDataContext = () => {
    const context = useContext(DataContext);
    console.log("useDataContext called", context);
    if (!context) {
        throw new Error("useDataContext must be used within a DataProvider");
    }
    return context;
};