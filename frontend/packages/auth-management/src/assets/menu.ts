import type { IMenuItem } from "c3k-library";
import { ref, type Ref } from "vue";

export const sidebarMenu: Ref<IMenuItem[]> = ref([
  {
    name: "Dashboard",
    title: "Dashboard",
    icon: "icon-[tabler--category-filled]",
    route: "/c3k-api-auth/dashboard",
    open: false,
    children: [
      { name: "HRMS", title: "HRMS", icon: "icon-[fluent-mdl2--recruitment-management]", open: false, route: "/hrms" },
      { name: "Retail", title: "Retail", icon: "icon-[vaadin--shop]", open: false, route: "/retail" },
      { name: "Production", title: "Production", icon: "icon-[mdi--office-building-settings-outline]", open: false, route: "/production" },
      { name: "Finance", title: "Finance", icon: "icon-[mdi--finance]", open: false, route: "/finance" },
      { name: "Marketing", title: "Marketing", icon: "icon-[nimbus--marketing]", open: false, route: "/marketing" },
    ]
  },
  {
    name: "User Management",
    title: "User Management",
    icon: "icon-[ic--baseline-supervisor-account]",
    route: "/user-management",
    open: false,
    children: [
      { name: "User Creation", title: "User Creation", icon: "icon-[mdi--account-plus-outline]", open: false, route: "/role/user" },
      { name: "User List", title: "User List", icon: "icon-[mdi--account-group-outline]", open: false, route: "/role/userlist" },
      { name: "Roles & Permissions", title: "Roles & Permissions", icon: "icon-[ic--baseline-security]", open: false, route: "/role/permissions" },
      { name: "Teams", title: "Teams", icon: "icon-[fluent-mdl2--people]", open: false, route: "/role/teams" },
    ]
  },
  {
    name: "Reports",
    title: "Reports",
    icon: "icon-[ic--baseline-assessment]",
    route: "/reports",
    open: false,
    children: [
      { name: "Sales Report", title: "Sales Report", icon: "icon-[ic--baseline-show-chart]", open: false, route: "/reports/sales" },
      { name: "Employee Performance", title: "Employee Performance", icon: "icon-[grommet-icons--document-performance]", open: false, route: "/reports/performance" },
      { name: "Expense Report", title: "Expense Report", icon: "icon-[mdi--finance]", open: false, route: "/reports/expenses" },
    ]
  },
  {
    name: "Settings",
    title: "Settings",
    icon: "icon-[ic--baseline-settings]",
    route: "/settings",
    open: false,
    children: [
      { name: "Profile", title: "Profile", icon: "icon-[ic--baseline-person]", open: false, route: "/profile" },
      { name: "App Settings", title: "App Settings", icon: "icon-[ic--baseline-settings-applications]", open: false, route: "/settings" },
      { name: "Notifications", title: "Notifications", icon: "icon-[ic--baseline-notifications]", open: false, route: "/settings/notifications" },
    ]
  },
  {
    name: "Actions",
    title: "Actions",
    icon: "icon-[ic--baseline-lightbulb]",
    route: "/actions",
    open: false,
    children: [
      { name: "Logout", title: "Logout", icon: "icon-[ic--baseline-logout]", open: false, route: "/logout" },
      { name: "Help", title: "Help", icon: "icon-[ic--baseline-help-outline]", open: false, route: "/help" },
    ]
  },
  {
    name: "Support",
    title: "Support",
    icon: "icon-[streamline--customer-support-1-solid]",
    route: "/support",
    open: false,
    children: [
      { name: "FAQ", title: "FAQ", icon: "icon-[ic--baseline-question-answer]", open: false, route: "/support/faq" },
      { name: "Contact Us", title: "Contact Us", icon: "icon-[ic--baseline-contact-phone]", open: false, route: "/support/contact" },
      { name: "Live Chat", title: "Live Chat", icon: "icon-[ic--baseline-chat]", open: false, route: "/support/chat" },
    ]
  }
]);
