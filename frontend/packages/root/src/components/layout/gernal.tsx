import { FC } from "react";
import { Outlet } from "react-router-dom";
import Header from "./gernal/header";
import Footer from "./gernal/footer";

const GernalLayout: FC = () => {
  return (
    <div className="text-blueGray-700 antialiased">
      <Header />
      <Outlet />
      <Footer />
    </div>
  );
};

export default GernalLayout;
