import { FC } from "react";
import { Outlet } from "react-router-dom";
import Header from "./auth/header";
import Footer from "./auth/footer";
import { DataProvider, PageProvider } from "c3k-utilities";

const AuthenticatedLayout: FC = () => {
  return (
    <div className="text-blueGray-700 antialiased">
      <DataProvider>
        <PageProvider>
          <Header />
          <Outlet />
          <Footer />
        </PageProvider>
      </DataProvider>
    </div>
  );
};

export default AuthenticatedLayout;
