import { FC, useContext } from "react";
import { VNodeRenderer, SystemContext } from "c3k-utilities";
import { Link } from "react-router-dom";
import { config } from "@/plugins/config";

const Header: FC = () => {
  const context = useContext(SystemContext);
  const isAuthenticated = context?.user.authenticated;

  return (
    <nav className="fixed z-50 w-full bg-white top-0 flex flex-wrap items-center justify-between px-1 py-1 shadow-lg">
      <div className="container px-4 mx-auto flex flex-wrap items-center justify-between">
        <div className="w-full relative flex justify-between lg:w-auto lg:static lg:block lg:justify-start">
          <a
            className="text-sm font-bold leading-relaxed inline-block mr-4 py-2 whitespace-nowrap uppercase text-gray-700"
            href="/"
          >
            <VNodeRenderer nodes={config.logo} /> {config.application}
          </a>
          <button
            className="cursor-pointer text-xl leading-none px-3 py-1 border border-solid border-transparent rounded bg-transparent block lg:hidden outline-none focus:outline-none"
            type="button"
          >
            <span className="icon-[fa-solid--bars]"></span>
          </button>
        </div>

        <div
          className="lg:flex flex-grow items-center hidden"
          id="example-navbar-danger"
        >
          <ul className="flex flex-col lg:flex-row list-none lg:ml-auto gap-4">
            {config.socialMedia.map((item, index) => (
              <li className="nav-item" key={index}>
                <a
                  href={item.link}
                  className="btn-rounded"
                  target="_blank"
                  rel="noopener noreferrer"
                >
                  <span className={item.icon}></span>
                </a>
              </li>
            ))}
            <li className="nav-item cursor-pointer">
              {isAuthenticated ? (
                <Link to="/dashboard" className="btn-gradient px-4 py-2">
                  Dashboard
                </Link>
              ) : (
                <Link to="/auth/login" className="btn-gradient px-4 py-2">
                  Login
                </Link>
              )}
            </li>
          </ul>
        </div>
      </div>
    </nav>
  );
};

export default Header;
