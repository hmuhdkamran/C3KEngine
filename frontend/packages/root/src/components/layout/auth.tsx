import { FC } from "react";
import { Outlet } from "react-router-dom";
import Header from "./auth/header";
import Footer from "./auth/footer";
import { CSSTransition, SwitchTransition } from "react-transition-group";
import { DataProvider } from "@/plugins/store/data";
import { PageProvider } from "@/plugins/store/page";

const AuthenticatedLayout: FC = () => {
  return (
    <div className="text-blueGray-700 antialiased">
      <DataProvider>
        <PageProvider>
          <Header />
          <SwitchTransition mode="out-in">
            <CSSTransition
              key={window.location.pathname}
              classNames="fade"
              timeout={300}
            >
              <div>
                <Outlet />
              </div>
            </CSSTransition>
          </SwitchTransition>
          <Footer />
        </PageProvider>
      </DataProvider>
    </div>
  );
};

export default AuthenticatedLayout;
