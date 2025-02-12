import { acceptHMRUpdate, defineStore } from 'pinia'
import { RepositoryService } from 'c3-library';
import { IStatus } from '~/models/setup/setup';

export const useSetupStatusStore = defineStore('SetupStatus', () => {
    const service = new RepositoryService<IStatus>('auth/setup/status');
    const items = ref<Array<IStatus>>([])
    const item = ref<IStatus>()
    const isLoading = ref(false)
    const dialogVisible = ref(false)
    const shouldUpdate = ref(false)
    const searchText = ref('')

    async function getItems() {
        isLoading.value = true;
        try {
            items.value = [];
            const response = await service.getAll();
            items.value = response as Array<IStatus>;
        } catch (err) {
            console.error(`Error: ${err}`)
        } finally {
            isLoading.value = false;
        }
    }

    async function createOrUpdateItem(item: IStatus) {
        isLoading.value = true;
        try {
            await shouldUpdate ? service.update(item) : service.add(item);
            getItems();
        } catch (err) {
            console.error(`Error: ${err}`)
        } finally {
            isLoading.value = false;
        }
    }

    async function deleteItem(item: IStatus) {
        isLoading.value = true;
        try {
            await service.remove(item.StatusId);
            getItems();
        } catch (err) {
            console.error(`Error: ${err}`)
        } finally {
            isLoading.value = false;
        }
    }

    return {
        item,
        items,
        isLoading,
        dialogVisible,
        shouldUpdate,
        searchText,
        getItems,
        createOrUpdateItem,
        deleteItem
    }
})
if (import.meta.hot)
    import.meta.hot.accept(acceptHMRUpdate(useSetupStatusStore, import.meta.hot))