import { FC, useState, useMemo, useRef, useEffect } from "react";
import { useNavigate } from "react-router-dom";

export interface IService {
  name: string;
  icon: string;
  description: string;
  route: string;
}

export interface ServiceDropdownProps {
  services: IService[];
  selectedCardTitle?: string;
  showDropdown: boolean;
  toggleDropdown: () => void;
}

const ServiceDropdown: FC<ServiceDropdownProps> = ({
  services,
  showDropdown,
  selectedCardTitle,
  toggleDropdown,
}) => {
  const [searchQuery, setSearchQuery] = useState("");
  const navigate = useNavigate();
  const dropdownRef = useRef<HTMLDivElement | null>(null);

  const filteredServices = useMemo(() => {
    return services.filter((service) =>
      service.name.toLowerCase().includes(searchQuery.toLowerCase())
    );
  }, [searchQuery, services]);

  const handleServiceClick = (route: string) => {
    navigate(route);
    toggleDropdown();
  };

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (
        dropdownRef.current &&
        !dropdownRef.current.contains(event.target as Node)
      ) {
        toggleDropdown();
      }
    };

    if (showDropdown) {
      document.addEventListener("mousedown", handleClickOutside);
    }

    return () => {
      document.removeEventListener("mousedown", handleClickOutside);
    };
  }, [showDropdown, toggleDropdown]);

  return (
    <div className="relative">
      <button
        onClick={toggleDropdown}
        className="flex items-center space-x-2 text-white hover:text-gray-300 focus:outline-none"
      >
        <span>{!selectedCardTitle ? "Dashboard" : selectedCardTitle}</span>
        <span className="icon-[ri--arrow-drop-down-line] text-white h-7 w-7"></span>
      </button>

      {showDropdown && (
        <div ref={dropdownRef}
          className="absolute left-0 mt-4 w-[75vw] sm:w-[600px] bg-white rounded-lg shadow-lg z-50 sm:p-4 
          grid grid-cols-1 sm:grid-cols-2 gap-2 sm:gap-3 transform origin-top scale-95 transition-transform duration-300"
        >
          <div className="col-span-full mb-2 relative">
            <input
              type="text"
              placeholder="Search Modules..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="input-bottom"
            />
          </div>

          {filteredServices.length ? (
            filteredServices.map((service, index) => (
              <div
                key={index}
                className="flex items-center cursor-pointer bg-gray-50 border border-gray-200 rounded-sm border-l-4 p-1 sm:p-4 hover:bg-gray-100 hover:shadow-md transition-all duration-300 relative"
                onClick={() => handleServiceClick(service.route)}
              >
                <span
                  className={`${service.icon} text-violet-600 h-5 w-5 sm:h-8 sm:w-8 mr-4`}
                ></span>
                <div>
                  <h4 className="text-sm sm:text-md font-semibold text-gray-700">
                    {service.name}
                  </h4>
                  <p className="text-xs sm:text-sm text-gray-500">
                    {service.description}
                  </p>
                </div>
                {service.name === "HRMS Service" && (
                  <span className="absolute top-2 right-2 text-xs text-white bg-red-500 rounded-full px-1 py-1">
                    New
                  </span>
                )}
              </div>
            ))
          ) : (
            <div className="text-sm col-span-full text-center text-gray-500">
              <p>No modules found.</p>
            </div>
          )}
        </div>
      )}
    </div>
  );
};

export default ServiceDropdown;
