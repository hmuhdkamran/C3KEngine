import { FC } from "react";
import { Outlet } from "react-router-dom";
import Header from "./gernal/header";
import Footer from "./gernal/footer";
import { CSSTransition, SwitchTransition } from "react-transition-group";

const GernalLayout: FC = () => {
  return (
    <div className="text-blueGray-700 antialiased">
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
    </div>
  );
};

export default GernalLayout;
