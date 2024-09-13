import { isToday } from './index'

export const round = (num: number, decimalPlaces = 0) => {
  const p = 10 ** decimalPlaces
  const roundedNumber = Math.round(num * p) / p

  // Add commas as thousands separators
  return roundedNumber.toLocaleString()
}

export const avatarText = (value: string) => {
  if (!value)
    return ''
  const nameArray = value.split(' ')

  return nameArray.map(word => word.charAt(0).toUpperCase()).join('')
}

export const nFormatter = (num: number, digits = 0) => {
  const lookup = [
    { value: 1, symbol: '' },
    { value: 1e3, symbol: 'k' },
    { value: 1e6, symbol: 'M' },
    { value: 1e9, symbol: 'G' },
    { value: 1e12, symbol: 'T' },
    { value: 1e15, symbol: 'P' },
    { value: 1e18, symbol: 'E' },
  ]

  const rx = /\.0+$|(\.[0-9]*[1-9])0+$/

  const item = lookup.slice().reverse().find(item => {
    return num >= item.value
  })

  return item ? (num / item.value).toFixed(digits).replace(rx, '$1') + item.symbol : '0'
}

// TODO: Try to implement this: https://twitter.com/fireship_dev/status/1565424801216311297
export const kFormatter = (num: number) => {
  const regex = /\B(?=(\d{3})+(?!\d))/g

  return Math.abs(num) > 9999 ? `${Math.sign(num) * +((Math.abs(num) / 1000).toFixed(1))}k` : Math.abs(num).toFixed(0).replace(regex, ',')
}

export const formatNumberToRupees = (number: number): string => {
  // Handle non-numeric input gracefully
  if (isNaN(number)) {
    return "Invalid number";
  }

  const parts = number.toFixed(2).split(".");

  // Reverse the integer part
  const reversedInteger = parts[0].split("").reverse().join("");

  // Insert commas every 3 digits
  const formattedInteger = reversedInteger.replace(/(\d{3})(?!$)/g, "$1,");

  // Reverse back and combine with decimal part
  const formattedNumber = formattedInteger.split("").reverse().join("") + "." + parts[1];

  // Prefix with "Rs."
  return "Rs. " + formattedNumber;
}

/**
 * Format and return date in Humanize format
 * Intl docs: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/format
 * Intl Constructor: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat
 * @param {String} value date to format
 * @param {Intl.DateTimeFormatOptions} formatting Intl object to format with
 */
export const formatDate = (value: string, formatting: Intl.DateTimeFormatOptions = { month: 'short', day: 'numeric', year: 'numeric' }) => {
  if (!value)
    return value

  return new Intl.DateTimeFormat('en-US', formatting).format(new Date(value))
}

export const formatLongDate = (dateString: string): string => {
  const options: Intl.DateTimeFormatOptions = { weekday: 'long', year: 'numeric', month: 'long', day: 'numeric' }

  // const formattedDate = new Date(dateString).toLocaleDateString(undefined, options);
  return new Intl.DateTimeFormat('en-US', options).format(new Date(dateString))
}

/**
 * Return short human friendly month representation of date
 * Can also convert date to only time if date is of today (Better UX)
 * @param {String} value date to format
 * @param {Boolean} toTimeForCurrentDay Shall convert to time if day is today/current
 */
export const formatDateToMonthShort = (value: string, toTimeForCurrentDay = true) => {
  const date = new Date(value)
  let formatting: Record<string, string> = { month: 'short', day: 'numeric' }

  if (toTimeForCurrentDay && isToday(date))
    formatting = { hour: 'numeric', minute: 'numeric' }

  return new Intl.DateTimeFormat('en-US', formatting).format(new Date(value))
}

export const prefixWithPlus = (value: number) => value > 0 ? `+${value}` : value
