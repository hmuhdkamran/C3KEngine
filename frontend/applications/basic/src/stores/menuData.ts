import type { IMenuItem } from "c3-library";

export const menuItems: IMenuItem[] = [
  {
    name: 'dashboard',
    title: 'Dashboard',
    icon: 'fa-solid fa-grid-2',
    route: '/dashboard',
    open: false,
  },
  {
    name: 'User Management',
    title: 'User Management',
    icon: 'fa-solid fa-user-shield',
    route: '/users',
    open: false,
    children: [
      {
        name: 'users',
        title: 'Users',
        route: '/users',
        icon: 'fa-solid fa-user',
        open: false,
      },
      {
        name: 'user-roles',
        title: 'User Roles',
        route: '/users/roles',
        icon: 'fa-solid fa-user-tag',
        open: false,
      },
      {
        name: 'routes',
        title: 'Routes',
        route: '/users/routes',
        icon: 'fa-solid fa-route',
        open: false,
      },
      {
        name: 'roles-routes-map',
        title: 'Roles Routes Map',
        route: '/users/roles-routes-map',
        icon: 'fa-solid fa-diagram-project',
        open: false,
      },
      {
        name: 'users-roles-map',
        title: 'Users Roles Map',
        route: '/users/users-roles-map',
        icon: 'fa-solid fa-users-between-lines',
        open: false,
      },
      {
        name: 'queries',
        title: 'Queries',
        route: '/users/queries',
        icon: 'fa-solid fa-comment-dots',
        open: false,
      },
    ],
  },
]
