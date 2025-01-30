export const menuItems = [
    {
      name: 'Home',
      icon: 'fa-solid fa-house',
      link: '/dashboard',
      children: [{ name: 'Dashboard', link: '/dashboard' }],
    },
    {
      name: 'HRMs',
      icon: 'fa-solid fa-briefcase',
      link: '/hrms',
      children: [
        { name: 'Employee Management', link: '/hrms/employees' },
        { name: 'Payroll', link: '/hrms/payroll' },
        { name: 'Attendance', link: '/hrms/attendance' },
      ],
    },
    {
      name: 'User Management',
      icon: 'fa-solid fa-user-shield',
      link: '/users',
      children: [
        { name: 'Users', link: '/users' },
        { name: 'User Roles', link: '/users/roles' },
        { name: 'Routes', link: '/users/routes' },
        { name: 'Roles Routes Map', link: '/users/roles-routes-map' },
        { name: 'Users Roles Map', link: '/users/users-roles-map' },
        { name: 'Queries', link: '/users/queries' },
      ],
    },
    {
      name: 'Production',
      icon: 'fa-solid fa-conveyor-belt-arm',
      link: '/production',
      children: [
        { name: 'Inventory', link: '/production/inventory' },
        { name: 'Manufacturing', link: '/production/manufacturing' },
        { name: 'Quality Control', link: '/production/quality-control' },
      ],
    },
    {
      name: 'Retail',
      icon: 'fa-solid fa-shopping-cart',
      link: '/retail',
      children: [
        { name: 'Point of Sale', link: '/retail/pos' },
        { name: 'Sales Reports', link: '/retail/sales-reports' },
        { name: 'Customer Feedback', link: '/retail/customer-feedback' },
      ],
    },
  ];
  