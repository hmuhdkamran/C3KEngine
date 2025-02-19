import type { IPayload } from '../models'

export interface IStoreService {
  exec: <T>(cb: Promise<{}>) => Promise<IPayload<T>>
  request: <T>(method: 'GET' | 'POST' | 'PUT' | 'DELETE', url: string, data?: any, auth?: boolean) => Promise<IPayload<T>>
}

export interface IRepository<TEntity> {
  getAll(): Promise<TEntity[]>;
  getByFilter(filter: string): Promise<TEntity[]>;
  add(entity: TEntity): Promise<boolean>;
  update(entity: TEntity): Promise<boolean>;
  remove(id: string): Promise<boolean>;
}
