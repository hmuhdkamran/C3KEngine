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

        <form class="grid grid-cols-1 gap-4">
            <div class="grid grid-cols-1 gap-4">
                <div class="grid grid-cols-2 gap-4">
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

                    <FormInput v-if="!store.shouldUpdate" id="ConfirmPassword" label="Confirm Password"
                        v-model="confirmPassword" type="password"
                        :validators="[() => confirmedValidator(confirmPassword, userItem.Password)]"
                        :validatorArgs="[[userItem.Password]]" required
                        @validation="validationResults.confirmPassword = $event" />

                    <FormSelect v-if="store.shouldUpdate" id="StatusId" label="Status:" v-model="userItem.StatusId"
                        :options="statusStore.items" :validators="[requiredValidator]" valueKey="StatusId"
                        labelKey="FullName" @validation="validationResults.statusId = $event" />
                </div>
            </div>

            <div class="grid grid-cols-1 gap-4">
                <div class="grid grid-cols-2 gap-4">
                    <div class="col-span-1 overflow-auto max-h-60">
                        <h3 class="text-sm font-medium text-gray-700 py-3 sticky top-0 bg-white z-10">Roles:</h3>
                        <ul role="listbox" aria-label="role lists" class="grid grid-cols-2 gap-2">
                            <li v-for="item in roleStore.items" :key="item.RoleId" tabindex="-1" role="option">
                                <FormCheckBox :id="item.RoleId" v-model="selectedRoles" :label="item.FullName"
                                    :value="item.RoleId" :icon="null"/>
                            </li>
                        </ul>
                    </div>

                    <div class="col-span-1 overflow-auto max-h-60">
                        <h3 class="text-sm font-medium text-gray-700 py-3 sticky top-0 bg-white z-10">Products:</h3>
                        <ul role="listbox" aria-label="product lists" class="grid grid-cols-1 gap-2">
                            <li v-for="item in productStore.items" :key="item.ProductId" tabindex="-1" role="option">
                                <FormCheckBox :id="item.ProductId" v-model="selectedProducts" :label="item.FullName"
                                    :value="item.ProductId" :icon="item.Icon" />
                            </li>
                        </ul>
                    </div>
                </div>
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
@media (max-width: 1200px) {
    .grid-cols-2 {
        grid-template-columns: 1fr;
    }
}

@media (max-width: 768px) {
    .grid-cols-2 {
        grid-template-columns: 1fr;
    }
}

@media (max-width: 480px) {
    .grid-cols-2 {
        grid-template-columns: 1fr;
    }
}
</style>