import { FC } from "react";
import PageFilter from "./page-filter";
import { HeaderAreaProps } from "../../types/models";
import Breadcrumb from "./bread-crumb";
import { useSelector } from "react-redux";
import { RootState } from "../../plugins/store";

const HeaderArea: FC<HeaderAreaProps> = ({
  goToMain,
  children,
}) => {
  // Access page state from the Redux store
  const { pageTitle } = useSelector((state: RootState) => state.system.pageState);

  return (
    <div className="border-b border-gray-300 py-2 px-4 flex flex-col md:flex-row justify-between shadow-md w-full space-y-4 md:space-y-0 md:space-x-8">
      <div className="w-full md:w-1/2 flex flex-col justify-center space-y-6">
        <div className="px-3">
          <h1 className="text-2xl font-bold text-gray-800">{pageTitle}</h1>
        </div>
        <Breadcrumb homeClick={goToMain} />
      </div>
      <div className="w-full md:w-1/2 flex flex-col space-y-2">
        <PageFilter>{children}</PageFilter>
      </div>
    </div>
  );
};

export default HeaderArea;
