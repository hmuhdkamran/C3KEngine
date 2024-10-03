import { FC } from "react";
import { config } from "@/plugins/config";

import image1 from "@/assets/images/image1.jpg";
import image2 from "@/assets/images/image2.jpg";
import image3 from "@/assets/images/image3.jpg";
import image4 from "@/assets/images/image4.jpg";

const WhatWeProvide: FC = () => {
  return (
    <>
      <section>
        <div className="relative p-4 px-6 mx-auto bg-gray-50 dark:bg-gray-800 sm:px-6 md:px-8 lg:px-8 py-26 lg:mt-20">
          <div className="container mx-auto px-4">
            <div className="lg:grid lg:grid-flow-row-dense lg:grid-cols-1 lg:gap-8 lg:items-center">
              <div className="ml-16 lg:col-start-2 lg:max-w-2xl lg:ml-36">
                <p className="text-base font-semibold leading-6 text-violet-600 uppercase">
                  {config.application}
                </p>
                <h4 className="mt-2 text-2xl font-extrabold leading-8 text-gray-900 dark:text-white sm:text-3xl sm:leading-9">
                  Interactivity and Efficiency for Your Enterprise
                </h4>
                <p className="mt-4 text-lg leading-6 text-gray-500 dark:text-gray-300">
                  Create a seamless and powerful collaborative space for your
                  enterprise. Track, share, and measure efficiently with full
                  control. It’s never been simpler and more effective.
                </p>
                <ul className="gap-6 mt-8 md:grid md:grid-cols-2">
                  <li className="mt-6 md:mt-0">
                    <div className="flex">
                      <span className="flex items-center justify-center flex-shrink-0 w-6 h-6 text-violet-800 bg-violet-100 rounded-full dark:text-violet-500 dark:bg-transparent">
                        <span className="icon-[charm--tick] w-4 h-4"></span>
                      </span>
                      <span className="ml-4 text-base font-medium leading-6 text-gray-500 dark:text-gray-200">
                        Real-time Updates
                      </span>
                    </div>
                  </li>
                  <li className="mt-6 md:mt-0">
                    <div className="flex">
                      <span className="flex items-center justify-center flex-shrink-0 w-6 h-6 text-violet-800 bg-violet-100 rounded-full dark:text-violet-500 dark:bg-transparent">
                        <span className="icon-[charm--tick] w-4 h-4"></span>
                      </span>
                      <span className="ml-4 text-base font-medium leading-6 text-gray-500 dark:text-gray-200">
                        Comprehensive Data Tracking
                      </span>
                    </div>
                  </li>
                  <li className="mt-6 md:mt-0">
                    <div className="flex">
                      <span className="flex items-center justify-center flex-shrink-0 w-6 h-6 text-violet-800 bg-violet-100 rounded-full dark:text-violet-500 dark:bg-transparent">
                        <span className="icon-[charm--tick] w-4 h-4"></span>
                      </span>
                      <span className="ml-4 text-base font-medium leading-6 text-gray-500 dark:text-gray-200">
                        24/7 Support
                      </span>
                    </div>
                  </li>
                  <li className="mt-6 md:mt-0">
                    <div className="flex">
                      <span className="flex items-center justify-center flex-shrink-0 w-6 h-6 text-violet-800 bg-violet-100 rounded-full dark:text-violet-500 dark:bg-transparent">
                        <span className="icon-[charm--tick] w-4 h-4"></span>
                      </span>
                      <span className="ml-4 text-base font-medium leading-6 text-gray-500 dark:text-gray-200">
                        Tips to Optimize Work Time
                      </span>
                    </div>
                  </li>
                </ul>
              </div>
              <div className="relative mt-12 lg:mt-0 space-y-4">
                <div className="flex items-end justify-center space-x-4 sm:justify-start">
                  <img
                    className="w-28 sm:w-32 md:w-56 lg:w-72 rounded-lg shadow-lg"
                    src={image3}
                    alt="1"
                  />
                  <img
                    className="w-36 sm:w-40 md:w-64 lg:w-80 rounded-lg shadow-lg"
                    src={image2}
                    alt="2"
                  />
                </div>
                <div className="flex items-start justify-center space-x-4 sm:justify-start">
                  <img
                    className="w-36 sm:w-40 md:w-64 lg:w-80 rounded-lg shadow-lg"
                    src={image4}
                    alt="4"
                  />
                  <img
                    className="w-28 sm:w-32 md:w-56 lg:w-72 rounded-lg shadow-lg"
                    src={image1}
                    alt="3"
                  />
                </div>
              </div>
            </div>
          </div>
        </div>
      </section>
    </>
  );
};

export default WhatWeProvide;
