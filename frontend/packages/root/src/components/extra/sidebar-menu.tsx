import { FC, useState, useEffect } from "react";
import { NavLink } from "react-router-dom";
import { useSystemContext } from "@/plugins/store";
import VNodeRenderer from "./node-renderer";
import { config } from "@/plugins/config";

interface SidebarProps {
  showSidebarDropdown: boolean;
  onToggleSidebar: () => void;
}

const SidebarMenu: FC<SidebarProps> = ({
  showSidebarDropdown,
  onToggleSidebar,
}) => {
  const { sidebarMenu } = useSystemContext();
  const [openSection, setOpenSection] = useState<number | null>(null);
  const [sidebarVisible, setSidebarVisible] = useState(showSidebarDropdown);

  useEffect(() => {
    if (showSidebarDropdown) {
      setSidebarVisible(true);
    }
  }, [showSidebarDropdown]);

  const handleTransitionEnd = () => {
    if (!showSidebarDropdown) {
      setSidebarVisible(false);
    }
  };

  const toggleSection = (index: number) => {
    setOpenSection((prevIndex) => (prevIndex === index ? null : index));
  };

  return (
    <>
      {/* Sidebar container */}
      {sidebarVisible && (
        <div
          className={`fixed inset-y-0 left-0 w-64 bg-white text-gray-800 border-r border-gray-200 shadow-lg z-50 rounded-tr-lg rounded-br-lg transform ${
            showSidebarDropdown ? "translate-x-0" : "-translate-x-full"
          } transition-transform duration-300 ease-in-out`}
          onTransitionEnd={handleTransitionEnd}
        >
          <div className="flex flex-col h-full">
            <div className="flex items-center justify-between px-3 py-4 bg-white border-b border-gray-200 shadow-sm">
              <a className="flex items-center space-x-2">
                <VNodeRenderer nodes={config.logo} />
                <span className="text-sm font-semibold text-gray-900">
                  {config.application}
                </span>
              </a>
              <button
                onClick={onToggleSidebar}
                className="text-gray-400 hover:text-gray-600 focus:outline-none"
              >
                <span className="icon-[fluent--dismiss-20-filled] h-4 w-4"></span>
              </button>
            </div>
            <div className="overflow-y-auto flex-grow px-4 py-3">
              <ul className="space-y-2">
                {sidebarMenu.map((section, index) => (
                  <li key={index} className="border-b last:border-none pb-1">
                    <div
                      onClick={() => toggleSection(index)}
                      className="flex items-center justify-between px-3 py-2 rounded-md cursor-pointer bg-gray-50 hover:text-violet-700 hover:bg-violet-50 transition-all duration-200 ease-in-out"
                    >
                      <span className="flex items-center space-x-2">
                        <span
                          className={`${section.icon} h-5 w-5 text-violet-600`}
                        ></span>
                        <span className="text-sm font-semibold text-gray-700">
                          {section.name}
                        </span>
                      </span>
                      <span
                        className={
                          openSection === index
                            ? "icon-[mdi--chevron-up] text-gray-600"
                            : "icon-[mdi--chevron-down] text-gray-500"
                        }
                      ></span>
                    </div>
                    <div
                      className={`overflow-hidden transition-max-height duration-300 ease-in-out ${
                        openSection === index ? "max-h-screen" : "max-h-0"
                      }`}
                    >
                      <ul className="mt-2 pl-4 space-y-1">
                        {section.children &&
                          section.children.map((item, subIndex) => (
                            <li key={subIndex}>
                              <NavLink
                                to={item.path || "#"}
                                className="flex items-center space-x-2 py-1 px-3 rounded-md text-sm text-gray-600 hover:text-violet-700 hover:bg-violet-50 hover:border-l-2 border-violet-500 transition-all duration-200 ease-in-out"
                              >
                                <span>{item.name}</span>
                              </NavLink>
                            </li>
                          ))}
                      </ul>
                    </div>
                  </li>
                ))}
              </ul>
            </div>
            <div className="px-4 py-3 bg-white border-t border-gray-200 shadow-sm mt-auto">
              <NavLink
                to="/profile"
                className="flex items-center justify-center btn-primary py-2 space-x-2 text-sm"
              >
                <span className="icon-[ic--baseline-person] h-4 w-4"></span>
                <span>Profile</span>
              </NavLink>
              <NavLink
                to="/logout"
                className="flex items-center justify-center btn-secondary py-2 mt-3 space-x-2 text-sm"
              >
                <span className="icon-[ic--baseline-logout] h-4 w-4"></span>
                <span>Logout</span>
              </NavLink>
            </div>
          </div>
        </div>
      )}
      {showSidebarDropdown && (
        <div
          className="fixed inset-0 bg-black bg-opacity-50 z-40"
          onClick={onToggleSidebar}
        ></div>
      )}
    </>
  );
};

export default SidebarMenu;
