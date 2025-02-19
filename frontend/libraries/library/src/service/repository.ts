import { GlobalConfig, StoreService, type IRepository } from '@/index'

export class RepositoryService<TEntity> extends StoreService implements IRepository<TEntity> {
  private readonly baseUrl: string

  constructor(baseUrl: string) {
    super()

    this.baseUrl = `${GlobalConfig.uri.services}${baseUrl}`
  }

  getAll(): Promise<TEntity[]> {
    return this.get<TEntity[]>(`${this.baseUrl}`, true)
      .then((value) => this.processPayload<TEntity[]>(value))
      .catch((error) => {
        console.error('Error in getAll:', error)
        throw error
      })
  }

  getByFilter(filter: string): Promise<TEntity[]> {
    return this.get<TEntity[]>(`${this.baseUrl}/${filter}`, true)
      .then((value) => this.processPayload<TEntity[]>(value))
      .catch((error) => {
        console.error('Error in getByFilter:', error)
        throw error
      })
  }

  add(entity: TEntity): Promise<boolean> {
    return this.post<boolean>(`${this.baseUrl}`, entity, true)
      .then((value) => this.processPayload<boolean>(value))
      .catch((error) => {
        console.error('Error in add:', error)
        throw error
      })
  }

  update(entity: TEntity): Promise<boolean> {
    return this.put<boolean>(`${this.baseUrl}`, entity, true)
      .then((value) => this.processPayload<boolean>(value))
      .catch((error) => {
        console.error('Error in update:', error)
        throw error
      })
  }

  remove(id: string): Promise<boolean> {
    return this.delete<boolean>(`${this.baseUrl}/${id}`, true)
      .then((value) => this.processPayload<boolean>(value))
      .catch((error) => {
        console.error('Error in remove:', error)
        throw error
      })
  }
}
