import { defineStore } from 'pinia';
import { ref } from 'vue';
import type { IUser } from '@/models';
import { UsersService } from '@/services/role/users-service'; // Assuming you have this path
import { useApplicationEventStore } from 'c3k-library';

export const useRoleUserStore = defineStore('roleUserStore', {
    state: () => ({
        items: ref<IUser[]>([]),
        entity: ref<IUser | null>(null),
        error: ref<string | null>(null),
        store: useApplicationEventStore()
    }),
    actions: {
        /**
         * Handles all operations (fetch, add, update, delete) based on provided action type
         * @param action The action type: 'get', 'add', 'update', or 'delete'
         * @param entity The user entity to be added or updated (optional for 'get' and 'delete')
         * @param filter Optional filter to get a user (only used for 'get' and 'getFindBy')
         */
        async execute(action: 'get' | 'add' | 'edit' | 'delete', entity?: IUser, filter?: string) {
            this.store.setLoading(true);
            this.error = null;

            const usersService = new UsersService();

            try {
                switch (action) {
                    case 'get':
                        // Fetch all users or specific filtered users
                        const response = filter
                            ? await usersService.GetFindBy(filter)
                            : await usersService.GetAll();

                        this.items = (response as any).data as IUser[];
                        break;

                    case 'add':
                        // Add a new user
                        if (entity) {
                            await usersService.AddOne(entity);
                            this.items.push(entity);
                        }
                        break;

                    case 'edit':
                        // Update an existing user
                        if (entity) {
                            await usersService.Update(entity);
                            const index = this.items.findIndex(user => user.UserId === entity?.UserId);
                            if (index !== -1) {
                                this.items[index] = entity;
                            }
                        }
                        break;

                    case 'delete':
                        // Delete a user by ID
                        if (this.entity?.UserId) {
                            await usersService.Delete(this.entity.UserId);
                            this.items = this.items.filter(user => user.UserId !== this.entity?.UserId);
                        }
                        break;

                    default:
                        throw new Error("Invalid action type");
                }
            } catch (error) {
                this.error = error instanceof Error ? error.message : "An unknown error occurred";
            } finally {
                this.store.setLoading(false);
                this.store.resetOperation();
            }
        },

        /**
         * Set the selected user entity
         * @param user The user to be selected
         */
        selectItem(user: IUser, action: string) {
            this.entity = user;
            this.store.setDrawerEvent({ Open: true, Title: action, OperationType: action.toLowerCase().split(' ')[0] });
        },

        /**
         * Clear the selected user
         */
        clearSelectedUser() {
            this.entity = null;
        }
    },
    getters: {
        /**
         * Get a list of all users
         */
        allUsers: (state) => state.items,

        /**
         * Get the currently selected user
         */
        selectedUser: (state) => state.entity,

        /**
         * Get any error messages
         */
        errorMessage: (state) => state.error
    }
});
