import { FC, useState, useRef, useEffect } from "react";
import { Link } from "react-router-dom";
import { MenuProps } from "../../types/models";

const MenuItems: FC<MenuProps> = ({ items }) => {
  const [openIndex, setOpenIndex] = useState<number | null>(null);
  const dropdownRef = useRef<HTMLDivElement | null>(null);

  const toggleDropdown = (index: number) => {
    setOpenIndex((prevIndex) => (prevIndex === index ? null : index));
  };

  const handleClickOutside = (event: MouseEvent) => {
    if (
      dropdownRef.current &&
      !dropdownRef.current.contains(event.target as Node)
    ) {
      setOpenIndex(null);
    }
  };

  useEffect(() => {
    document.addEventListener("mousedown", handleClickOutside);

    return () => {
      document.removeEventListener("mousedown", handleClickOutside);
    };
  }, []);

  return (
    <nav className="flex space-x-4">
      {items.map((item, index) => {
        const hasChildren = item.children && item.children.length > 0;

        return (
          <div key={index} className="relative" ref={dropdownRef}>
            {hasChildren ? (
              <div>
                <button
                  onClick={() => toggleDropdown(index)}
                  className="flex items-center text-white text-sm hover:text-gray-200 transition duration-200 focus:outline-none"
                >
                  {item.icon && <span className={`mr-2 ${item.icon}`}></span>}
                  {item.name}
                  <span className="ml-2 icon-[ri--arrow-drop-down-line]"></span>{" "}
                </button>
                {openIndex === index && (
                  <div className="absolute mt-2 w-44 bg-white shadow-lg rounded-md overflow-hidden z-50 left-0">
                    {item.children?.map((child, childIndex) =>
                      child.path ? (
                        <Link
                          key={childIndex}
                          to={child.path}
                          className="block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100"
                        >
                          {child.icon && (
                            <span className={`mr-2 ${child.icon}`}></span>
                          )}
                          {child.name}
                        </Link>
                      ) : null
                    )}
                  </div>
                )}
              </div>
            ) : (
              item.path && (
                <Link
                  to={item.path}
                  className="flex items-center text-white text-sm hover:text-gray-200 transition duration-200"
                >
                  {item.icon && <span className={`mr-2 ${item.icon}`}></span>}
                  {item.name}
                </Link>
              )
            )}
          </div>
        );
      })}
    </nav>
  );
};

export default MenuItems;
