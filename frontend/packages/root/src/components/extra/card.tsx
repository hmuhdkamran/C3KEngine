import React from "react";

interface CardProps {
  title?: string;
  description?: string;
  status?: string;
  buttonText?: string;
  iconClass?: string;
  containerClass?: string;
  showHeader?: boolean;
  showFooter?: boolean;
  children?: React.ReactNode;
  headerSlot?: React.ReactNode;
  footerSlot?: React.ReactNode;
  onClick?: () => void; // Added the onClick prop
  borderColor?: string;
}

const Card: React.FC<CardProps> = ({
  title,
  description,
  status,
  buttonText,
  iconClass,
  containerClass = "max-w-sm rounded-md overflow-hidden shadow-lg bg-white sm:max-w-full lg:max-w-sm relative group border border-gray-200 border-l-4 border-l-violet-500",
  showHeader = true,
  showFooter = true,
  headerSlot,
  footerSlot,
  children,
  onClick,
  borderColor = "#7c3aed",
}) => {
  return (
    <div
      className={`transition duration-300 transform hover:translate-y-[-2px] hover:shadow-xl ${containerClass}`}
      onClick={onClick}
      style={{
        borderColor: "#e2e8f0",
        borderLeftWidth: "3px",
        borderLeftColor: borderColor,
      }}
    >
      {showHeader && (
        <div>
          {headerSlot ? (
            headerSlot
          ) : (
            <div className="px-4 py-2 flex items-center gap-3 border-b border-gray-100 bg-gray-50">
              <div className="relative w-12 h-12 bg-violet-200 rounded-full flex items-center justify-center transition-all duration-500 ease-in-out">
                <span
                  className={`${iconClass} text-violet-700 text-lg`} />
              </div>
              <div className="flex-1">
                <div className="font-bold text-md flex items-center justify-between">
                  <span>{title}</span>
                  <span className="icon-[ph--dots-three-vertical-bold] cursor-pointer" />
                </div>
                <p className="text-gray-500 text-sm">{description}</p>
              </div>
            </div>
          )}
        </div>
      )}

      {children}

      {showFooter && (
        <div>
          {footerSlot ? (
            footerSlot
          ) : (
            <div className="px-4 py-2 text-green-500 text-sm flex justify-between items-center border-t border-gray-100">
              <span>{status}</span>
              <button className="bg-violet-100 hover:bg-violet-500 text-violet-700 hover:text-white py-1 px-3 rounded text-sm transition-colors duration-300">
                {buttonText}
              </button>
            </div>
          )}
        </div>
      )}
    </div>
  );
};

export default Card;
