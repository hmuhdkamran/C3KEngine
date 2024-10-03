import { FC, useEffect, useMemo } from "react";
import HeaderArea from "@/components/page/header-area";
import {
  useDataContext,
  usePageContext,
  useSystemContext,
} from "@/plugins/store";
import Card from "@/components/extra/card";

const DashboardIndex: FC = () => {
  const { pageTitle, updatePageState } = usePageContext();
  const { currentPage, itemsPerPage } = useDataContext();
  const { services } = useSystemContext();

  useEffect(() => {
    const newState = {
      pageTitle: "Dashboard",
      breadcrumbItems: [
        { title: "Human Resource Management", route: "/", icon: "home" },
      ],
    };
    updatePageState(newState);
  }, []);

  const paginatedFilteredCards = useMemo(() => {
    const start = (currentPage - 1) * itemsPerPage;
    const end = start + itemsPerPage;
    return services.slice(start, end);
  }, [services, currentPage, itemsPerPage]);

  // Handle card click, capturing both card title and index
  const handleCardClick = (title: string, index: number) => {
    console.log(`Card clicked: ${title} at index: ${index}`);
  };

  const goToMain = () => {
    console.log("Going to main page");
  };

  const toggleFilters = () => {
    console.log("Toggling filters");
  };

  const groupByCategory = () => {
    console.log("Grouping by category");
  };

  const toggleFavorites = () => {
    console.log("Toggling favorites");
  };

  return (
    <>
      <div className="bg-white mt-12 flex flex-col">
        <HeaderArea pageHeading={pageTitle} goToMain={goToMain}>
          <div className="flex space-x-2">
            <button
              onClick={toggleFilters}
              className="flex items-center bg-violet-600 text-white hover:bg-violet-700 transition px-2 py-1 rounded-sm shadow-md"
            >
              <i className="icon-[fluent--filter-16-filled] mr-1"></i> Filters
            </button>
            <button
              onClick={groupByCategory}
              className="flex items-center bg-gray-200 text-gray-600 hover:bg-gray-300 transition px-2 py-1 rounded-sm shadow-md"
            >
              <i className="icon-[fluent--group-24-filled] mr-1"></i> Group By
            </button>
            <button
              onClick={toggleFavorites}
              className="flex items-center bg-red-500 text-white hover:bg-red-600 transition px-2 py-1 rounded-sm shadow-md"
            >
              <i className="icon-[mdi--star-outline] mr-1"></i> Favorites
            </button>
          </div>
        </HeaderArea>

        <div className="bg-gray-50 flex-1 mx-auto mt-4 py-6 px-6 w-full h-full">
          <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6">
            {paginatedFilteredCards.map((card, index) => (
              <Card
                key={index}
                title={card.name}
                description={card.description}
                status="active"
                buttonText="Proceed"
                iconClass=""
                onClick={() => handleCardClick(card.name, index)}
              />
            ))}
          </div>
        </div>
      </div>
    </>
  );
};

export default DashboardIndex;
