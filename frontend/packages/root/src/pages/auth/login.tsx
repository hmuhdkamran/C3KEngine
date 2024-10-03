import { FC, useState } from "react";
import { Link, useLocation, useNavigate } from "react-router-dom";
import { AuthenticationService } from "@/services/auth/authentication";
import { config } from "@/plugins/config";
import VNodeRenderer from "@/components/extra/node-renderer";
import { ICredential } from "@/types/axios";
import { useSystemContext } from "@/plugins/store";
import { IService } from "@/components/extra/service-dropdown";
import { MenuItem } from "@/components/extra/menu-items";

const Login: FC = () => {
  const services: IService[] = [
    {
      name: "Authentication Service",
      icon: "icon-[mdi--account-check]",
      description: "Manage user authentication",
      route: "/roles/user-module",
    },
    {
      name: "Business Setup Service",
      icon: "icon-[mdi--office-building]",
      description: "Setup business essentials",
      route: "/business-setup",
    },
    {
      name: "HRMS Service",
      icon: "icon-[clarity--employee-group-solid]",
      description: "Human resource management",
      route: "/hrms",
    },
    {
      name: "Retail Service",
      icon: "icon-[mdi--store]",
      description: "Manage retail operations",
      route: "/retail",
    },
    {
      name: "Point of Sale Service",
      icon: "icon-[mdi--cash-register]",
      description: "Point of Sale functionality",
      route: "/pos",
    },
    {
      name: "Supply Chain Service",
      icon: "icon-[mdi--truck-delivery]",
      description: "Manage supply chain",
      route: "/supply-chain",
    },
    {
      name: "Finance Service",
      icon: "icon-[mdi--currency-usd]",
      description: "Financial services",
      route: "/finance",
    },
  ];

  const menuItems: MenuItem[] = [
    {
      name: "CRM Dashboard",
      children: [
        { name: "Overview", path: "/crm/dashboard/overview" },
        { name: "Reports", path: "/crm/dashboard/reports" },
      ],
    },
    {
      name: "Sales Overview",
      path: "/sales-overview",
    },
    {
      name: "Customer Engagement",
      children: [
        { name: "Feedback", path: "/customer/engagement/feedback" },
        { name: "Follow-ups", path: "/customer/engagement/follow-ups" },
      ],
    },
    {
      name: "Pipeline Management",
      path: "/pipeline-management",
    },
  ];

  const sidebarMenu: MenuItem[] = [
    {
      name: "Dashboard",
      icon: "icon-[tabler--category-filled]",
      children: [
        { name: "HRMS", icon: "icon-[fluent-mdl2--recruitment-management]", path: "/hrms" },
        { name: "Retail", icon: "icon-[vaadin--shop]", path: "/retail" },
        { name: "Production", icon: "icon-[mdi--office-building-settings-outline]", path: "/production" },
        { name: "Finance", icon: "icon-[mdi--finance]", path: "/finance" },
        { name: "Marketing", icon: "icon-[nimbus--marketing]", path: "/marketing" },
      ]
    },
    {
      name: "User Management",
      icon: "icon-[ic--baseline-supervisor-account]",
      children: [
        { name: "User Creation", icon: "icon-[mdi--account-plus-outline]", path: "/role/user" },
        { name: "User List", icon: "icon-[mdi--account-group-outline]", path: "/role/userlist" },
        { name: "Roles & Permissions", icon: "icon-[ic--baseline-security]", path: "/role/permissions" },
        { name: "Teams", icon: "icon-[fluent-mdl2--people]", path: "/role/teams" },
      ]
    },
    {
      name: "Reports",
      icon: "icon-[ic--baseline-assessment]",
      children: [
        { name: "Sales Report", icon: "icon-[ic--baseline-show-chart]", path: "/reports/sales" },
        { name: "Employee Performance", icon: "icon-[grommet-icons--document-performance]", path: "/reports/performance" },
        { name: "Expense Report", icon: "icon-[mdi--finance]", path: "/reports/expenses" },
      ]
    },
    {
      name: "Settings",
      icon: "icon-[ic--baseline-settings]",
      children: [
        { name: "Profile", icon: "icon-[ic--baseline-person]", path: "/profile" },
        { name: "App Settings", icon: "icon-[ic--baseline-settings-applications]", path: "/settings" },
        { name: "Notifications", icon: "icon-[ic--baseline-notifications]", path: "/settings/notifications" },
      ]
    },
    {
      name: "Actions",
      icon: "icon-[ic--baseline-lightbulb]",
      children: [
        { name: "Logout", icon: "icon-[ic--baseline-logout]", path: "/logout" },
        { name: "Help", icon: "icon-[ic--baseline-help-outline]", path: "/help" },
      ]
    },
    {
      name: "Support",
      icon: "icon-[streamline--customer-support-1-solid]",
      children: [
        { name: "FAQ", icon: "icon-[ic--baseline-question-answer]", path: "/support/faq" },
        { name: "Contact Us", icon: "icon-[ic--baseline-contact-phone]", path: "/support/contact" },
        { name: "Live Chat", icon: "icon-[ic--baseline-chat]", path: "/support/chat" },
      ]
    }
  ];  

  const service = new AuthenticationService();
  const location = useLocation();
  const navigate = useNavigate();
  const { updateUser, updateServices, updateMenuItems, updateSidebarMenu } = useSystemContext();

  const [email, setEmail] = useState("admin@sefam.com");
  const [password, setPassword] = useState("P@ssw0rd");

  const login = () => {
    const credentials: ICredential = {
      username: email.toLowerCase(),
      password: password,
    };

    service.login(credentials).then((response) => {
      if (response?.authenticated) {
        
        updateUser(response);
        updateServices(services);
        updateMenuItems(menuItems);
        updateSidebarMenu(sidebarMenu);

        const redirectTo =
          new URLSearchParams(location.search).get("to") || "/dashboard";
        navigate(redirectTo);
      } else {
        console.error("Login failed, response is undefined.");
      }
    });
  };

  return (
    <div className="min-h-screen py-6 flex flex-col justify-center sm:py-12 relative h-full md:flex items-center p-10 overflow-hidden bg-violet-900 text-white bg-no-repeat bg-cover">
      <div className="absolute bg-gradient-to-b from-violet-500 to-purple-300 opacity-75 inset-0 z-0"></div>
      <ul className="circles">
        <li></li>
        <li></li>
        <li></li>
        <li></li>
        <li></li>
        <li></li>
        <li></li>
        <li></li>
        <li></li>
        <li></li>
      </ul>
      <div className="relative py-3 sm:max-w-xl sm:mx-auto w-full">
        <div className="absolute inset-0 bg-gradient-to-r from-violet-900 to-purple-900 shadow-lg -skew-y-6 sm:skew-y-0 sm:-rotate-6 sm:rounded-3xl transition transform hover:scale-105 duration-500"></div>
        <div className="relative bg-white shadow-lg sm:rounded-3xl sm:p-5 transition transform hover:scale-105 duration-500">
          <div className="flex flex-col bg-white">
            <div className="flex justify-center md:justify-start md:pl-6 md:-mb-12">
              <a href="/" className="text-white font-bold text-xl p-2">
                <VNodeRenderer nodes={config.logo} />
              </a>
            </div>
            <div className="flex flex-col justify-center md:justify-start px-2 my-auto md:pt-0 md:px-12">
              <p className="text-center text-2xl text-gray-800">Welcome.</p>
              <div className="flex flex-col pt-3 md:pt-8">
                <div className="flex flex-col pt-4">
                  <label htmlFor="email" className="text-lg text-gray-800">
                    Email
                  </label>
                  <input
                    type="email"
                    id="email"
                    placeholder="your@email.com"
                    value={email}
                    onChange={(e) => setEmail(e.target.value)}
                    className="input-bottom"
                  />
                </div>
                <div className="flex flex-col pt-4">
                  <label htmlFor="password" className="text-lg text-gray-800">
                    Password
                  </label>
                  <input
                    type="password"
                    id="password"
                    placeholder="Password"
                    value={password}
                    onChange={(e) => setPassword(e.target.value)}
                    className="input-bottom"
                  />
                </div>
                <button onClick={login} className="btn-gradient mt-6">
                  Login
                </button>
              </div>
              <div className="text-center pt-12 pb-12">
                <p className="text-gray-800">
                  Don't have an account?{" "}
                  <Link
                    to="/auth/register"
                    className="underline font-semibold text-violet-700"
                  >
                    Register here.
                  </Link>
                </p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Login;
