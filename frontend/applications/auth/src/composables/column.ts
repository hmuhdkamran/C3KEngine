import type { DataTableColumns } from 'naive-ui';
import { h, resolveDirective } from 'vue';
import { NButton, NPopconfirm } from 'naive-ui';
import { withDirectives } from 'vue';

const vPermission = resolveDirective('permission')

export function createActionColumn<TRow>(
  editHandler: (row: TRow) => void,
  deleteHandler: (row: TRow) => void,
  t: (key: string) => string
): DataTableColumns<TRow>[number] {
  return {
    title: t('forms.action'),
    key: 'action',
    width: 80,
    render(row) {
      return h('div', {}, [
        // Edit Button
        h(NButton, {
          size: 'small',
          text: true,
          onClick: () => editHandler(row)
        }, {
          icon: () => h('i', { class: 'fa-solid fa-pen' }),
        }),
        h(
          NPopconfirm,
          {
            onPositiveClick: () => deleteHandler(row),
            onNegativeClick: () => { },
          },
          {
            trigger: () => {
              const deleteButton = h(
                NButton,
                {
                  text: true,
                  size: 'small',
                  type: 'error',
                  style: { marginLeft: '8px' },
                },
                {
                  icon: () => h('i', { class: 'fa-solid fa-trash-can' }),
                }
              );
              // Apply v-permission directive
              return withDirectives(deleteButton, [[vPermission, 'delete']]);
            },
            default: () => h('div', t('forms.deleteText')),
          }
        )
      ]);
    },
  };
}