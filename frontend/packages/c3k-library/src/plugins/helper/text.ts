export enum CaseType {
  UPPER = 'upper',
  LOWER = 'lower',
  SENTENCE = 'sentence',
  TITLE = 'title',
  WORD = 'word',
}

export const convertCase = (input: string, caseType: CaseType): string => {
  switch (caseType) {
    case CaseType.UPPER:
      return input.toUpperCase()
  case CaseType.LOWER:
      return input.toLowerCase()
  case CaseType.SENTENCE:
      return input.charAt(0).toUpperCase() + input.slice(1).toLowerCase()
  case CaseType.TITLE:
      return input.split(' ')
        .map(word => word.charAt(0).toUpperCase() + word.slice(1).toLowerCase())
        .join(' ')
  case CaseType.WORD:
      return input.split(' ')
        .map(word => word.toUpperCase())
        .join(' ')
  default:
      throw new Error(`Unsupported case type: ${caseType}`)
  }
}

export const truncateText = (text: string, maxLength = 80): string => {
  if (text === undefined)
    return text

  if (text.length <= maxLength)
    return text

  return `${text.slice(0, maxLength)}...`
}
