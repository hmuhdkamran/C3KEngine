/**
 * StoreService is an abstract class that provides common functionality for handling API responses and generating page parameters.
 * It implements the IStoreService interface.
 */
import type { IPayload } from '../models'
import { PayloadMessageTypes } from '../models'

import type { IStoreService } from './iservice'
import { PayloadMapper } from './payload-mapper'

import { message } from '@/plugins/helper'

interface IProcessPayloadOptions {
  timeout?: number
  uri?: string
  messageTypeIds?: string[]
}

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
  processPayload<T>(payload: IPayload<T>, options?: IProcessPayloadOptions): Promise<T> {
    const msg = payload.message

    options = options || {}

    const messageTypeIds = options.messageTypeIds || [
      PayloadMessageTypes.error,
      PayloadMessageTypes.failure,
    ]

    const messageTypeId = messageTypeIds.find(o => o === msg.messageTypeId)

    if (messageTypeId) {
      message('Failed', msg.text, messageTypeId)
      return Promise.reject({} as T)
    }
    else {
      return Promise.resolve(payload.data as T);
    }
  }

  /**
     * Executes a Promise and handles fulfillment or rejection using the provided callbacks.
     * @param cb The Promise to execute.
     * @returns A Promise resolving to an IPayload object.
     */
  exec<T>(cb: Promise<{}>): Promise<IPayload<T>> {
    const onFulfilled = (value: any) => this.handleFulfilled<T>(value)
    const onRejection = (reason: any) => this.handleRejection<T>(reason)

    return cb.then(onFulfilled, onRejection)
  }
}
