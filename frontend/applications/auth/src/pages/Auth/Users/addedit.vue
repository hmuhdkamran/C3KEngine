<script setup lang='ts'>
import type { FormInst, FormRules } from 'naive-ui/es/form'
import { storeToRefs } from 'pinia';
import { IUser } from '~/models/roles/IUser';
import { useRoleUserStore } from '~/store/role/user-store';

const { t } = useI18n()
const formRef = ref<FormInst | null>(null)
const store = useRoleUserStore();
const { item, dialogVisible } = storeToRefs(store)

const rules: FormRules = {
    Username: [
        {
            required: true,
            trigger: ['blur', 'change'],
            message: t('role.user.username'),
        },
    ],
}

async function execute() {
    formRef.value?.validate(async (errors: any) => {
        if (!errors) {
            await store.createOrUpdateItem(item.value as IUser);
            dialogVisible.value = false;
        }
    })
}

</script>

<template>
    <CrudModal v-model:visible="dialogVisible" :title="t('role.user.title')" :loading="false" @onSave="execute()">
        <n-form ref="formRef" :model="item" :rules="rules">
            <div class="form-control">
                <n-form-item path="username" :label="t('role.user.username')">
                    <n-input id="username" ref="usernameInput" v-model:value="(item as IUser).Username" autofocus
                        :placeholder="t('role.user.username')" />
                </n-form-item>
            </div>
            <div class="form-control">
                <n-form-item path="displayName" :label="t('role.user.displayName')">
                    <n-input id="displayName" ref="displayNameInput" v-model:value="(item as IUser).DisplayName"
                        autofocus :placeholder="t('role.user.username')" />
                </n-form-item>
            </div>
        </n-form>
    </CrudModal>
</template>