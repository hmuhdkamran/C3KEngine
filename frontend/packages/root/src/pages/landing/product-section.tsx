import { Card } from "c3k-utilities";
import { FC } from "react";

const ProductSection: FC = () => {
  return (
    <>
      <section>
        <div className="mt-4 bg-gray-50 radius-for-skewed">
          <div className="container mx-auto px-4">
            <div className="flex flex-wrap items-center">
              <div className="w-full lg:w-1/2 mb-12 lg:mb-0">
                <div className="max-w-md lg:mx-auto">
                  <span className="text-violet-600 font-bold">
                    ERP Solutions
                  </span>
                  <h2 className="my-2 text-4xl lg:text-5xl font-bold font-heading">
                    Streamline Your Business Operations
                  </h2>
                  <p className="mb-6 text-violet-500 leading-loose">
                    Our ERP solution simplifies complex business processes,
                    enhances operational efficiency, and provides comprehensive
                    data management across all departments.
                  </p>
                  <ul className="text-gray-500 font-bold">
                    <li className="flex mb-4">
                      <span className="icon-[ep--success-filled] mr-2 w-6 h-6 text-violet-600"></span>
                      <span>Unified Business Management</span>
                    </li>
                    <li className="flex mb-4">
                      <span className="icon-[ep--success-filled] mr-2 w-6 h-6 text-violet-600"></span>
                      <span>Automated Financial Processes</span>
                    </li>
                    <li className="flex mb-4">
                      <span className="icon-[ep--success-filled] mr-2 w-6 h-6 text-violet-600"></span>
                      <span>Detailed Analytics & Reporting</span>
                    </li>
                  </ul>
                </div>
              </div>

              <div className="w-full lg:w-1/2 flex flex-wrap -mx-4">
                <div className="mb-8 lg:mb-0 w-full md:w-1/2 px-4">
                  <Card
                    showHeader={false}
                    showFooter={false}
                    borderColor="transparent"
                    containerClass={
                      "mb-8 py-6 pl-6 pr-4 shadow rounded bg-white"
                    }
                  >
                    <span className="mb-4 inline-block p-3 rounded-lg bg-violet-100">
                      <span className="icon-[tabler--photo] w-10 h-10 text-violet-600"></span>
                    </span>
                    <h4 className="mb-2 text-2xl font-bold font-heading">
                      Inventory Management
                    </h4>
                    <p className="text-gray-500 leading-loose">
                      Monitor stock levels, track orders, and manage your supply
                      chain efficiently with our advanced inventory tools.
                    </p>
                  </Card>

                  <Card
                    showHeader={false}
                    showFooter={false}
                    borderColor="transparent"
                    containerClass={"py-6 pl-6 pr-4 shadow rounded bg-white"}
                  >
                    <span className="mb-4 inline-block p-3 rounded-lg bg-violet-100">
                      <span className="icon-[fluent--toolbox-32-filled] w-10 h-10 text-violet-600"></span>
                    </span>
                    <h4 className="mb-2 text-2xl font-bold font-heading">
                      Financial Management
                    </h4>
                    <p className="text-gray-500 leading-loose">
                      Streamline accounting, billing, and financial reporting
                      with automated tools to improve accuracy and compliance.
                    </p>
                  </Card>
                </div>

                <div className="w-full md:w-1/2 lg:mt-20 px-4">
                  <Card
                    showHeader={false}
                    showFooter={false}
                    borderColor="transparent"
                    containerClass={
                      "mb-8 py-6 pl-6 pr-4 shadow rounded-lg bg-white"
                    }
                  >
                    <span className="mb-4 inline-block p-3 rounded bg-violet-100">
                      <span className="icon-[streamline--star-badge-solid] w-10 h-10 text-violet-600"></span>
                    </span>
                    <h4 className="mb-2 text-2xl font-bold font-heading">
                      CRM Integration
                    </h4>
                    <p className="text-gray-500 leading-loose">
                      Manage customer relationships with built-in CRM tools,
                      providing a holistic view of customer data and
                      interactions.
                    </p>
                  </Card>

                  <Card
                    showHeader={false}
                    showFooter={false}
                    borderColor="transparent"
                    containerClass={"py-6 pl-6 pr-4 shadow rounded-lg bg-white"}
                  >
                    <span className="mb-4 inline-block p-3 rounded bg-violet-100">
                      <span className="icon-[mdi--tick-decagram] w-10 h-10 text-violet-600"></span>
                    </span>
                    <h4 className="mb-2 text-2xl font-bold font-heading">
                      Compliance and Risk Management
                    </h4>
                    <p className="text-gray-500 leading-loose">
                      Ensure your business meets industry standards and
                      regulations with automated compliance monitoring and risk
                      assessments.
                    </p>
                  </Card>
                </div>
              </div>
            </div>
          </div>
        </div>
      </section>
    </>
  );
};

export default ProductSection;
