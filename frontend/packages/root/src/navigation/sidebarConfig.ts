const sidebarMenu = [
  {
    title: "Dashboard",
    icon: "icon-[tabler--category-filled]",
    items: [
      { name: "HRMS", icon: "icon-[fluent-mdl2--recruitment-management]", link: "/hrms" },
      { name: "Retail", icon: "icon-[vaadin--shop]", link: "/retail" },
      { name: "Production", icon: "icon-[mdi--office-building-settings-outline]", link: "/production" },
      { name: "Finance", icon: "icon-[mdi--finance]", link: "/finance" },
      { name: "Marketing", icon: "icon-[nimbus--marketing]", link: "/marketing" },
    ]
  },
  {
    title: "User Management",
    icon: "icon-[ic--baseline-supervisor-account]",
    items: [
      { name: "User Creation", icon: "icon-[mdi--account-plus-outline]", link: "/role/user" },
      { name: "User List", icon: "icon-[mdi--account-group-outline]", link: "/role/userlist" },
      { name: "Roles & Permissions", icon: "icon-[ic--baseline-security]", link: "/role/permissions" },
      { name: "Teams", icon: "icon-[fluent-mdl2--people]", link: "/role/teams" },
    ]
  },
  {
    title: "Reports",
    icon: "icon-[ic--baseline-assessment]",
    items: [
      { name: "Sales Report", icon: "icon-[ic--baseline-show-chart]", link: "/reports/sales" },
      { name: "Employee Performance", icon: "icon-[grommet-icons--document-performance]", link: "/reports/performance" },
      { name: "Expense Report", icon: "icon-[mdi--finance]", link: "/reports/expenses" },
    ]
  },
  {
    title: "Settings",
    icon: "icon-[ic--baseline-settings]",
    items: [
      { name: "Profile", icon: "icon-[ic--baseline-person]", link: "/profile" },
      { name: "App Settings", icon: "icon-[ic--baseline-settings-applications]", link: "/settings" },
      { name: "Notifications", icon: "icon-[ic--baseline-notifications]", link: "/settings/notifications" },
    ]
  },
  {
    title: "Actions",
    icon: "icon-[ic--baseline-lightbulb]",
    items: [
      { name: "Logout", icon: "icon-[ic--baseline-logout]", link: "/logout" },
      { name: "Help", icon: "icon-[ic--baseline-help-outline]", link: "/help" },
    ]
  },
  {
    title: "Support",
    icon: "icon-[streamline--customer-support-1-solid]",
    items: [
      { name: "FAQ", icon: "icon-[ic--baseline-question-answer]", link: "/support/faq" },
      { name: "Contact Us", icon: "icon-[ic--baseline-contact-phone]", link: "/support/contact" },
      { name: "Live Chat", icon: "icon-[ic--baseline-chat]", link: "/support/chat" },
    ]
  }
];

export default sidebarMenu;
