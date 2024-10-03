import { BreadcrumbItem } from "@/components/page/bread-crumb";
import { createContext, FC, ReactNode, useState } from "react";

export interface PageState {
  pageTitle: string;
  breadcrumbItems: BreadcrumbItem[];
  updatePageState: (newState: Partial<PageState>) => void;
}

export const PageContext = createContext<PageState | undefined>(undefined);

export const PageProvider: FC<{ children: ReactNode }> = ({ children }) => {
  const [pageState, setPageState] = useState<PageState>({
    pageTitle: "",
    breadcrumbItems: [],
    updatePageState: () => {},
  });

  const updatePageState = (newState: Partial<PageState>) => {
    setPageState((prevState) => ({ ...prevState, ...newState }));
  };

  return (
    <PageContext.Provider value={{ ...pageState, updatePageState }}>
      {children}
    </PageContext.Provider>
  );
};