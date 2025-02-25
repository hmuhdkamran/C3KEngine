export const menuItems = [
  {
    name: 'Home',
    icon: 'fa-solid fa-house',
    link: '/dashboard',
    children: [{
      name: 'Dashboard',
      link: '/dashboard',
      icon: 'fa-solid fa-grid-2'
    }],
  },
  {
    name: 'HRMs',
    icon: 'fa-solid fa-briefcase',
    link: '/hrms',
    children: [
      {
        name: 'Employee Management',
        link: '/hrms/employees',
        icon: 'fa-solid fa-user-gear'
      },
      {
        name: 'Payroll',
        link: '/hrms/payroll',
        icon: 'fa-solid fa-money-check-dollar'
      },
      {
        name: 'Attendance',
        link: '/hrms/attendance',
        icon: 'fa-solid fa-calendar-check'
      },
    ],
  },
  {
    name: 'User Management',
    icon: 'fa-solid fa-user-shield',
    link: '/users',
    children: [
      {
        name: 'Users',
        link: '/users',
        icon: 'fa-solid fa-user'
      },
      {
        name: 'User Roles',
        link: '/users/roles',
        icon: 'fa-solid fa-user-tag'
      },
      {
        name: 'Routes',
        link: '/users/routes',
        icon: 'fa-solid fa-route'
      },
      {
        name: 'Roles Routes Map',
        link: '/users/roles-routes-map',
        icon: 'fa-solid fa-diagram-project'
      },
      {
        name: 'Users Roles Map',
        link: '/users/users-roles-map',
        icon: 'fa-solid fa-users-between-lines'
      },
      {
        name: 'Queries',
        link: '/users/queries',
        icon: 'fa-solid fa-comment-dots'
      },
    ],
  },
  {
    name: 'Production',
    icon: 'fa-solid fa-conveyor-belt-arm',
    link: '/production',
    children: [
      {
        name: 'Inventory',
        link: '/production/inventory',
        icon: 'fa-solid fa-boxes'
      },
      {
        name: 'Manufacturing',
        link: '/production/manufacturing',
        icon: 'fa-solid fa-industry'
      },
      {
        name: 'Quality Control',
        link: '/production/quality-control',
        icon: 'fa-solid fa-clipboard-check'
      },
    ],
  },
  {
    name: 'Retail',
    icon: 'fa-solid fa-shopping-cart',
    link: '/retail',
    children: [
      {
        name: 'Point of Sale',
        link: '/retail/pos',
        icon: 'fa-solid fa-cash-register'
      },
      {
        name: 'Sales Reports',
        link: '/retail/sales-reports',
        icon: 'fa-solid fa-chart-bar'
      },
      {
        name: 'Customer Feedback',
        link: '/retail/customer-feedback',
        icon: 'fa-solid fa-comments'
      },
    ],
  },
];