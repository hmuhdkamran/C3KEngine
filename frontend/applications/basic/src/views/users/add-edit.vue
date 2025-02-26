<script setup lang="ts">
import { setFormOpen, formStatus } from '@/stores/edit-form';
import type { IUser } from '@/models';
import {
    FormInput, FormSelect,
    DialogBox, newGuid, useSystemStore, validate, requiredValidator, rangeValidator,
    emailValidator,
    passwordValidator,
    confirmedValidator, type ISignupUsers
} from 'c3-library';
import {
    useRoleUserStore,
    useRoleRolesStore,
    useRoleUserRoleMapStore,
    useSetupStatusStore,
    useRoleProductsStore,
    useRoleUserProductMapsStore
} from '@/stores';
import { computed, ref, watch, onMounted } from 'vue';
import { AuthenticationService } from '@/services/authentication-service';
import FormCheckBox from './FormCheckBox.vue';

// Store instances
const color = useSystemStore();
const store = useRoleUserStore();
const roleStore = useRoleRolesStore();
const userRoleStore = useRoleUserRoleMapStore();
const productStore = useRoleProductsStore();
const userProductStore = useRoleUserProductMapsStore();
const statusStore = useSetupStatusStore();
const auth_service = new AuthenticationService();

// Form data and validation
const userItem = computed(() => store.item || ({} as IUser));
const selectedRoles = ref<string[]>([]);
const selectedProducts = ref<string[]>([]);
const confirmPassword = ref('');

const validationResults = ref({
    displayName: false,
    language: false,
    Username: false,
    Password: false,
    confirmPassword: false,
    statusId: false
});

const languageOptions = [
    { value: 'en-US', label: 'English' },
    { value: 'ur-PK', label: 'Urdu' }
];

// Initialize form data on mount
onMounted(() => {
    if (store.shouldUpdate) {
        selectedRoles.value = userRoleStore.items.map(i => i.RoleId);
        selectedProducts.value = userProductStore.items.map(i => i.ProductId);
    }
});

// Update selected values when store items change
watch(() => userRoleStore.items, () => {
    selectedRoles.value = userRoleStore.items.map(i => i.RoleId);
}, { deep: true });

watch(() => userProductStore.items, () => {
    selectedProducts.value = userProductStore.items.map(i => i.ProductId);
}, { deep: true });

// Toggle selection functions
const toggleRole = (roleId: string) => {
    const index = selectedRoles.value.indexOf(roleId);

    if (index === -1) {
        selectedRoles.value.push(roleId);
    } else {
        selectedRoles.value.splice(index, 1);
    }
};

const toggleProduct = (productId: string) => {
    const index = selectedProducts.value.indexOf(productId);
    if (index === -1) {
        selectedProducts.value.push(productId);
    } else {
        selectedProducts.value.splice(index, 1);
    }
};

// Form submission
const saveUser = () => {
    const excludeFields = store.shouldUpdate ? ['Password', 'confirmPassword'] : [];

    if (!validate(validationResults.value, excludeFields)) {
        return;
    }

    if (!store.shouldUpdate) {
        const entity: ISignupUsers = {
            user_id: newGuid(),
            username: userItem.value.Username as string,
            display_name: userItem.value.DisplayName as string,
            language: userItem.value.Language as string,
            password: userItem.value.Password as string,
            status_id: statusStore.items[0].StatusId as string,
            roles: selectedRoles.value,
            products: selectedProducts.value
        };

        auth_service.signup(entity).then(() => {
            store.getItems();
            console.log('User saved successfully');
            close();
        }).catch(error => {
            console.error('Error creating user:', error);
            // Handle API errors here
        });
    } else {
        store.createOrUpdateItem(store.item as IUser)
            .then(() => {
                close();
            })
            .catch((error) => {
                console.error('Error updating user:', error);
            });
    }
};

// Close form and reset
const close = () => {
    selectedRoles.value = [];
    selectedProducts.value = [];
    confirmPassword.value = '';

    store.item = { UserId: '', Username: '', DisplayName: '', Language: '', Password: '', Salt: '', StatusId: '' };

    store.shouldUpdate = false;
    setFormOpen(false);
};
</script>

