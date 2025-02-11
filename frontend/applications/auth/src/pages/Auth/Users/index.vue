<script setup lang="ts">
import { NButton, NPopconfirm, type DataTableColumns } from 'naive-ui/es/components'
import type { RowData } from 'naive-ui/es/data-table/src/interface'
import { storeToRefs } from 'pinia';
import { IUser } from '~/models/roles/IUser';
import { useRoleUserStore } from '~/store/role/user-store';
import AddEdit from './addedit.vue';
import { resolveDirective, withDirectives } from 'vue';

const { t } = useI18n()
const store = useRoleUserStore();
const { item, isLoading, searchText, dialogVisible } = storeToRefs(store)
const vPermission = resolveDirective('permission')

function getItems() {
    store.getItems();
}

onMounted(getItems);

function handleAdd() {
    dialogVisible.value = true;
}

function handleEdit(pItem: IUser) {
    item.value = pItem;
    dialogVisible.value = true;
}

function handleDelete(item: IUser) {
    store.deleteItem(item);
}

const columns: DataTableColumns<RowData> = [
    {
        title: t('role.user.username'),
        key: 'Username',
    },
    {
        title: t('role.user.displayName'),
        key: 'DisplayName',
    },
    {
        title: t('role.user.language'),
        key: 'Language',
    },
    {
        title: t('forms.action'),
        key: 'action',
        width: 80,
        render(row) {
            return h('div', {}, [
                h(NButton, {
                    size: 'small',
                    text: true,
                    onClick: () => handleEdit(row as IUser)
                }, {
                    icon: () => h('i', { class: 'fa-solid fa-pen' }),
                }),
                h(
                    NPopconfirm,
                    {
                        onPositiveClick: () => handleDelete(row as IUser),
                        onNegativeClick: () => { },
                    },
                    {
                        trigger: () =>
                            withDirectives(
                                h(
                                    NButton,
                                    {
                                        text: true,
                                        size: 'small',
                                        type: 'error',
                                        style: 'margin-left: 8px;',
                                    },
                                    {
                                        icon: () => h('i', { class: 'fa-solid fa-trash-can' }),
                                    }
                                ),
                                [[vPermission, 'delete']]
                            ),
                        default: () => h('div', {}, t('forms.deleteText')),
                    }
                )
            ]);
        }
    }
]

</script>

<template>
    <div>
        <n-page-header style="margin-bottom: 10px;">
            <template #title>
                <p class="font-bold text-lg">{{ t('role.user.title') }}</p>
            </template>
            <template #extra>
                <n-flex gap="md" justify="end">
                    <n-input v-model="searchText" type="text" size="small" placeholder="Search..." clearable
                        style="width: 180px;" />
                    <n-button type="primary" size="small" @click="handleAdd" strong style="margin-left: 6px;">
                        <i class="fa-solid fa-plus" />
                    </n-button>
                </n-flex>
            </template>
        </n-page-header>
        <n-data-table size="small" :bordered="false" :columns="columns" :data="store.items" :loading="isLoading"
            :scroll-x="500" />

        <AddEdit />
    </div>
</template>

<route lang="yaml">
meta:
    name: users
    title: users
    layout: auth
    authRequired: true
    breadcrumb:
    - authentication
    - users
</route>