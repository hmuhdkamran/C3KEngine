/* eslint-disable @typescript-eslint/no-explicit-any */
export const exportToCsv = (filename: string, rows: object[], headers?: string[]): void => {
  if (!rows || !rows.length) {
    return
  }
  const separator: string = ','
  const keys: string[] = Object.keys(rows[0])
  let columHearders: string[]

  if (headers) {
    columHearders = headers
  } else {
    columHearders = keys
  }

  const csvContent =
    'sep=,\n' +
    columHearders.join(separator) +
    '\n' +
    rows
      .map((row) => {
        return keys
          .map((k) => {
            let cell =
              (row as { [key: string]: any })[k] === null ||
              (
                row as {
                  [key: string]: any
                }
              )[k] === undefined
                ? ''
                : (row as { [key: string]: any })[k]

            cell =
              cell instanceof Date ? cell.toLocaleString() : cell.toString().replace(/"/g, '""')

            if (cell.search(/("|,|\n)/g) >= 0) {
              cell = `"${cell}"`
            }
            return cell
          })
          .join(separator)
      })
      .join('\n')

  const blob = new Blob([csvContent], { type: 'text/csv;charset=utf-8;' })
  if ((navigator as any).msSaveBlob) {
    ;(navigator as any).msSaveBlob(blob, filename)
  } else {
    const link = document.createElement('a')
    if (link.download !== undefined) {
      // Browsers that support HTML5 download attribute
      const url = URL.createObjectURL(blob)
      link.setAttribute('href', url)
      link.setAttribute('download', filename)
      link.style.visibility = 'hidden'
      document.body.appendChild(link)
      link.click()
      document.body.removeChild(link)
    }
  }
}

export const fileToBase64 = (file: File) =>
  new Promise((resolve, reject) => {
    const reader = new FileReader()
    reader.readAsDataURL(file)
    reader.onload = () => resolve((reader.result as string).split(',')[1])
    reader.onerror = reject
  })

export const base64ToFile = (base64String: string, fileName: string): File => {
  base64String = base64String.substring(2).replace(`'`, '')
  const byteCharacters = atob(base64String)
  const byteNumbers = new Array(byteCharacters.length)
  for (let i = 0; i < byteCharacters.length; i++) {
    byteNumbers[i] = byteCharacters.charCodeAt(i)
  }
  const byteArray = new Uint8Array(byteNumbers)
  const blob = new Blob([byteArray], { type: 'application/octet-stream' })
  return new File([blob], fileName)
}