<template>
    <DialogBox :show="formStatus" :showClose="false" @close="close()">
        <template #header>
            <h2 class="text-lg font-semibold">{{ store.shouldUpdate ? 'User Edit Form' : 'Add User Form' }}</h2>
            <p class="text-sm text-gray-500">{{ store.shouldUpdate ? 'Edit Record' : 'Add Record' }}</p>
        </template>

        <form class="grid grid-cols-1 sm:grid-cols-2 gap-4" @submit.prevent="saveUser">
            <FormInput id="displayName" label="Name" v-model="userItem.DisplayName"
                :validators="[requiredValidator, (value) => rangeValidator(value, 3, 50)]"
                :validatorArgs="[[], [3, 50]]" required @validation="validationResults.displayName = $event" />

            <FormInput id="Email" label="Email" v-model="userItem.Username" type="email"
                :validators="[requiredValidator, emailValidator]" required
                @validation="validationResults.Username = $event" />

            <FormSelect id="Language" label="Language:" v-model="userItem.Language" :options="languageOptions"
                :validators="[requiredValidator]" @validation="validationResults.language = $event" />

            <FormInput v-if="!store.shouldUpdate" id="Password" label="Password" v-model="userItem.Password"
                type="password" :validators="[requiredValidator, passwordValidator]" required
                @validation="validationResults.Password = $event" />

            <!-- Confirm Password input -->
            <FormInput v-if="!store.shouldUpdate" id="ConfirmPassword" label="Confirm Password"
                v-model="confirmPassword" type="password"
                :validators="[() => confirmedValidator(confirmPassword, userItem.Password)]"
                :validatorArgs="[[userItem.Password]]" required
                @validation="validationResults.confirmPassword = $event" />

            <FormSelect v-if="store.shouldUpdate" id="StatusId" label="Status:" v-model="userItem.StatusId"
                :options="statusStore.items" :validators="[requiredValidator]" valueKey="StatusId" labelKey="FullName"
                @validation="validationResults.statusId = $event" />

            <!-- Roles Section -->
            <div class="col-span-1 sm:col-span-2">
                <h3 class="text-sm font-medium text-gray-700 mb-2">Roles:</h3>
                <ul role="listbox" aria-label="role lists" class="grid grid-cols-1 sm:grid-cols-2 gap-2">
                    <li v-for="item in roleStore.items" :key="item.RoleId" tabindex="-1" role="option">
                        <FormCheckBox :id="item.RoleId" v-model="item.RoleId" :label="item.FullName" :icon="item.Icon" />
                        <!-- <label :for="item.RoleId" class="flex items-center cursor-pointer my-1">
                            <div class="relative">
                                <input type="checkbox" :id="item.RoleId" :checked="selectedRoles.includes(item.RoleId)"
                                    @change="toggleRole(item.RoleId)" class="sr-only">
                                <div class="block w-14 h-6 rounded-sm transition"
                                    :class="selectedRoles.includes(item.RoleId) ? 'bg-green-500' : 'bg-gray-600'"></div>
                                <div class="dot absolute left-1 top-1 bg-white w-6 h-4 rounded-sm transition"
                                    :style="{ 'transform': selectedRoles.includes(item.RoleId) ? 'translateX(7px)' : 'translateX(0)' }">
                                </div>
                            </div>
                            <div class="ml-3 text-gray-700 font-medium">
                                {{ item.FullName }}
                            </div>
                        </label> -->
                    </li>
                </ul>
            </div>

            <!-- Products Section -->
            <div class="col-span-1 sm:col-span-2 mt-4">
                <h3 class="text-sm font-medium text-gray-700 mb-2">Products:</h3>
                <ul role="listbox" aria-label="product lists" class="grid grid-cols-1 sm:grid-cols-2 gap-2">
                    <li v-for="item in productStore.items" :key="item.ProductId" tabindex="-1" role="option">
                        <FormCheckBox :id="item.ProductId" v-model="item.ProductId" :label="item.FullName" :icon="item.Icon" />
                        <!-- <label :for="item.ProductId" class="flex items-center cursor-pointer my-1">
                            <div class="relative">
                                <input type="checkbox" :id="item.ProductId"
                                    :checked="selectedProducts.includes(item.ProductId)"
                                    @change="toggleProduct(item.ProductId)" class="sr-only">
                                <div class="block w-14 h-6 rounded-sm transition"
                                    :class="selectedProducts.includes(item.ProductId) ? 'bg-green-500' : 'bg-gray-600'">
                                </div>
                                <div class="dot absolute left-1 top-1 bg-white w-6 h-4 rounded-sm transition"
                                    :style="{ 'transform': selectedProducts.includes(item.ProductId) ? 'translateX(7px)' : 'translateX(0)' }">
                                </div>
                            </div>
                            <div class="ml-3 text-gray-700 font-medium">
                                {{ item.FullName }}
                            </div>
                        </label> -->
                    </li>
                </ul>
            </div>
        </form>

        <template #footer>
            <div class="flex justify-end gap-3 mt-4">
                <button type="button"
                    class="px-3 py-1.5 bg-gray-200 rounded-sm hover:bg-gray-300 transition cursor-pointer"
                    @click="close()">
                    Cancel
                </button>
                <button type="button" @click="saveUser" class="px-3 py-1.5 text-white rounded-sm cursor-pointer"
                    :style="{ backgroundColor: color.application.primaryColor }">
                    Save
                </button>
            </div>
        </template>
    </DialogBox>
</template>

<style scoped>
/* Use CSS transitions for smoother toggle animations */
.dot {
    transition: transform 0.3s ease;
}
</style>