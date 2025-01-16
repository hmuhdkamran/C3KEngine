import type { IPayload } from '../models'

export interface IStoreService {
  exec: <T>(cb: Promise<{}>) => Promise<IPayload<T>>
  request: <T>(method: 'GET' | 'POST' | 'PUT' | 'DELETE', url: string, data?: any, auth?: boolean) => Promise<IPayload<T>>
}