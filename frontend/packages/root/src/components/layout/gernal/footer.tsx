import { FC } from "react";
import { config } from "@/plugins/config";
import { VNodeRenderer } from "c3k-utilities";

const Footer: FC = () => {
  return (
    <footer className="relative bg-gray-200 pt-8 pb-6">
      <div className="bottom-auto top-0 left-0 right-0 w-full absolute pointer-events-none overflow-hidden -mt-20">
        <svg
          className="absolute bottom-0 overflow-hidden"
          xmlns="http://www.w3.org/2000/svg"
          preserveAspectRatio="none"
          version="1.1"
          viewBox="0 0 2560 100"
          x="0"
          y="0"
        >
          <polygon
            className="text-gray-200 fill-current"
            points="2560 0 2560 100 0 100"
          ></polygon>
        </svg>
      </div>
      <div className="container mx-auto px-4">
        <div className="grid grid-cols-1 gap-8 lg:grid-cols-3">
          <div>
            <div className="flex justify-center sm:justify-start">
              <h4 className="text-3xl font-semibold">
                <VNodeRenderer nodes={config.logo} />
                {config.application}
              </h4>
            </div>
            <p className="mx-auto mt-6 max-w-md text-center leading-relaxed text-gray-500 sm:mx-0 sm:max-w-xs sm:text-left">
              Find us on any of these platforms, we respond 1-2 business days.
            </p>
            <ul className="mt-8 flex justify-center gap-4 sm:justify-start">
              {config.socialMedia.map((item, index) => (
                <li className="nav-item" key={index}>
                  <a
                    href={item.link}
                    className="btn-rounded inline-flex items-center"
                    target="_blank"
                    rel="noopener noreferrer"
                  >
                    <span className={item.icon}></span>
                  </a>
                </li>
              ))}
            </ul>
          </div>

          <div className="grid grid-cols-1 gap-8 sm:grid-cols-2 md:grid-cols-4 lg:col-span-2">
            <div className="text-center sm:text-left">
              <p className="text-lg font-medium text-gray-900">About Us</p>

              <nav aria-label="Footer About Nav" className="mt-8">
                <ul className="space-y-4 text-sm">
                  <li>
                    <a
                      className="text-gray-700 transition hover:text-gray-700/75"
                      href="/"
                    >
                      Company History
                    </a>
                  </li>

                  <li>
                    <a
                      className="text-gray-700 transition hover:text-gray-700/75"
                      href="/"
                    >
                      Meet the Team
                    </a>
                  </li>

                  <li>
                    <a
                      className="text-gray-700 transition hover:text-gray-700/75"
                      href="/"
                    >
                      Employee Handbook
                    </a>
                  </li>

                  <li>
                    <a
                      className="text-gray-700 transition hover:text-gray-700/75"
                      href="/"
                    >
                      Careers
                    </a>
                  </li>
                </ul>
              </nav>
            </div>

            <div className="text-center sm:text-left">
              <p className="text-lg font-medium text-gray-900">Our Services</p>

              <nav aria-label="Footer Services Nav" className="mt-8">
                <ul className="space-y-4 text-sm">
                  <li>
                    <a
                      className="text-gray-700 transition hover:text-gray-700/75"
                      href="/"
                    >
                      Web Development
                    </a>
                  </li>

                  <li>
                    <a
                      className="text-gray-700 transition hover:text-gray-700/75"
                      href="/"
                    >
                      Web Design
                    </a>
                  </li>

                  <li>
                    <a
                      className="text-gray-700 transition hover:text-gray-700/75"
                      href="/"
                    >
                      Software Development
                    </a>
                  </li>

                  <li>
                    <a
                      className="text-gray-700 transition hover:text-gray-700/75"
                      href="/"
                    >
                      Google Ads
                    </a>
                  </li>
                </ul>
              </nav>
            </div>

            <div className="text-center sm:text-left">
              <p className="text-lg font-medium text-gray-900">Helpful Links</p>

              <nav aria-label="Footer Helpful Nav" className="mt-8">
                <ul className="space-y-4 text-sm">
                  <li>
                    <a
                      className="text-gray-700 transition hover:text-gray-700/75"
                      href="/"
                    >
                      FAQs
                    </a>
                  </li>

                  <li>
                    <a
                      className="text-gray-700 transition hover:text-gray-700/75"
                      href="/"
                    >
                      Support
                    </a>
                  </li>

                  <li>
                    <a
                      className="group flex justify-center gap-1.5 sm:justify-start"
                      href="/"
                    >
                      <span className="text-gray-700 transition group-hover:text-gray-700/75">
                        Live Chat
                      </span>

                      <span className="relative -mr-2 flex h-2 w-2">
                        <span className="absolute inline-flex h-full w-full animate-ping rounded-full bg-teal-400 opacity-75"></span>
                        <span className="relative inline-flex h-2 w-2 rounded-full bg-teal-500"></span>
                      </span>
                    </a>
                  </li>
                </ul>
              </nav>
            </div>

            <div className="text-center sm:text-left">
              <p className="text-lg font-medium text-gray-900">Contact Us</p>

              <ul className="mt-8 space-y-4 text-sm">
                <li>
                  <a
                    className="flex items-center justify-center gap-1.5 sm:justify-start"
                    href="/"
                  >
                    <span className="icon-[wpf--message-outline]"></span>
                    <span className="text-gray-700">admin@c3k.org</span>
                  </a>
                </li>

                <li>
                  <a
                    className="flex items-center justify-center gap-1.5 sm:justify-start"
                    href="/"
                  >
                    <span className="icon-[fluent--call-28-regular]"></span>
                    <span className="text-gray-700">+012 345 6789</span>
                  </a>
                </li>

                <li className="flex items-start justify-center gap-1.5 sm:justify-start">
                  <span className="icon-[ep--location]"></span>
                  <address className="-mt-0.5 not-italic text-gray-700">
                    8 Waris Road, Lahore, Pakistan
                  </address>
                </li>
              </ul>
            </div>
          </div>
        </div>

        <div className="mt-12 border-t border-gray-100 pt-6">
          <div className="text-center sm:flex sm:justify-between sm:text-left">
            <p className="text-sm text-gray-500">
              <span className="block sm:inline">All rights reserved.</span>

              <a
                className="inline-block text-violet-600 underline transition hover:text-violet-600/75"
                href="/"
              >
                Terms &amp; Conditions
              </a>

              <span>·</span>

              <a
                className="inline-block text-violet-600 underline transition hover:text-violet-600/75"
                href="/"
              >
                Privacy Policy
              </a>
            </p>
            <p className="mt-4 text-sm text-gray-500 sm:order-first sm:mt-0">
              Copyright © 2024 C3K Engine by{" "}
              <a
                href="https://combinesoft.net"
                className="text-gray-500 hover:text-gray-800"
                target="_blank"
              >
                COMBINE CHIPSoft
              </a>
              .
            </p>
          </div>
        </div>
      </div>
    </footer>
  );
};

export default Footer;
