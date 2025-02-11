import { acceptHMRUpdate, defineStore } from 'pinia'
import { RepositoryService } from 'c3-library';
import { IUser } from '~/models/roles/IUser';

export const useRoleUserStore = defineStore('RoleUser', () => {
    const service = new RepositoryService<IUser>('auth/role/users');
    const items = ref<Array<IUser>>([])
    const item = ref<IUser>()
    const isLoading = ref(false)

    async function getItems() {
        isLoading.value = true;
        try {
            const response = await service.getAll();
            items.value = response as Array<IUser>;
        } catch (err) {
            console.error(`Error: ${err}`)
        } finally {
            isLoading.value = false;
        }
    }

    async function createOrUpdateItem(item: IUser, insert: boolean = false) {
        isLoading.value = true;
        try {
            await insert ? service.add(item) : service.update(item);
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
        getItems,
        createOrUpdateItem,
        deleteItem
    }
})
if (import.meta.hot)
    import.meta.hot.accept(acceptHMRUpdate(useRoleUserStore, import.meta.hot))