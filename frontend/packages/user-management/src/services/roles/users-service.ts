/**
 * This service handles API calls related to Users. It extends the StoreService class.
 * It provides methods for fetching, adding, updating, and deleting Users data.
 * The methods utilize Axios for HTTP requests and Vuex for state management.
 *
 * Author: H.Muhammad Kamran
 * Email: hmuhdkamran@gmail.com
 * Contact: +92 (313 / 333) 9112 845
 */

// Importing necessary dependencies and modules.
import { IUser } from '@/models';
import Axios from 'axios';
import { GlobalConfig, StoreService } from 'c3k-library';

// Setting the base URL for User API calls.
const BASE_URL = `${GlobalConfig.uri.services}auth/user`;

export class UsersService extends StoreService {
  /**
   * Fetches all Users data.
   * @returns Promise with API response data.
   */
  public GetAll() {
    return this.exec(Axios.get(`${BASE_URL}`))
      .then((value: any) => this.processPayload(value))
      .catch((error: any) => console.error(error));
  }

  /**
   * Fetches Users data based on a filter.
   * @param filter Filter parameter.
   * @returns Promise with API response data.
   */
  public GetFindBy(filter: string) {
    return this.exec(Axios.get(`${BASE_URL}/${filter}`))
      .then((value: any) => this.processPayload(value))
      .catch((error: any) => console.error(error));
  }

  /**
   * Adds a new User entity.
   * @param entity User entity object.
   * @returns Promise with API response data.
   */
  public AddOne(entity: IUser) {
    return this.exec(Axios.post(`${BASE_URL}`, entity))
      .then((value: any) => this.processPayload(value))
      .catch((error: any) => console.error(error));
  }

  /**
   * Updates an existing User entity.
   * @param entity User entity object.
   * @returns Promise with API response data.
   */
  public Update(entity: IUser) {
    return this.exec(Axios.put(`${BASE_URL}`, entity))
      .then((value: any) => this.processPayload(value))
      .catch((error: any) => console.error(error));
  }

  /**
   * Deletes a User entity by ID.
   * @param id User ID.
   * @returns Promise with API response data.
   */
  public Delete(id: string) {
    return this.exec(Axios.delete(`${BASE_URL}/${id}`))
      .then((value: any) => this.processPayload(value))
      .catch((error: any) => console.error(error));
  }
}
