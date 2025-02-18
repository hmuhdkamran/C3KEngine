// ~/stores/generic.ts
import { RepositoryService } from '@/service'
import { defineStore } from 'pinia'
import { ref } from 'vue'

export function createGenericPiniaStore<T extends Record<string, any>, K extends keyof T>(
  storeId: string,
  apiPath: string,
  primaryKey: K,
) {
  return defineStore(storeId, () => {
    const service = new RepositoryService<T>(apiPath)
    const items = ref<Array<T>>([])
    const item = ref<T>()
    const isLoading = ref(false)
    const shouldUpdate = ref(false)
    const filteringText = ref('')

    async function getItems(filter: string = '') {
      isLoading.value = true
      filteringText.value = filter;
      try {
        const response =
          filter.length > 0 ? await service.getByFilter(filter) : await service.getAll()
        items.value = response as Array<T>
      } catch (err) {
        console.error(`Error: ${err}`)
      } finally {
        isLoading.value = false
      }
    }

    async function createOrUpdateItem(itemToSave: T) {
      isLoading.value = true
      try {
        ;(await shouldUpdate.value) ? service.update(itemToSave) : service.add(itemToSave)
        await getItems(filteringText.value)
      } catch (err) {
        console.error(`Error: ${err}`)
      } finally {
        isLoading.value = false
      }
    }

    async function deleteItem(itemToDelete: T) {
      isLoading.value = true
      try {
        const primaryKeyValue = itemToDelete[primaryKey]
        await service.remove(primaryKeyValue)
        await getItems(filteringText.value)
      } catch (err) {
        console.error(`Error: ${err}`)
      } finally {
        isLoading.value = false
      }
    }

    return {
      item,
      items,
      isLoading,
      shouldUpdate,
      getItems,
      createOrUpdateItem,
      deleteItem,
    }
  })
}
