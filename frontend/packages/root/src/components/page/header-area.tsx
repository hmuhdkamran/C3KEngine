import { FC, ReactNode } from "react";
import Breadcrumb from "@/components/page/bread-crumb";
import PageFilter from "./page-filter";
import { usePageContext } from "@/plugins/store";

interface HeaderAreaProps {
  pageHeading: string;
  goToMain: () => void;
  children?: ReactNode;
}

const HeaderArea: FC<HeaderAreaProps> = ({
  pageHeading,
  goToMain,
  children,
}) => {
  const { breadcrumbItems } = usePageContext();

  return (
    <div className="border-b border-gray-300 py-2 px-4 flex flex-col md:flex-row justify-between shadow-md w-full space-y-4 md:space-y-0 md:space-x-8">
      <div className="w-full md:w-1/2 flex flex-col justify-center space-y-6">
        <div className="px-3">
          <h1 className="text-2xl font-bold text-gray-800">{pageHeading}</h1>
        </div>
        <Breadcrumb items={breadcrumbItems} homeClick={goToMain} />
      </div>
      <div className="w-full md:w-1/2 flex flex-col space-y-2">
        <PageFilter>{children}</PageFilter>
      </div>
    </div>
  );
};

export default HeaderArea;
