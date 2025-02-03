import type { User } from '@/models/user';
import { GlobalConfig, StoreService, TokenHelper, useSystemStore } from 'c3-library';
import Axios, { type AxiosResponse } from 'axios';

export class UserService extends StoreService {
  constructor() {
    super();
  }

  async getAll(): Promise<User[]> {
    const token = TokenHelper.getAccessToken();
    if (!token) {
      throw new Error('Authentication token not found');
    }

    try {
      const response: AxiosResponse = await Axios.get(`${GlobalConfig.uri.auth}/role/users`, {
        headers: {
          Authorization: `Bearer ${token}`,
        },
      });

      if (response.data.result === 'Success') {
        return response.data.data;
      } else {
        throw new Error(response.data.description || 'Failed to fetch users');
      }
    } catch (error: any) {
      if (error.response && error.response.status === 401) {
        TokenHelper.removeAccessToken();
        throw new Error('Invalid token, please login again');
      }
      console.error('Error fetching users:', error);
      throw error;
    }
  }

  async add(user: User): Promise<boolean> {
    const token = TokenHelper.getAccessToken();
    if (!token) {
      throw new Error('Authentication token not found');
    }

    try {
      const response: AxiosResponse = await Axios.post(`${GlobalConfig.uri.auth}/role/users`, user, {
        headers: {
          Authorization: `Bearer ${token}`,
        },
      });

      if (response.data.result === 'Success') {
        return true;
      } else {
        throw new Error(response.data.description || 'Failed to add user');
      }
    } catch (error: any) {
      console.error('Error adding user:', error);
      throw error;
    }
  }

  async update(user: User): Promise<boolean> {
    const token = TokenHelper.getAccessToken();
    if (!token) {
      throw new Error('Authentication token not found');
    }

    try {
      const response: AxiosResponse = await Axios.put(`${GlobalConfig.uri.auth}/role/users`, user, {
        headers: {
          Authorization: `Bearer ${token}`,
        },
      });

      if (response.data.result === 'Success') {
        return true;
      } else {
        throw new Error(response.data.description || 'Failed to update user');
      }
    } catch (error: any) {
      console.error('Error updating user:', error);
      throw error;
    }
  }

  async delete(userId: string): Promise<boolean> {
    const token = TokenHelper.getAccessToken();
    if (!token) {
      throw new Error('Authentication token not found');
    }

    try {
      const response: AxiosResponse = await Axios.delete(`${GlobalConfig.uri.auth}/role/users/${userId}`, {
        headers: {
          Authorization: `Bearer ${token}`,
        },
      });

      if (response.data.result === 'Success') {
        return true;
      } else {
        throw new Error(response.data.description || 'Failed to delete user');
      }
    } catch (error: any) {
      console.error('Error deleting user:', error);
      throw error;
    }
  }
}