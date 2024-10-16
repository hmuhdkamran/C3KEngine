import { FC, useRef, useEffect } from "react";

interface NotificationDropdownProps {
  showDropdown: boolean;
  toggleDropdown: () => void;
}

const NotificationDropdown: FC<NotificationDropdownProps> = ({
  showDropdown,
  toggleDropdown,
}) => {
  const dropdownRef = useRef<HTMLDivElement | null>(null);

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
    <div
      className="relative"
      ref={dropdownRef}
      onClick={(e) => e.stopPropagation()}
    >
      <button
        className="btn btn-ghost btn-circle"
        onClick={toggleDropdown}
      >
        <div className="relative flex items-center p-2 text-gray-600 hover:text-gray-800 focus:outline-none">
          <span className="icon-[mdi--bell-outline] text-white h-4 w-4 sm:h-5 sm:w-5"></span>
          <span className="absolute top-0 right-0 block w-4 h-4 text-xs text-white bg-red-600 rounded-full">
            4
          </span>
        </div>
      </button>

      {showDropdown && (
        <div
          className={`absolute mt-2 w-64 sm:w-80 bg-white shadow-lg rounded-md overflow-hidden z-50 sm:right-auto sm:left-0 md:right-auto md:left-0 ${
            showDropdown ? "left-1/2 transform -translate-x-1/2 md:left-1/2 md:transform -translate-x-1/2 sm:left-auto" : ""
          }`} 
          style={{ maxWidth: '90vw' }}
        >
          <div className="py-1">
            <div className="px-4 py-2 bg-gray-100 border-b border-gray-200">
              <h3 className="text-sm font-semibold text-gray-800">
                Notifications
              </h3>
            </div>
            <div className="max-h-60 sm:max-h-80 overflow-y-auto">
              {[
                {
                  icon: "icon-[mdi--email]",
                  message: "You have a new message from John Doe",
                  time: "2 minutes ago",
                },
                {
                  icon: "icon-[mdi--bell-ring]",
                  message: "Reminder: Meeting at 3 PM",
                  time: "10 minutes ago",
                },
                {
                  icon: "icon-[mdi--file-document]",
                  message: "New report available: Sales Summary",
                  time: "30 minutes ago",
                },
                {
                  icon: "icon-[mdi--account]",
                  message: "Jane Smith updated her profile",
                  time: "1 hour ago",
                },
              ].map((notification, index) => (
                <div
                  key={index}
                  className="px-3 py-2 flex items-start space-x-2 border-b border-gray-200"
                >
                  <span
                    className={`${notification.icon} text-violet-500 h-4 w-4`}
                  ></span>
                  <div className="flex-1">
                    <p className="text-sm text-gray-700">
                      {notification.message}
                    </p>
                    <span className="text-xs text-gray-500">
                      {notification.time}
                    </span>
                  </div>
                </div>
              ))}
            </div>
          </div>
          <div className="px-3 py-2 bg-gray-100 text-center">
            <button className="text-sm text-blue-500 hover:underline">
              View All
            </button>
          </div>
        </div>
      )}
    </div>
  );
};

export default NotificationDropdown;
