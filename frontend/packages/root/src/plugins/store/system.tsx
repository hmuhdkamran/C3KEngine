import { DefaultUser, IUser } from "@/types/axios";
import { createContext, FC, ReactNode, useState } from "react";
import { TokenHelper } from "@/plugins/helper/token-helper";
import { IService } from "@/components/extra/service-dropdown";
import { MenuItem } from "@/components/extra/menu-items";

export interface ICommonContext {
  isLoading: boolean;
  user: IUser;
  services: IService[];
  menuItems: MenuItem[];
  sidebarMenu: MenuItem[];
  updateLoading: (loading: boolean) => void;
  updateUser: (user: IUser | string) => void;
  updateServices: (values: IService[]) => void;
  updateMenuItems: (values: MenuItem[]) => void;
  updateSidebarMenu: (values: MenuItem[]) => void;
}

export const SystemContext = createContext<ICommonContext | undefined>(
  undefined
);

export const SystemProvider: FC<{ children: ReactNode }> = ({ children }) => {
  const [isLoading, setIsLoading] = useState(false);
  const [user, setUser] = useState<IUser>(DefaultUser);
  const [services, setServices] = useState<IService[]>([]);
  const [menuItems, setMenuItems] = useState<MenuItem[]>([]);
  const [sidebarMenu, setSidebarMenu] = useState<MenuItem[]>([]);

  const updateLoading = (loading: boolean) => {
    setIsLoading(loading);
  };

  const updateUser = (user: IUser | string) => {
    if (typeof user === "string") {
      setUser(TokenHelper.parseUserToken(user));
    } else {
      setUser(user);
    }
  };

  const updateServices = (values: IService[]) => {
    setServices(values);
  };

  const updateMenuItems = (values: MenuItem[]) => {
    setMenuItems(values);
  };

  const updateSidebarMenu = (values: MenuItem[]) => {
    setSidebarMenu(values);
  };

  const exporters = {
    isLoading,
    user,
    services,
    menuItems,
    sidebarMenu,
    updateLoading,
    updateUser,
    updateServices,
    updateMenuItems,
    updateSidebarMenu
  };

  return (
    <SystemContext.Provider value={exporters}>
      {children}
    </SystemContext.Provider>
  );
};
