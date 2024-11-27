import { defineStore } from 'pinia';
import { ref } from 'vue';
import type { IStatus } from '@/models';
import { StatusService } from '@/services'; // Assuming you have this path
import { useApplicationEventStore } from 'c3k-library';

export const useRoleStatusStore = defineStore('roleStatusStore', {
    state: () => ({
        items: ref<IStatus[]>([]),
        entity: ref<IStatus | null>(null),
        error: ref<string | null>(null),
        loading: ref<boolean>(false),
        store: useApplicationEventStore()
    }),
    actions: {
        /**
         * Handles all operations (fetch, add, update, delete) based on provided action type
         * @param action The action type: 'get', 'add', 'update', or 'delete'
         * @param entity The item entity to be added or updated (optional for 'get' and 'delete')
         * @param filter Optional filter to get a item (only used for 'get' and 'getFindBy')
         */
        async execute(action: 'get' | 'add' | 'edit' | 'delete', entity?: IStatus, filter?: string) {
            this.loading = true;
            this.error = null;

            const itemsService = new StatusService();

            try {
                switch (action) {
                    case 'get':
                        // Fetch all items or specific filtered items
                        const response = filter
                            ? await itemsService.GetFindBy(filter)
                            : await itemsService.GetAll();

                        this.items = (response as any) as IStatus[];
                        break;

                    case 'add':
                        // Add a new item
                        if (entity) {
                            await itemsService.AddOne(entity);
                            this.items.push(entity);
                        }
                        break;

                    case 'edit':
                        // Update an existing item
                        if (entity) {
                            await itemsService.Update(entity);
                            const index = this.items.findIndex(item => item.StatusId === entity?.StatusId);
                            if (index !== -1) {
                                this.items[index] = entity;
                            }
                        }
                        break;

                    case 'delete':
                        // Delete a item by ID
                        if (this.entity?.StatusId) {
                            await itemsService.Delete(this.entity.StatusId);
                            this.items = this.items.filter(item => item.StatusId !== this.entity?.StatusId);
                        }
                        break;

                    default:
                        throw new Error("Invalid action type");
                }
            } catch (error) {
                this.error = error instanceof Error ? error.message : "An unknown error occurred";
            } finally {
                this.loading = false;
                this.store.resetOperation();
            }
        },

        /**
         * Set the selected item entity
         * @param item The item to be selected
         */
        selectItem(item: IStatus, action: string) {
            this.entity = item;
            this.store.setDrawerEvent({ Open: true, Title: action, OperationType: action.toLowerCase().split(' ')[0] });
        },

        /**
         * Clear the selected item
         */
        clearSelectedItem() {
            this.entity = null;
        }
    },
    getters: {
        /**
         * Get a list of all items
         */
        allItems: (state) => state.items,

        /**
         * Get the currently selected item
         */
        selectedItem: (state) => state.entity,

        /**
         * Get any error messages
         */
        errorMessage: (state) => state.error
    }
});
