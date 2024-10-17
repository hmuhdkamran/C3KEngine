import { ChangeEvent, FC } from "react";
import Paginator from "../data/paginator";
import { useSelector, useDispatch } from "react-redux";
import { RootState } from "../../plugins/store";
import { updateSearchQuery } from "../../plugins/store/dataSlice";
import { PageFilterProps } from "../../types/models";

const PageFilter: FC<PageFilterProps> = ({ children }) => {
  const dispatch = useDispatch();
  // Access search query from the Redux store
  const { searchQuery } = useSelector((state: RootState) => state.data);

  const handleSearchChange = (event: ChangeEvent<HTMLInputElement>) => {
    dispatch(updateSearchQuery(event.target.value));
  };

  return (
    <div className="flex flex-col space-y-2">
      <div className="relative">
        <i className="icon-[mdi--magnify] absolute left-2 top-3 text-gray-400"></i>
        <input
          type="text"
          placeholder="Search..."
          className="input-bottom pl-6 w-full"
          value={searchQuery}
          onChange={handleSearchChange}
        />
      </div>
      <div className="flex flex-wrap items-center justify-between lg:space-x-96 sm:space-x-80 md:space-x-12">
        <div className="flex justify-start space-x-1 items-center text-sm">
          {children}
        </div>
        <Paginator />
      </div>
    </div>
  );
};

export default PageFilter;
