import { useSystemStore } from '../store/system-store'

export { mapKeys } from './mapKeys'
export { pick } from './pick'

export * from './text'
export * from './files'
export * from './local-storage';
export * from './token-helper';

export const shouldAllow = (find: string, operation: string) => {
  const store = useSystemStore()
  const role = store.user.roles

  const route = role.find((r: any) => r.RouteName === find)

  return !!(route && (route as any).Operation.includes(operation))
}

export const newGuid = () => {
  return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, c => {
    const r = (Math.random() * 16) | 0
    const v = c == 'x' ? r : (r & 0x3) | 0x8

    return v.toString(16)
  })
}

export function deepClone<T>(value: T): T {
  return JSON.parse(JSON.stringify(value))
}

// 👉 IsEmpty
export const isEmpty = (value: unknown): boolean => {
  if (value === null || value === undefined || value === '')
    return true

  return !!(Array.isArray(value) && value.length === 0)
}

// 👉 IsNullOrUndefined
export const isNullOrUndefined = (value: unknown): value is undefined | null => {
  return value === null || value === undefined
}

// 👉 IsEmptyArray
export const isEmptyArray = (arr: unknown): boolean => {
  return Array.isArray(arr) && arr.length === 0
}

// 👉 IsObject
export const isObject = (obj: unknown): obj is Record<string, unknown> =>
  obj !== null && !!obj && typeof obj === 'object' && !Array.isArray(obj)

export const isToday = (date: Date) => {
  const today = new Date()

  return (
    /* eslint-disable operator-linebreak */
    date.getDate() === today.getDate() &&
    date.getMonth() === today.getMonth() &&
    date.getFullYear() === today.getFullYear()
    /* eslint-enable */
  )
}

export const sortDataByColumn = <T>(data: T[], column: keyof T, ascending = true): T[] => {
  return data.slice().sort((a, b) => {
    const valueA = a[column];
    const valueB = b[column];

    if (typeof valueA === 'number' && typeof valueB === 'number') {
      return ascending ? valueA - valueB : valueB - valueA;
    } else if (typeof valueA === 'string' && typeof valueB === 'string') {
      return ascending
        ? valueA.localeCompare(valueB)
        : valueB.localeCompare(valueA);
    } else {
      throw new Error('Unsupported data type for sorting');
    }
  });
}

export const exportCsv = (data: Record<string, any>[]) => {
  if (data.length === 0) {
    console.warn('No data to export.');
    return;
  }
  const currentDate = new Date();
  const formattedDate = currentDate.toISOString().replace(/[:.]/g, '-');
  const fileName = `SK_${formattedDate}.csv`;


  const headers = Object.keys(data[0]).join(',');
  const rows = data.map(row => Object.values(row).join(',')).join('\n');
  const csvContent = `${headers}\n${rows}`;
  const blob = new Blob([csvContent], { type: 'text/csv' });
  const link = document.createElement('a');
  link.href = URL.createObjectURL(blob);
  link.download = fileName;
  link.click();
}