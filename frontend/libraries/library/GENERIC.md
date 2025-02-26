# Generic Implementation of the Repository

## 1. Create a Custom Repository Class
Extend the generic RepositoryService for your specific entity (e.g., Role) and add new methods.

### role-repository.ts
```typescript
import { RepositoryService } from './generic-repository';
import { IRole } from '@/models';

export class RoleRepository extends RepositoryService<IRole> {
  constructor() {
    super('auth/role/roles'); // Base URL for roles
  }

  getInactive(): Promise<IRole[]> {
    return this.get<IRole[]>('/inactive');
  }

  getRolesByStatus(status: string): Promise<IRole[]> {
    return this.get<IRole[]>(`/status/${status}`);
  }
}
```
## 2. Generate Base Actions
Create a helper function to generate standard CRUD actions using the provided repository and state.

### base-actions.ts
```typescript
import type { IRepository } from '@/index';
import { ref } from 'vue';

export interface BaseActionsState<T> {
  repository: IRepository<T>;
  shouldUpdate: Ref<boolean>;
  items: Ref<T[]>;
  isLoading: Ref<boolean>;
  filteringText: Ref<string>;
  primaryKey: keyof T;
}

export function generateBaseActions<T extends Record<string, any>>(
  state: BaseActionsState<T>,
) {
  return {
    async getItems(filter = '') {
      state.isLoading.value = true;
      state.filteringText.value = filter;
      try {
        const response = filter
          ? await state.repository.getByFilter(filter)
          : await state.repository.getAll();
        state.items.value = response;
      } catch (err) {
        console.error(err);
      } finally {
        state.isLoading.value = false;
      }
    },
    async createOrUpdateItem(itemToSave: T) {
      state.isLoading.value = true;
      try {
        await (state.shouldUpdate.value
          ? state.repository.update(itemToSave)
          : state.repository.add(itemToSave));
        await this.getItems(state.filteringText.value);
      } catch (err) {
        console.error(err);
      } finally {
        state.isLoading.value = false;
      }
    },
    async deleteItem(itemToDelete: T) {
      state.isLoading.value = true;
      try {
        const id = itemToDelete[state.primaryKey];
        await state.repository.remove(id);
        await this.getItems(state.filteringText.value);
      } catch (err) {
        console.error(err);
      } finally {
        state.isLoading.value = false;
      }
    },
  };
}
```

## 3. Create a Custom Store
Define a custom store for your entity that uses the custom repository and merges base actions with custom ones.

### role-store.ts
```typescript
import type { IRole } from '@/models';
import { defineStore } from 'pinia';
import { generateBaseActions, BaseActionsState } from './base-actions';
import { RoleRepository } from './role-repository';
import { ref } from 'vue';

export const useRoleRolesStore = defineStore('RoleRoles', () => {
  const repository = new RoleRepository();
  const primaryKey = 'RoleId' as keyof IRole;

  // State variables
  const items = ref<IRole[]>([]);
  const item = ref<IRole>();
  const isLoading = ref(false);
  const shouldUpdate = ref(false);
  const filteringText = ref('');

  // Generate base actions using the repository and state
  const baseActions = generateBaseActions({
    repository,
    shouldUpdate,
    items,
    isLoading,
    filteringText,
    primaryKey,
  });

  // Custom actions
  const customActions = {
    async getInactiveRoles() {
      isLoading.value = true;
      try {
        const response = await repository.getInactive();
        items.value = response;
      } catch (err) {
        console.error(err);
      } finally {
        isLoading.value = false;
      }
    },
    async getRolesByStatus(status: string) {
      isLoading.value = true;
      try {
        const response = await repository.getRolesByStatus(status);
        items.value = response;
      } catch (err) {
        console.error(err);
      } finally {
        isLoading.value = false;
      }
    },
  };

  return {
    item,
    items,
    isLoading,
    shouldUpdate,
    ...baseActions,
    ...customActions,
  };
});
```