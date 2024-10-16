import { DefaultUser, IUser } from "../../types/axios";
import { createContext, FC, ReactNode, useEffect, useState } from "react";
import { TokenHelper } from "../helper/token-helper";
import { ICommonContext, IService, MenuItem } from "../../types/models";

export const SystemContext = createContext<ICommonContext | undefined>(
  undefined,
);

export const SystemProvider: FC<{ children: ReactNode }> = ({ children }) => {
  const [isLoading, setIsLoading] = useState(false);
  const [user, setUser] = useState<IUser>(DefaultUser);
  const [services, setServices] = useState<IService[]>([]);
  const [menuItems, setMenuItems] = useState<MenuItem[]>([]);
  const [sidebarMenu, setSidebarMenu] = useState<MenuItem[]>([]);

  useEffect(() => {
    const token = TokenHelper.getAccessToken();
    if (token) {
      const decodedToken = TokenHelper.parseUserToken(token);
      if (decodedToken) {
        const userExp = new Date(decodedToken.exp);
        if (userExp > new Date()) {
          setUser(decodedToken);
        } else {
          setUser(DefaultUser);
        }
      } else {
        setUser(DefaultUser);
      }
    } else {
      setUser(DefaultUser);
    }
  }, []);

  const updateLoading = (loading: boolean) => {
    setIsLoading(loading);
  };

  const updateUser = (user: IUser) => {
    setUser(user);
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
    updateSidebarMenu,
  };

  return (
    <SystemContext.Provider value={exporters}>
      {children}
    </SystemContext.Provider>
  );
};
