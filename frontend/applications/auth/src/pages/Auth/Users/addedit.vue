<script setup lang='ts'>
import type { FormInst, FormRules } from 'naive-ui';
import { storeToRefs } from 'pinia';
import { useI18n } from 'vue-i18n';
import { reactive, watch, computed, ref } from 'vue';

import { IUser } from '~/models/roles/IUser';
import { useRoleUserStore } from '~/store/role/user-store';

const { t } = useI18n();
const formRef = ref<FormInst | null>(null);
const store = useRoleUserStore();
const { item, dialogVisible } = storeToRefs(store);

// Form data model including confirmation password
const formData = reactive<{
    UserId: string;
    Username: string;
    DisplayName: string;
    Language: string;
    Password: string;
    ConfirmPassword: string;
    StatusId: string;
    Salt?: string;
}>({
    UserId: '',
    Username: '',
    DisplayName: '',
    Language: '',
    Password: '',
    ConfirmPassword: '',
    StatusId: '',
    Salt: '', // Assuming it's part of the model but not displayed
});

// Watch dialogVisible to reset form data when modal opens
watch(dialogVisible, (val) => {
    if (val && item.value) {
        Object.assign(formData, {
            ...item.value,
            ConfirmPassword: '', // Reset confirmation password on edit
        });
    }
});

// Language options
const languageOptions = [
    { label: 'en', value: 'en' },
    { label: 'de', value: 'de' },
    { label: 'ar', value: 'ar' },
    { label: 'tr', value: 'tr' },
    { label: 'fa', value: 'fa' },
    { label: 'ch', value: 'ch' },
    { label: 'ur', value: 'ur' },
];

// Status options with translated labels
const statusOptions = computed(() => [
    { label: t('role.user.status.active'), value: 'Active' },
    { label: t('role.user.status.inactive'), value: 'Inactive' },
]);

// Form validation rules
const rules = computed(() => ({
    Username: [
        { required: true, trigger: ['blur', 'change'], message: t('role.user.username.required') },
    ],
    Language: [
        { required: true, trigger: ['blur', 'change'], message: t('role.user.language.required') },
    ],
    Password: [
        { required: true, trigger: ['blur', 'change'], message: t('role.user.password.required') },
    ],
    ConfirmPassword: [
        { required: true, trigger: ['blur', 'change'], message: t('role.user.confirmPassword.required') },
        {
            validator: (rule: any, value: string) => {
                if (value !== formData.Password) {
                    return new Error(t('role.user.passwordMismatch'));
                }
                return true;
            },
            trigger: ['blur', 'change'],
        },
    ],
    StatusId: [
        { required: true, trigger: ['blur', 'change'], message: t('role.user.status.required') },
    ],
}));

async function execute() {
    formRef.value?.validate(async (errors: any) => {
        if (!errors) {
            // Create a new user object without confirmPassword
            const newUser: IUser = {
                UserId: formData.UserId,
                Username: formData.Username,
                DisplayName: formData.DisplayName,
                Language: formData.Language,
                Password: formData.Password,
                Salt: formData.Salt as string,
                StatusId: formData.StatusId,
            };
            // Assign the new user to the store's item and save
            item.value = newUser;
            await store.createOrUpdateItem(item.value);
            dialogVisible.value = false;
        }
    });
}

</script>

<template>
    <CrudModal v-model:visible="dialogVisible" :title="t('role.user.title')" :loading="false" @onSave="execute()">
        <n-form ref="formRef" :model="formData" :rules="rules">
            <!-- User ID (read-only) -->
            <div class="form-control">
                <n-form-item path="UserId" :label="t('role.user.userId')">
                    <n-input v-model:value="formData.UserId" disabled />
                </n-form-item>
            </div>

            <!-- Username -->
            <div class="form-control">
                <n-form-item path="Username" :label="t('role.user.username')">
                    <n-input ref="usernameInput" v-model:value="formData.Username" autofocus
                        :placeholder="t('role.user.usernamePlaceholder')" />
                </n-form-item>
            </div>

            <!-- Display Name -->
            <div class="form-control">
                <n-form-item path="DisplayName" :label="t('role.user.displayName')">
                    <n-input ref="displayNameInput" v-model:value="formData.DisplayName" autofocus
                        :placeholder="t('role.user.displayNamePlaceholder')" />
                </n-form-item>
            </div>

            <!-- Language -->
            <div class="form-control">
                <n-form-item path="Language" :label="t('role.user.language')">
                    <n-select v-model:value="formData.Language" :placeholder="t('role.user.languagePlaceholder')"
                        :options="languageOptions" />
                </n-form-item>
            </div>

            <!-- Password -->
            <div class="form-control">
                <n-form-item path="Password" :label="t('role.user.password')">
                    <n-input type="password" v-model:value="formData.Password" show-password-on="click"
                        :placeholder="t('role.user.passwordPlaceholder')" />
                </n-form-item>
            </div>

            <!-- Confirm Password -->
            <div class="form-control">
                <n-form-item path="ConfirmPassword" :label="t('role.user.confirmPassword')">
                    <n-input type="password" v-model:value="formData.ConfirmPassword" show-password-on="click"
                        :placeholder="t('role.user.confirmPasswordPlaceholder')" />
                </n-form-item>
            </div>

            <!-- Status -->
            <div class="form-control">
                <n-form-item path="StatusId" :label="t('role.user.status')">
                    <n-select v-model:value="formData.StatusId" :placeholder="t('role.user.statusPlaceholder')"
                        :options="statusOptions" />
                </n-form-item>
            </div>
        </n-form>
    </CrudModal>
</template>