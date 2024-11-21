/**
 * This service handles API calls related to Statuss. It extends the StoreService class.
 * It provides methods for fetching, adding, updating, and deleting Statuss data.
 * The methods utilize Axios for HTTP requests and Vuex for state management.
 *
 * Author: H.Muhammad Kamran
 * Email: hmuhdkamran@gmail.com
 * Contact: +92 (313 / 333) 9112 845
 */

// Importing necessary dependencies and modules.
import type { IStatus } from '@/models';
import { GlobalConfig, StoreService } from 'c3k-library';

// Setting the base URL for Status API calls.
const BASE_URL = `${GlobalConfig.uri.services}auth/setup/status`;

export class StatusService extends StoreService {
  /**
   * Fetches all Statuss data.
   * @returns Promise with API response data.
   */
  public GetAll() {
    return this.get(`${BASE_URL}`, true)
      .then((value: any) => this.processPayload(value))
      .catch((error: any) => console.error(error));
  }

  /**
   * Fetches Statuss data based on a filter.
   * @param filter Filter parameter.
   * @returns Promise with API response data.
   */
  public GetFindBy(filter: string) {
    return this.get(`${BASE_URL}/${filter}`, true)
      .then((value: any) => this.processPayload(value))
      .catch((error: any) => console.error(error));
  }

  /**
   * Adds a new Status entity.
   * @param entity Status entity object.
   * @returns Promise with API response data.
   */
  public AddOne(entity: IStatus) {
    return this.post(`${BASE_URL}`, entity, true)
      .then((value: any) => this.processPayload(value))
      .catch((error: any) => console.error(error));
  }

  /**
   * Updates an existing Status entity.
   * @param entity Status entity object.
   * @returns Promise with API response data.
   */
  public Update(entity: IStatus) {
    return this.put(`${BASE_URL}`, entity, true)
      .then((value: any) => this.processPayload(value))
      .catch((error: any) => console.error(error));
  }

  /**
   * Deletes a Status entity by ID.
   * @param id Status ID.
   * @returns Promise with API response data.
   */
  public Delete(id: string) {
    return this.delete(`${BASE_URL}/${id}`, true)
      .then((value: any) => this.processPayload(value))
      .catch((error: any) => console.error(error));
  }
}
