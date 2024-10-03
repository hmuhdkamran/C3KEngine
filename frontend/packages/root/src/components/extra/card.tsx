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
}

const Card: React.FC<CardProps> = ({
  title,
  description,
  status,
  buttonText,
  iconClass,
  containerClass = "max-w-sm rounded overflow-hidden shadow-lg bg-white transition transform hover:scale-105 duration-200 sm:max-w-full lg:max-w-sm",
  showHeader = true,
  showFooter = true,
  headerSlot,
  footerSlot,
  children,
  onClick,
}) => {
  return (
    <div
      className={`transition transform shadow-lg hover:scale-105 duration-200 ${containerClass}`}
      onClick={onClick}
    >
      {showHeader && (
        <div>
          {headerSlot ? (
            headerSlot
          ) : (
            <div className="px-2 py-1 flex items-center gap-3">
              <div className="relative w-10 h-10 bg-violet-200 rounded-full">
                <span
                  className={`${iconClass} absolute w-6 h-6 text-violet-700 transform -translate-x-1/2 -translate-y-1/2 left-1/2 top-1/2`}
                />
              </div>
              <div className="flex-1">
                <div className="font-bold text-xl mb-1 flex items-center justify-between">
                  <span>{title}</span>
                  <span className="icon-[ph--dots-three-vertical-bold]" />
                </div>
                <p className="text-gray-500">{description}</p>
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
            <div className="px-2 py-1 text-green-500 text-sm flex justify-between">
              <span>{status}</span>
              <button className="bg-transparent hover:bg-violet-500 text-violet-700 hover:text-white py-1 px-2 rounded">
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
