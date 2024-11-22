import { ref } from 'vue';
import { IUser } from '@/plugins';


type ValidationRule = (value: unknown) => string | boolean;

export function useValidation() {
  const validationErrors = ref<Record<string, string>>({});

  const validateField = (
    field: string,
    value: unknown,
    rules: ValidationRule[]
  ): boolean => {
    for (const rule of rules) {
      const result = rule(value);
      if (typeof result === 'string') {
        validationErrors.value[field] = result;
        return false;
      }
    }
    validationErrors.value[field] = '';
    return true;
  };

  const validateForm = (
    entity: IUser,
    fields: Array<{ field: keyof IUser; rules: Array<(value: unknown) => string | boolean> }>
  ): boolean => {
    const isValid = fields
      .map(({ field, rules }) => validateField(field as keyof IUser, entity[field], rules))
      .every(Boolean);
    return isValid;
  };

  return { validationErrors, validateField, validateForm };
}
