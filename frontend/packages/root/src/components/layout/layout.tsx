import { FC } from "react";
import { Outlet } from "react-router-dom";
import AuthHeader from "./auth/header";
import GernalHeader from "./gernal/header";
import AuthFooter from "./auth/footer";
import GernalFooter from "./gernal/footer";

interface LayoutProps {
  isAuthenticated?: boolean;  // Optional prop to determine layout type
}

const Layout: FC<LayoutProps> = ({ isAuthenticated = false }) => {
  return (
    <div className="text-blueGray-700 antialiased">
      {isAuthenticated ? <AuthHeader /> : <GernalHeader />}
      <Outlet />
      {isAuthenticated ? <AuthFooter /> : <GernalFooter />}
    </div>
  );
};

export default Layout;
