import { FC, useEffect } from "react";
import HeaderArea from "@/components/page/header-area";
import { usePageContext } from "@/plugins/store";
import DataTable from "@/components/data/data-table";

const UserModule: FC = () => {
  const { pageTitle, updatePageState } = usePageContext();

  useEffect(() => {
    const newState = {
      pageTitle: "User Module",
      breadcrumbItems: [{ title: "Authentication", route: "/", icon: "home" }],
    };
    updatePageState(newState);
  }, [updatePageState]);

  const goToMain = () => {
    console.log("Going to main page");
  };

  const hrmsCards = [
    {
      title: 'Employee Management',
      description: 'Manage employee records and workflows efficiently.',
      status: 'Installed',
      buttonText: 'LEARN MORE',
      iconClass: 'icon-[fluent-mdl2--people]',
    },
    {
      title: 'Payroll',
      description: 'Automate payroll processing and tax management.',
      status: 'Activate',
      buttonText: 'LEARN MORE',
      iconClass: 'icon-[fluent-mdl2--money]',
    },
    {
      title: 'Attendance Tracking',
      description: 'Track employee attendance with real-time reporting.',
      status: 'Installed',
      buttonText: 'LEARN MORE',
      iconClass: 'icon-[fluent-mdl2--clock]',
    },
    {
      title: 'Recruitment',
      description: 'Streamline the recruitment process with automation.',
      status: 'Installed',
      buttonText: 'LEARN MORE',
      iconClass: 'icon-[fluent-mdl2--recruitment-management]',
    },
    {
      title: 'Benefits Management',
      description: 'Easily manage employee benefits and compliance.',
      status: 'Activate',
      buttonText: 'LEARN MORE',
      iconClass: 'icon-[fluent-mdl2--heart]',
    },
    {
      title: 'Training',
      description: 'Facilitate employee training and development programs.',
      status: 'Installed',
      buttonText: 'LEARN MORE',
      iconClass: 'icon-[fluent-mdl2--education]',
    },
    {
      title: 'Performance Management',
      description: 'Evaluate and improve employee performance.',
      status: 'Installed',
      buttonText: 'LEARN MORE',
      iconClass: 'icon-[fluent-mdl2--task-manager]',
    },
    {
      title: 'Employee Self-Service',
      description: 'Allow employees to manage their own information.',
      status: 'Installed',
      buttonText: 'LEARN MORE',
      iconClass: 'icon-[flowbite--user-settings-outline]',
    },
    {
      title: 'Time-Off Management',
      description: 'Manage employee leave requests and approvals.',
      status: 'Activate',
      buttonText: 'LEARN MORE',
      iconClass: 'icon-[fluent-mdl2--calendar-settings]',
    },
    {
      title: 'Expense Management',
      description: 'Track and manage employee expenses.',
      status: 'Activate',
      buttonText: 'LEARN MORE',
      iconClass: 'icon-[healthicons--money-bag]',
    },
    {
      title: 'Onboarding',
      description: 'Simplify the onboarding process for new hires.',
      status: 'Installed',
      buttonText: 'LEARN MORE',
      iconClass: 'icon-[healthicons--default]',
    },
    {
      title: 'Offboarding',
      description: 'Manage exit processes and final paperwork.',
      status: 'Installed',
      buttonText: 'LEARN MORE',
      iconClass: 'icon-[healthicons--default]',
    },
    {
      title: 'Employee Surveys',
      description: 'Conduct surveys to gather employee feedback.',
      status: 'Activate',
      buttonText: 'LEARN MORE',
      iconClass: 'icon-[healthicons--default]',
    },
    {
      title: 'Health & Safety',
      description: 'Ensure compliance with health and safety regulations.',
      status: 'Activate',
      buttonText: 'LEARN MORE',
      iconClass: 'icon-[healthicons--default]',
    },
    {
      title: 'Compliance Management',
      description: 'Maintain regulatory compliance and reporting.',
      status: 'Activate',
      buttonText: 'LEARN MORE',
      iconClass: 'icon-[healthicons--default]',
    },
    {
      title: 'Project Management',
      description: 'Plan, execute, and monitor projects effectively.',
      status: 'Installed',
      buttonText: 'LEARN MORE',
      iconClass: 'icon-[healthicons--default]',
    },
    {
      title: 'Document Management',
      description: 'Organize and manage employee documents securely.',
      status: 'Activate',
      buttonText: 'LEARN MORE',
      iconClass: 'icon-[healthicons--default]',
    },
    {
      title: 'Succession Planning',
      description: 'Prepare for future leadership needs and talent gaps.',
      status: 'Installed',
      buttonText: 'LEARN MORE',
      iconClass: 'icon-[healthicons--default]',
    },
    {
      title: 'HR Analytics',
      description: 'Analyze HR metrics for data-driven decisions.',
      status: 'Activate',
      buttonText: 'LEARN MORE',
      iconClass: 'icon-[healthicons--default]',
    },
    {
      title: 'Employee Engagement',
      description: 'Enhance employee satisfaction and retention strategies.',
      status: 'Activate',
      buttonText: 'LEARN MORE',
      iconClass: 'icon-[healthicons--default]',
    },
  ];
  
  const columns = [
    { key: 'check', label: 'check', sort: false, check: true },
    { key: 'title', label: 'Title', sort: true },
    { key: 'description', label: 'Description', sort: true },
    { key: 'status', label: 'Status', sort: false, width: '100px', class: 'text-center' },
    { key: 'action', label: 'Action', sort: false, width: '100px', class: 'text-center' }
  ];

  return (
    <>
      <div className="bg-white mt-12 flex flex-col">
        <HeaderArea pageHeading={pageTitle} goToMain={goToMain}>
        <button
              className="flex items-center bg-violet-600 text-white hover:bg-violet-700 transition px-2 py-1 rounded-sm shadow-md"
            >
              <i className="icon-[fluent--filter-16-filled] mr-1"></i> Add
            </button>
            <button
              className="flex items-center bg-red-500 text-white hover:bg-red-600 transition px-2 py-1 rounded-sm shadow-md"
            >
              <i className="icon-[fluent--group-24-filled] mr-1"></i> Remove
            </button>

        </HeaderArea>
        <div className="bg-gray-50 flex-1 mx-auto mt-4 py-6 px-6 w-full h-full">
          <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6">
            <DataTable data={hrmsCards} columns={columns} />
          </div>
        </div>
      </div>
    </>
  );
};

export default UserModule;
