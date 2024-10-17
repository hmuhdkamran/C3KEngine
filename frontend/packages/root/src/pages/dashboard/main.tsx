import { Card, HeaderArea, RootState, updatePageState } from "c3k-utilities";
import { FC, useEffect, useMemo, useState } from "react";
import { useSelector } from "react-redux";

const DashboardIndex: FC = () => {
  const { currentPage, itemsPerPage } = useSelector(
    (state: RootState) => state.data
  );
  const { services } = useSelector((state: RootState) => state.system);

  const [selectedCategory, setSelectedCategory] = useState("All");
  const [showModulePage, setShowModulePage] = useState(false);
  const [selectedCardTitle, setSelectedCardTitle] = useState("");

  useEffect(() => {
    const breadcrumbItems = [{ title: "Dashboard", route: "#", icon: "home" }];

    if (selectedCategory !== "All") {
      breadcrumbItems.push({
        title: selectedCategory,
        route: "#",
        icon: "folder",
      });
    }

    if (showModulePage && selectedCardTitle) {
      breadcrumbItems.push({
        title: selectedCardTitle,
        route: "#",
        icon: "file",
      });
    }

    updatePageState({
      pageTitle: showModulePage ? selectedCardTitle : "Dashboard",
      breadcrumbItems,
    });
  }, [selectedCategory, showModulePage, selectedCardTitle, updatePageState]);

  const filteredCards = useMemo(() => {
    return services.filter((card) => {
      return selectedCategory === "All" || card.category === selectedCategory;
    });
  }, [services, selectedCategory]);

  const paginatedFilteredCards = useMemo(() => {
    const start = (currentPage - 1) * itemsPerPage;
    const end = start + itemsPerPage;
    return filteredCards.slice(start, end);
  }, [filteredCards, currentPage, itemsPerPage]);

  const categories = [
    "All",
    ...new Set(services.map((service) => service.category || "")),
  ];

  const handleCategoryClick = (category: string) => {
    setSelectedCategory(category);
    setShowModulePage(false);
    // setSelectedCardTitle("");
  };

  const handleCardClick = (title: string) => {
    setShowModulePage(true);
    setSelectedCardTitle(title);
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
        <HeaderArea goToMain={goToMain}>
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

        <div className="flex flex-row">
          <div className="h-screen border-r border-gray-300 w-64 p-4 hidden sm:block">
            <div className="text-md font-semibold mb-2">
              <span className="icon-[ion--folder-sharp] text-violet-600"></span>
              <span className="text-violet-600"> CATEGORIES </span>
            </div>
            <ul className="px-2 text-sm">
              {categories.map((category) => (
                <li
                  key={category}
                  className={`mb-1 hover:bg-violet-100 hover:rounded-md py-1 px-3 cursor-pointer ${
                    selectedCategory === category
                      ? "bg-violet-100 text-violet-700 rounded-md"
                      : ""
                  }`}
                  onClick={() => {
                    if (category) {
                      handleCategoryClick(category);
                    }
                  }}
                >
                  <span>{category}</span>
                </li>
              ))}
            </ul>
          </div>

          <div className="bg-gray-100 flex-1 mx-auto py-6 px-6 min-h-screen">
            <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6">
              {paginatedFilteredCards.map((card, index) => (
                <Card
                  key={index}
                  title={card.name}
                  description={card.description}
                  status="active"
                  buttonText="Proceed"
                  iconClass={card.icon}
                  onClick={() => handleCardClick(card.name)}
                />
              ))}
            </div>
          </div>
        </div>
      </div>
    </>
  );
};

export default DashboardIndex;
