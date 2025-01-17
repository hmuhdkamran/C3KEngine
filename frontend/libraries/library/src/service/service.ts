/**
 * StoreService is an abstract class that provides common functionality for handling API responses and generating page parameters.
 * It implements the IStoreService interface.
 */
import Axios from 'axios';

import { useNotification, PayloadMapper, type IStoreService, TokenHelper, PayloadMessageTypes, type IPayload } from '..'

export abstract class StoreService implements IStoreService {
  constructor() { }
  /**
     * Handles a successful API response by converting the data to an IPayload object.
     * @param data The response data.
     * @returns A Promise resolving to an IPayload object.
     */
  handleFulfilled<T>(data: T): Promise<IPayload<T>> {
    return Promise.resolve(new PayloadMapper().fromObject<T>(data) as IPayload<T>)
  }

  /**
     * Handles a failed API response by converting the reason to an IPayload object.
     * @param reason The reason for the failure.
     * @returns A Promise rejecting with an IPayload object.
     */
  handleRejection<T>(reason: any): Promise<IPayload<T>> {
    return Promise.reject(new PayloadMapper().fromObject<T>(reason) as IPayload<T>)
  }

  /**
     * Processes an IPayload object and extracts the data based on specified message types.
     * @param payload The IPayload object to process.
     * @param options Additional processing options.
     * @returns A Promise resolving to the extracted data.
     */
  processPayload<T>(payload: IPayload<T>): Promise<T> {
    const { addNotification } = useNotification();

    const messageTypeIds = [PayloadMessageTypes.error, PayloadMessageTypes.failure]
    if (messageTypeIds.includes(payload.result)) {
      addNotification(JSON.stringify(payload), 'error', 'top-right', 'Error', 3000);
      return Promise.reject({} as T)
    }
    return Promise.resolve(payload.data as T)
  }

  /**
     * Executes a Promise and handles fulfillment or rejection using the provided callbacks.
     * @param cb The Promise to execute.
     * @returns A Promise resolving to an IPayload object.
     */
  exec<T>(cb: Promise<{}>): Promise<IPayload<T>> {
    const onFulfilled = (value: any) => this.handleFulfilled<T>(value)
    const onRejection = (reason: any) => this.handleRejection<T>(reason)

    return cb.then(onFulfilled, onRejection);
  }

  request<T>(method: 'GET' | 'POST' | 'PUT' | 'DELETE', url: string, data?: any, auth: boolean = true): Promise<IPayload<T>> {
    const headers = auth ? { Authorization: TokenHelper.getBearerToken().Authorization } : undefined
    return this.exec(Axios({ method, url, data, headers }))
  }

  get<T>(url: string, auth: boolean = true): Promise<IPayload<T>> {
    return this.request<T>('GET', url, undefined, auth)
  }

  post<T>(url: string, data: any, auth: boolean = true): Promise<IPayload<T>> {
    return this.request<T>('POST', url, data, auth)
  }

  put<T>(url: string, data: any, auth: boolean = true): Promise<IPayload<T>> {
    return this.request<T>('PUT', url, data, auth)
  }

  delete<T>(url: string, auth: boolean = true): Promise<IPayload<T>> {
    return this.request<T>('DELETE', url, undefined, auth)
  }
}
