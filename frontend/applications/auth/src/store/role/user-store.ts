import { acceptHMRUpdate, defineStore } from 'pinia'
import { RepositoryService } from 'c3-library';
import { IUser } from '~/models/roles/IUser';

export const useRoleUserStore = defineStore('RoleUser', () => {
    const service = new RepositoryService<IUser>('auth/role/users');
    const items = ref<Array<IUser>>([])
    const item = ref<IUser>()
    const isLoading = ref(false)
    const dialogVisible = ref(false)
    const shouldUpdate = ref(false)
    const searchText = ref('')

    async function getItems() {
        isLoading.value = true;
        try {
            items.value = [];
            const response = await service.getAll();
            items.value = response as Array<IUser>;
        } catch (err) {
            console.error(`Error: ${err}`)
        } finally {
            isLoading.value = false;
        }
    }

    async function createOrUpdateItem(item: IUser) {
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

    async function deleteItem(item: IUser) {
        isLoading.value = true;
        try {
            await service.remove(item.UserId);
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
    import.meta.hot.accept(acceptHMRUpdate(useRoleUserStore, import.meta.hot))