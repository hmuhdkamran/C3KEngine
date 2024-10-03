
import Axios from 'axios'
import type { IUsers } from '@/models/user'
import { GlobalConfig } from '@/plugins/models'
import { StoreService } from '@/plugins/service/service'

const BASE_URL = `${GlobalConfig.uri.services}Role_Users/`

export class UsersService extends StoreService {
  /**
     * Fetches all CorrectedParts data with pagination support.
     * @param page Page number (default is 1).
     * @returns Promise with API response data.
     */
  public GetAll(page = 1, max = 10) {
    return this.exec<any>(Axios.get(`${BASE_URL}GetAll${this.PageParameters(page, max)}`))
      .then(value => this.processPayload(value))
      .catch(error => console.error(error))
  }

  /**
     * Fetches CorrectedParts data based on provided parameters with pagination support.
     * @param parameter Search parameter.
     * @param page Page number (default is 1).
     * @returns Promise with API response data.
     */
  public GetFindBy(parameter: string, page = 1, max = 10) {
    return this.exec<any>(Axios.post(`${BASE_URL}FindBy${this.PageParameters(page, max)}`, { parameter }))
      .then(value => this.processPayload(value))
      .catch(error => console.error(error))
  }

  /**
     * Adds a new CorrectedParts entity.
     * @param entity CorrectedParts entity object.
     * @returns Promise with API response data.
     */
  public AddOne(entity: IUsers) {
    return this.exec<any>(Axios.post(`${BASE_URL}Create`, entity))
      .then(value => this.processPayload(value))
      .catch(error => console.error(error))
  }

  /**
     * Updates an existing CorrectedParts entity.
     * @param entity CorrectedParts entity object.
     * @returns Promise with API response data.
     */
  public Update(entity: IUsers) {
    return this.exec<any>(Axios.post(`${BASE_URL}Update`, entity))
      .then(value => this.processPayload(value))
      .catch(error => console.error(error))
  }

  /**
     * Deletes a CorrectedParts entity.
     * @param entity CorrectedParts entity object.
     * @returns Promise with API response data.
     */
  public Delete(entity: IUsers) {
    return this.exec<any>(Axios.post(`${BASE_URL}Delete`, entity))
      .then(value => this.processPayload(value))
      .catch(error => console.error(error))
  }
}
