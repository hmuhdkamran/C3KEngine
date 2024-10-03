import React from 'react';

export interface BreadcrumbItem {
  title: string;
  route: string;
  icon: string;
}

interface BreadcrumbProps {
  items: BreadcrumbItem[];
  homeClick: () => void;
}

const Breadcrumb: React.FC<BreadcrumbProps> = ({ items, homeClick }) => {
  return (
    <nav className="sm:text-md px-2 text-sm flex space-x-2 items-center">
      <div
        onClick={homeClick}
        className="hover:underline cursor-pointer text-gray-600 flex items-center"
      >
        <i className="icon-[mdi--home-outline] mr-1 text-gray-500"></i> Apps
      </div>

      {items.map((item, index) => (
        <React.Fragment key={index}>
          <span className="text-gray-400">/</span>
          <div
            onClick={() => (item.route ? null : null)}
            className="hover:underline cursor-pointer text-gray-600"
          >
            {item.title}
          </div>
        </React.Fragment>
      ))}
    </nav>
  );
};

export default Breadcrumb;
