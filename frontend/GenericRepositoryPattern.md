# Generic Repository Pattern Implementation with Pinia and TypeScript

A scalable, maintainable, and reusable repository pattern implementation using Vue.js, Pinia, and TypeScript. This pattern separates API calls from component logic, promotes code reusability, and enhances testability.

## Table of Contents
1. **Introduction**
2. **Core Components**
   - Repository Service (Generic)
   - Store Service (State Management)
3. **Implementation Steps**
   - [1. Create a Custom Repository Class](#1-create-a-custom-repository-class)
   - [2. Generate Base Actions](#2-generate-base-actions)
   - [3. Create a Custom Store](#3-create-a-custom-store)
4. **Usage Example**
5. **Key Benefits**
6. **Conclusion**

---

## 1. Create a Custom Repository Class

Extend the generic `RepositoryService` to add entity-specific API calls while retaining core CRUD operations.


###role-repository.ts
```typescript
import { RepositoryService } from './generic-repository';
import { IRole } from '@/models';

export class RoleRepository extends RepositoryService<IRole> {
  constructor() {
    super('auth/role/roles'); // Base URL for roles
  }

  /**
   * Fetches all inactive roles from the API.
   */
  getInactive(): Promise<IRole[]> {
    return this.get<IRole[]>('/inactive');
  }

  /**
   * Fetches roles filtered by a specific status.
   * @param status The status to filter roles (e.g., "active", "inactive")
   */
  getRolesByStatus(status: string): Promise<IRole[]> {
    return this.get<IRole[]>(`/status/${status}`);
  }
}
```

**Purpose**: 
- Centralizes API logic for roles, adding custom endpoints alongside generic CRUD.
- Leverages TypeScript generics for type safety (`IRole`).

---

## 2. Generate Base Actions

Reusable CRUD actions abstracted into a helper function for minimal code duplication across stores.

### base-actions.ts
```typescript
import type { IRepository } from '@/index';
import { ref, Ref } from 'vue';

export interface BaseActionsState<T> {
  repository: IRepository<T>;
  shouldUpdate: Ref<boolean>;
  items: Ref<T[]>;
  isLoading: Ref<boolean>;
  filteringText: Ref<string>;
  primaryKey: keyof T;
}

export function generateBaseActions<T extends Record<string, any>>(
  state: BaseActionsState<T>
) {
  return {
    /**
     * Fetches all items or applies a filter based on user input.
     * @param filter Optional filter string
     */
    async getItems(filter = '') {
      state.isLoading.value = true;
      state.filteringText.value = filter;
      
      try {
        const response = filter
          ? await state.repository.getByFilter(filter)
          : await state.repository.getAll();
        state.items.value = response;
      } catch (err) {
        console.error('Error while fetching items:', err);
      } finally {
        state.isLoading.value = false;
      }
    },
    /**
     * Creates or updates an item in the repository and refreshes the list.
     * @param itemToSave The item to save
     */
    async createOrUpdateItem(itemToSave: T) {
      state.isLoading.value = true;

      try {
        state.shouldUpdate.value
          ? await state.repository.update(itemToSave)
          : await state.repository.add(itemToSave);
        
        await state.repository.getItems(state.filteringText.value);
      } catch (err) {
        console.error('Error while saving item:', err);
      } finally {
        state.isLoading.value = false;
      }
    },
    /**
     * Deletes an item from the repository and refreshes the list.
     * @param itemToDelete The item to delete
     */
    async deleteItem(itemToDelete: T) {
      state.isLoading.value = true;

      try {
        const id = itemToDelete[state.primaryKey];
        await state.repository.remove(id);
        await this.getItems(state.filteringText.value);
      } catch (err) {
        console.error('Error while deleting item:', err);
      } finally {
        state.isLoading.value = false;
      }
    }
  };
}
```

**Key Features**:
- **Responsiveness**: Uses Vue's `ref` for reactive state management.
- **Reusability**: Once implemented, can be used with any entity (users, products, etc.).
- **Error Handling**: Centralizes try/catch blocks to streamline development.

---

## 3. Create a Custom Store

Merges generic CRUD actions with entity-specific logic using Pinia.

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

  // Generate and merge base actions
  const baseActions = generateBaseActions({
    repository,
    shouldUpdate,
    items,
    isLoading,
    filteringText,
    primaryKey,
  });

  // Custom actions specific to roles
  const customActions = {
    /**
     * Fetches all inactive roles and updates the store's state.
     */
    async getInactiveRoles() {
      isLoading.value = true;

      try {
        const response = await repository.getInactive();
        items.value = response;
      } catch (err) {
        console.error('Error while fetching inactive roles:', err);
      } finally {
        isLoading.value = false;
      }
    },
    /**
     * Fetches roles filtered by a specific status.
     * @param status The status to filter roles
     */
    async getRolesByStatus(status: string) {
      isLoading.value = true;

      try {
        const response = await repository.getRolesByStatus(status);
        items.value = response;
      } catch (err) {
        console.error('Error while fetching roles by status:', err);
      } finally {
        isLoading.value = false;
      }
    }
  };

  // Combine state and actions
  return {
    item,
    items,
    isLoading,
    shouldUpdate,
    ...baseActions,  // Include generic CRUD actions
    ...customActions // Add role-specific actions
  };
});
```

**Advantages**:
- **SOLID Principles**: Separation of concerns between data fetching and state management.
- **Maintainability**: Easily extendable with new custom actions without modifying base code.
- **Performance**: Single source of truth for state, ensuring consistency.

---

## 4. Usage Example

Consume the store in a Vue component to perform role-related operations.

```vue
<script setup>
import { useRoleRolesStore } from '@/stores/role-store';

const roleStore = useRoleRolesStore();

// Fetch inactive roles
roleStore.getInactiveRoles();

// Create a new role
roleStore.createOrUpdateItem({
  RoleId: '123',
  Name: 'Admin',
  IsActive: true
});
</script>
```

---

## 5. Key Benefits

- **Code Reusability**: Generic CRUD operations reusable across all entities.
- **Scalability**: Easily add new entities by extending the repository pattern.
- **Testability**: Isolated API logic simplifies unit and e2e testing.
- **Maintainability**: Centralized logic reduces the risk of human error.

---

## 6. Conclusion

This pattern provides a robust foundation for applications requiring scalable state management and API interaction. By separating concerns and leveraging TypeScript, this approach ensures clean, maintainable, and efficient code.