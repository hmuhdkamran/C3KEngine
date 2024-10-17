import { FC, useRef, useEffect } from "react";
import { config } from "@/plugins/config";
import VNodeRenderer from "@/components/extra/node-renderer";
import { useSystemContext } from "@/plugins/store";

interface ProfileDropdownProps {
  showDropdown: boolean;
  toggleDropdown: () => void;
}

const ProfileDropdown: FC<ProfileDropdownProps> = ({
  showDropdown,
  toggleDropdown,
}) => {
  const dropdownRef = useRef<HTMLDivElement | null>(null);
  const { user } = useSystemContext();

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
    <div className="relative" ref={dropdownRef}>
      <button
        onClick={() => toggleDropdown()}
        className="flex items-center p-1 text-gray-600 hover:text-gray-800 focus:outline-none gap-1"
      >
        <VNodeRenderer nodes={config.logo} />
        <span className="text-sm text-white truncate md:whitespace-normal">
          {user.displayName}
        </span>
        <span className="icon-[ri--arrow-drop-down-line] text-white h-5 w-5"></span>
      </button>
      <div className={`absolute right-0 mt-2 w-48 bg-white shadow-lg rounded-md z-10 overflow-hidden transform transition-all duration-300 ease-in-out ${
          showDropdown ? "opacity-100 scale-100" : "opacity-0 scale-95 pointer-events-none"
        }`}
      >
        <div className="px-3 py-2 bg-gray-50 border-b border-gray-200">
          <div className="flex items-center">
            <VNodeRenderer nodes={config.logo} />
            <div className="ml-2">
              <p className="text-sm font-semibold text-gray-800">
                {user.displayName}
              </p>
              <p className="text-sm text-gray-600">{user.username}</p>
            </div>
          </div>
        </div>
        <div className="py-2">
          <ul className="space-y-2">
            <li>
              <a
                href="#"
                className="flex items-center px-3 py-1.5 text-gray-800 hover:bg-violet-100 rounded-sm"
              >
                <span className="icon-[ic--baseline-person] bg-violet-600 h-4 w-4 mr-2"></span>
                <span className="text-sm">Profile</span>
              </a>
            </li>
            <li>
              <a
                href="#"
                className="flex items-center px-3 py-1.5 text-gray-800 hover:bg-violet-100 rounded-sm"
              >
                <span className="icon-[ic--baseline-settings] text-violet-500 h-4 w-4 mr-2"></span>
                <span className="text-sm">Settings</span>
              </a>
            </li>
            <li>
              <a
                href="#"
                className="flex items-center px-3 py-1.5 text-gray-800 hover:bg-violet-100 rounded-sm"
              >
                <span className="icon-[ic--baseline-history] text-violet-500 h-4 w-4 mr-2"></span>
                <span className="text-sm">Activity Log</span>
              </a>
            </li>
            <li>
              <a
                href="#"
                className="flex items-center px-3 py-1.5 text-gray-800 hover:bg-violet-100 rounded-sm"
              >
                <span className="icon-[ic--baseline-help] text-violet-500 h-4 w-4 mr-2"></span>
                <span className="text-sm">Help Center</span>
              </a>
            </li>
          </ul>
          <hr className="my-1 border-gray-200" />
          <ul className="mt-1">
            <li>
              <a
                href="#"
                className="flex items-center px-3 py-1.5 text-gray-800 hover:bg-violet-100 rounded-sm"
              >
                <span className="icon-[ic--baseline-settings-applications] text-violet-500 h-4 w-4 mr-2"></span>
                <span className="text-sm">Account Settings</span>
              </a>
            </li>
            <li>
              <a
                href="#"
                className="flex items-center px-3 py-1.5 text-gray-800 hover:bg-violet-100 rounded-sm"
              >
                <span className="icon-[ic--baseline-logout] text-violet-500 h-4 w-4 mr-2"></span>
                <span className="text-sm">Logout</span>
              </a>
            </li>
          </ul>
        </div>
      </div>
    </div>
  );
};

export default ProfileDropdown;
