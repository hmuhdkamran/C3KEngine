interface MenuItem {
  name: string;
  title: string;
  icon: string;
  route: string;
  children?: MenuItem[];
}

const sidebarMenu: MenuItem[] = [
  {
    name: "Dashboard",
    title: "Dashboard",
    icon: "icon-[tabler--category-filled]",
    route: "/dashboard",
    children: [
      { name: "HRMS", title: "HRMS", icon: "icon-[fluent-mdl2--recruitment-management]", route: "/hrms" },
      { name: "Retail", title: "Retail", icon: "icon-[vaadin--shop]", route: "/retail" },
      { name: "Production", title: "Production", icon: "icon-[mdi--office-building-settings-outline]", route: "/production" },
      { name: "Finance", title: "Finance", icon: "icon-[mdi--finance]", route: "/finance" },
      { name: "Marketing", title: "Marketing", icon: "icon-[nimbus--marketing]", route: "/marketing" },
    ]
  },
  {
    name: "User Management",
    title: "User Management",
    icon: "icon-[ic--baseline-supervisor-account]",
    route: "/user-management",
    children: [
      { name: "User Creation", title: "User Creation", icon: "icon-[mdi--account-plus-outline]", route: "/role/user" },
      { name: "User List", title: "User List", icon: "icon-[mdi--account-group-outline]", route: "/role/userlist" },
      { name: "Roles & Permissions", title: "Roles & Permissions", icon: "icon-[ic--baseline-security]", route: "/role/permissions" },
      { name: "Teams", title: "Teams", icon: "icon-[fluent-mdl2--people]", route: "/role/teams" },
    ]
  },
  {
    name: "Reports",
    title: "Reports",
    icon: "icon-[ic--baseline-assessment]",
    route: "/reports",
    children: [
      { name: "Sales Report", title: "Sales Report", icon: "icon-[ic--baseline-show-chart]", route: "/reports/sales" },
      { name: "Employee Performance", title: "Employee Performance", icon: "icon-[grommet-icons--document-performance]", route: "/reports/performance" },
      { name: "Expense Report", title: "Expense Report", icon: "icon-[mdi--finance]", route: "/reports/expenses" },
    ]
  },
  {
    name: "Settings",
    title: "Settings",
    icon: "icon-[ic--baseline-settings]",
    route: "/settings",
    children: [
      { name: "Profile", title: "Profile", icon: "icon-[ic--baseline-person]", route: "/profile" },
      { name: "App Settings", title: "App Settings", icon: "icon-[ic--baseline-settings-applications]", route: "/settings" },
      { name: "Notifications", title: "Notifications", icon: "icon-[ic--baseline-notifications]", route: "/settings/notifications" },
    ]
  },
  {
    name: "Actions",
    title: "Actions",
    icon: "icon-[ic--baseline-lightbulb]",
    route: "/actions",
    children: [
      { name: "Logout", title: "Logout", icon: "icon-[ic--baseline-logout]", route: "/logout" },
      { name: "Help", title: "Help", icon: "icon-[ic--baseline-help-outline]", route: "/help" },
    ]
  },
  {
    name: "Support",
    title: "Support",
    icon: "icon-[streamline--customer-support-1-solid]",
    route: "/support",
    children: [
      { name: "FAQ", title: "FAQ", icon: "icon-[ic--baseline-question-answer]", route: "/support/faq" },
      { name: "Contact Us", title: "Contact Us", icon: "icon-[ic--baseline-contact-phone]", route: "/support/contact" },
      { name: "Live Chat", title: "Live Chat", icon: "icon-[ic--baseline-chat]", route: "/support/chat" },
    ]
  }
];

export default sidebarMenu;
