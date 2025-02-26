import { isEmpty, isEmptyArray, isNullOrUndefined } from './index'

export interface ValidationResult {
  valid: boolean
}

export const validate = (
  validationObj: Record<string, boolean>,
  exclude: string[],
): boolean => {
  return Object.keys(validationObj)
    .filter((key) => !exclude.includes(key))
    .every((key) => validationObj[key])
}

// 👉 Required Validator
export const requiredValidator = (value: unknown): string | true => {
  if (
    isNullOrUndefined(value) ||
    isEmptyArray(value) ||
    isEmpty(value) ||
    !String(value).trim().length
  ) {
    return 'This field is required'
  }
  return true
}

// 👉 Email Validator
export const emailValidator = (value: unknown): string | true => {
  if (isEmpty(value)) return "This field is required";

  const re =
    /^(([^<>()\[\]\\.,;:\s@"]+(\.[^<>()\[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/

  if (Array.isArray(value))
    return value.every((val) => re.test(String(val))) || 'The Email field must be a valid email'

  return re.test(String(value)) || 'The Email field must be a valid email'
}

// 👉 Password Validator
export const passwordValidator = (
  value: unknown,
  regex: RegExp | string = /(?=.*\d)(?=.*[a-z])(?=.*[A-Z])(?=.*[!@#$%&*()]).{8,}/,
): string | true => {
  if (isEmpty(value)) return "This field is required";

  if (typeof value !== 'string') return 'Invalid password type'

  let regeX = regex
  if (typeof regeX === 'string') regeX = new RegExp(regeX)

  const validPassword = regeX.test(value)

  return (
    // eslint-disable-next-line operator-linebreak
    validPassword ||
    'Field must contain at least one uppercase, lowercase, special character and digit with min 8 chars'
  )
}

// 👉 Confirm Password Validator
export const confirmedValidator = (value: string, target: string): string | true => {
  if (isEmpty(value)) return "This field is required";

  return value === target || 'The Confirm Password field confirmation does not match'
}

// 👉 Between Validator
export const betweenValidator = (value: unknown, min: number, max: number): string | true => {
  if (isEmpty(value)) return "This field is required";

  const valueAsNumber = Number(value)

  return (
    (Number(min) <= valueAsNumber && Number(max) >= valueAsNumber) ||
    `Enter number between ${min} and ${max}`
  )
}

// 👉 Integer Validator
export const integerValidator = (value: unknown): string | true => {
  if (isEmpty(value)) return "This field is required";

  if (Array.isArray(value))
    return value.every((val) => /^-?[0-9]+$/.test(String(val))) || 'This field must be an integer'

  return /^-?[0-9]+$/.test(String(value)) || 'This field must be an integer'
}

// 👉 Regex Validator
export const regexValidator = (value: unknown, regex: RegExp | string): string | boolean => {
  if (isEmpty(value)) return "This field is required";

  let regeX = regex
  if (typeof regeX === 'string') regeX = new RegExp(regeX)

  if (Array.isArray(value)) return value.every((val) => regexValidator(val, regeX))

  return regeX.test(String(value)) || 'The Regex field format is invalid'
}

// 👉 Alpha Validator
export const alphaValidator = (value: unknown): string | true => {
  if (isEmpty(value)) return "This field is required";

  return /^[A-Z]*$/i.test(String(value)) || 'The Alpha field may only contain alphabetic characters'
}

// 👉 URL Validator
export const urlValidator = (value: unknown): string | true => {
  if (isEmpty(value)) return "This field is required";

  const re = /^(http[s]?:\/\/){0,1}(www\.){0,1}[a-zA-Z0-9\.\-]+\.[a-zA-Z]{2,5}[\.]{0,1}/

  return re.test(String(value)) || 'URL is invalid'
}

// 👉 Length Validator
export const lengthValidator = (value: unknown, length: number): string | true => {
  if (isEmpty(value)) return "This field is required";

  return (
    String(value).length === length || `The Character field must be at least ${length} characters`
  )
}

// 👉 Alpha-dash Validator
export const alphaDashValidator = (value: unknown): string | true => {
  if (isEmpty(value)) return "This field is required";

  const valueAsString = String(value)

  return /^[0-9A-Z_-]*$/i.test(valueAsString) || 'All Character are not valid'
}

// 👉 Range Validator
// For numeric values: ensure the value is between min and max.
// For string values: ensure the string length is between min and max.
export const rangeValidator = (value: unknown, min: number, max: number): string | true => {
  if (isEmpty(value)) return "This field is required";

  if (typeof value === 'number') {
    return (value >= min && value <= max) || `Enter a number between ${min} and ${max}`
  }

  if (typeof value === 'string') {
    const len = value.length
    return (len >= min && len <= max) || `Enter text length between ${min} and ${max} characters`
  }

  // If the value is not a number or a string, try to convert it to a string.
  const str = String(value)
  const len = str.length
  return (len >= min && len <= max) || `Enter text length between ${min} and ${max} characters`
}
