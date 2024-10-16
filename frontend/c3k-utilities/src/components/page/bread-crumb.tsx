import React from 'react';
import { Link } from 'react-router-dom';
import { BreadcrumbProps } from '../../types/models';

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
          <Link to={item.route}>
            <div className="hover:underline cursor-pointer text-gray-600">
              {item.title}
            </div>
          </Link>
        </React.Fragment>
      ))}
    </nav>
  );
};

export default Breadcrumb;
