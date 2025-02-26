import type { RepositoryService } from '@/service'
import { type Ref } from 'vue'

export interface BaseActionsState<T> {
  repository: RepositoryService<T>
  shouldUpdate: Ref<boolean>
  item: Ref<T>
  items: Ref<T[]>
  isLoading: Ref<boolean>
  filteringText: Ref<string>
  primaryKey: keyof T
}

export function generateBaseActions<T extends Record<string, any>>(state: BaseActionsState<T>) {
  return {
    async getItems(filter = '') {
      state.isLoading.value = true
      state.filteringText.value = filter
      try {
        const response = filter
          ? await state.repository.getByFilter(filter)
          : await state.repository.getAll()
        state.items.value = response
      } catch (err) {
        console.error(err)
      } finally {
        state.isLoading.value = false
      }
    },
    async createOrUpdateItem(itemToSave: T) {
      state.isLoading.value = true
      try {
        await (state.shouldUpdate.value
          ? state.repository.update(itemToSave)
          : state.repository.add(itemToSave))
        await this.getItems(state.filteringText.value)
      } catch (err) {
        console.error(err)
      } finally {
        state.isLoading.value = false
      }
    },
    async deleteItem(itemToDelete: T) {
      state.isLoading.value = true
      try {
        const id = itemToDelete[state.primaryKey]
        await state.repository.remove(id)
        await this.getItems(state.filteringText.value)
      } catch (err) {
        console.error(err)
      } finally {
        state.isLoading.value = false
      }
    },
  }
}
