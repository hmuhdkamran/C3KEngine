import { FC, useState } from "react";
import ServiceDropdown from "@/components/extra/service-dropdown";
import MenuItems from "@/components/extra/menu-items";
import ProfileDropdown from "@/components/extra/profile-dropdown";
import NotificationDropdown from "@/components/extra/notification-dropdown";
import { useSystemContext } from "@/plugins/store";
import SidebarMenu from "@/components/extra/sidebar-menu";

const Header: FC = () => {
  const { services, menuItems } = useSystemContext();

  const [showModuleDropdown, setShowModuleDropdown] = useState(false);
  const toggleModuleDropdown = () => setShowModuleDropdown((prev) => !prev);

  const [showProfileDropdown, setShowProfileDropdown] = useState(false);
  const toggleProfileDropdown = () => setShowProfileDropdown((prev) => !prev);

  const [showNotificationDropdown, setNotificationDropdown] = useState(false);
  const toggleNotifications = () => setNotificationDropdown((prev) => !prev);

  const [sidebarDropdown, setSidebarDropdown] = useState(false);
  const toggleSidebarDropdown = () => setSidebarDropdown((prev) => !prev);

  return (
    <>
      <div className="fixed top-0 left-0 w-full px-4 sm:px-6 lg:px-8 flex items-center justify-between h-11 bg-gradient-to-r from-violet-600 to-indigo-500 shadow-[0_4px_10px_rgba(0,0,0,0.3)] z-50">
        <SidebarMenu
          showSidebarDropdown={sidebarDropdown}
          onToggleSidebar={toggleSidebarDropdown}
        />
        <div className="flex items-center space-x-2 sm:space-x-4">
          <button
            onClick={toggleSidebarDropdown}
            className="p-1 text-white hover:text-gray-300 focus:outline-none"
          >
            <span className="icon-[fluent--navigation-unread-20-filled] h-6 w-6"></span>
          </button>

          <ServiceDropdown
            services={services}
            showDropdown={showModuleDropdown}
            toggleDropdown={toggleModuleDropdown}
            selectedCardTitle={showModuleDropdown ? "Dashboard" : undefined}
          />
        </div>

        <div className="flex-grow flex justify-center">
          <MenuItems items={menuItems} />
        </div>

        <div className="flex items-center space-x-2">
          <NotificationDropdown
            showDropdown={showNotificationDropdown}
            toggleDropdown={toggleNotifications}
          />
          <button className="p-1 text-white hover:text-gray-300 focus:outline-none">
            <span className="icon-[ic--baseline-chat] h-4 w-4 sm:h-5 sm:w-5"></span>
          </button>
          <button className="p-1 text-white hover:text-gray-300 focus:outline-none">
            <span className="icon-[ic--baseline-help] h-4 w-4 sm:h-5 sm:w-5"></span>
          </button>
          <button className="p-1 text-white hover:text-gray-300 focus:outline-none">
            <span className="icon-[ic--baseline-settings] h-4 w-4 sm:h-5 sm:w-5"></span>
          </button>
          <ProfileDropdown
            showDropdown={showProfileDropdown}
            toggleDropdown={toggleProfileDropdown}
          />
        </div>
      </div>
    </>
  );
};

export default Header;
