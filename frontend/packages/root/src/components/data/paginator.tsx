import { useDataContext } from "@/plugins/store";
import { FC } from "react";

const Paginator: FC = () => {
  const { currentPage, totalPages, setPage } = useDataContext();

  const goToFirstPage = () => setPage(1);
  const goToPreviousPage = () => setPage(currentPage - 1);
  const goToNextPage = () => setPage(currentPage + 1);
  const goToLastPage = () => setPage(totalPages);

  return (
    <div className="mt-6 flex flex-col md:flex-row justify-between items-center space-y-4 md:space-y-0">
      <div className="flex items-center space-x-2">
        <button
          onClick={goToFirstPage}
          disabled={currentPage === 1}
          className="text-gray-600 hover:text-indigo-600 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
        >
          <span className="icon-[material-symbols--first-page] w-5 h-5"></span>
        </button>
        <button
          onClick={goToPreviousPage}
          disabled={currentPage === 1}
          className="text-gray-600 hover:text-indigo-600 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
        >
          <span className="icon-[material-symbols--keyboard-arrow-left] w-5 h-5"></span>
        </button>
        <div className="text-gray-600 text-sm mb-2">
          <span className="font-bold">Page</span> {currentPage} of {totalPages}
        </div>
        <button
          onClick={goToNextPage}
          disabled={currentPage === totalPages}
          className="text-gray-600 hover:text-indigo-600 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
        >
          <span className="icon-[material-symbols--keyboard-arrow-right] w-5 h-5"></span>
        </button>
        <button
          onClick={goToLastPage}
          disabled={currentPage === totalPages}
          className="text-gray-600 hover:text-indigo-600 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
        >
          <span className="icon-[material-symbols--last-page] w-5 h-5"></span>
        </button>
      </div>
    </div>
  );
};

export default Paginator;
