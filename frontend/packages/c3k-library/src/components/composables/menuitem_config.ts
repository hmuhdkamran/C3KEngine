import type { IMenuItem } from '@/index';

export const menuItems: IMenuItem[] = [
  {
    name: 'Home',
    title: 'Home Dashboard',
    icon: 'mdi:home',
    route: '#',
    open: false,
    children: [
      { name: 'Home', title: 'Dashboard', icon: '', route: '/dashboard', open: false },
    ],
  },
  {
    name: 'Role',
    title: 'User Roles',
    icon: 'mdi:users',
    route: '#',
    open: false,
    children: [
      { name: 'User', title: 'User', icon: '', route: '/user', open: false },
      { name: 'Role', title: 'Role', icon: '', route: '/user-role', open: false },
      { name: 'Routes', title: 'Routes', icon: '', route: '/user-routes', open: false },
    ],
  },
];