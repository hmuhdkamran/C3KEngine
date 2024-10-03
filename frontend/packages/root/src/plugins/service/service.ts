/**
 * StoreService is an abstract class that provides common functionality for handling API responses and generating page parameters.
 * It implements the IStoreService interface.
 */
import type { IPayload, IStoreService } from '@/types/axios';
import { PayloadMapper } from './payload-mapper';

interface IProcessPayloadOptions {
  timeout?: number;
  uri?: string;
  messageTypeIds?: string[];
}

export abstract class StoreService implements IStoreService {
  constructor() { }

  /**
   * Handles a successful API response by converting the data to an IPayload object.
   * @param data The response data.
   * @returns A Promise resolving to an IPayload object.
   */
  handleFulfilled<T>(data: T): Promise<IPayload<T>> {
    return Promise.resolve(new PayloadMapper().fromObject<T>(data) as IPayload<T>);
  }

  /**
   * Handles a failed API response by converting the reason to an IPayload object.
   * @param reason The reason for the failure.
   * @returns A Promise rejecting with an IPayload object.
   */
  handleRejection<T>(reason: unknown): Promise<IPayload<T>> {
    return Promise.reject(new PayloadMapper().fromObject<T>(reason) as IPayload<T>);
  }

  /**
   * Processes an IPayload object and extracts the data based on specified message types.
   * @param payload The IPayload object to process.
   * @param options Additional processing options.
   * @returns A Promise resolving to the extracted data.
   */
  processPayload<T>(payload: IPayload<T>, options?: IProcessPayloadOptions): Promise<T> {
    const msg = payload;

    options = options || {};

    const messageTypeIds = options.messageTypeIds || ['error', 'failure'];

    const messageTypeId = messageTypeIds.find(o => o === msg.result);

    if (messageTypeId) {
      alert(`Failed: ${msg.description}`);
      return Promise.reject({} as T);
    } else {
      return Promise.resolve(payload.data as T);
    }
  }

  /**
   * Executes a Promise and handles fulfillment or rejection using the provided callbacks.
   * @param cb The Promise to execute.
   * @returns A Promise resolving to an IPayload object.
   */
  exec<T>(cb: Promise<unknown>): Promise<IPayload<T>> {
    const onFulfilled = (value: unknown): Promise<IPayload<T>> => {
      return this.handleFulfilled(value as T); // Type assertion
    };

    const onRejection = (reason: unknown): Promise<IPayload<T>> => {
      return this.handleRejection<T>(reason);
    };

    return cb.then(onFulfilled, onRejection);
  }
}
