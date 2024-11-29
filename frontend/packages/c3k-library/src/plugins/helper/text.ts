export enum CaseType {
  UPPER = 'upper',
  LOWER = 'lower',
  SENTENCE = 'sentence',
  TITLE = 'title',
  WORD = 'word',
}

export class TextHelper {
  /**
   * Extracts the initials and assigns a text-based color.
   * @param text The input string.
   * @returns An object with initials and their color.
   */
  public static getInitialsWithColors(text: string): { initials: string; color: string } {
    if (!text) {
      throw new Error("Input text cannot be empty.");
    }

    // Extract the first two characters
    const initials = text
      .trim()
      .split(" ")
      .map((word) => word.charAt(0).toUpperCase())
      .join("")
      .substring(0, 2);

    // Generate a color based on the text
    const color = this.generateColorFromText(initials);

    return { initials, color };
  }

  /**
   * Converts the input string to a specified case format.
   * @param input The input string.
   * @param caseType The desired case format.
   * @returns The converted string.
   */
  public static convertCase(input: string, caseType: CaseType): string {
    switch (caseType) {
      case CaseType.UPPER:
        return input.toUpperCase();
      case CaseType.LOWER:
        return input.toLowerCase();
      case CaseType.SENTENCE:
        return input.charAt(0).toUpperCase() + input.slice(1).toLowerCase();
      case CaseType.TITLE:
        return input
          .split(" ")
          .map((word) => word.charAt(0).toUpperCase() + word.slice(1).toLowerCase())
          .join(" ");
      case CaseType.WORD:
        return input
          .split(" ")
          .map((word) => word.toUpperCase())
          .join(" ");
      default:
        throw new Error(`Unsupported case type: ${caseType}`);
    }
  }

  /**
   * Truncates a string to a specified maximum length.
   * @param text The input string.
   * @param maxLength The maximum length of the truncated string.
   * @returns The truncated string.
   */
  public static truncateText(text: string, maxLength = 80): string {
    if (text === undefined) return text;

    if (text.length <= maxLength) return text;

    return `${text.slice(0, maxLength)}...`;
  }

  /**
   * Generates a color hash based on input text.
   * @param text The input string.
   * @returns A hex color string.
   */
  public static generateColorFromText(text: string): string {
    let hash = 0;

    // Generate a simple hash from the text
    for (let i = 0; i < text.length; i++) {
      hash = text.charCodeAt(i) + ((hash << 5) - hash);
    }

    // Convert hash to RGB
    const r = (hash >> 16) & 0xff;
    const g = (hash >> 8) & 0xff;
    const b = hash & 0xff;

    // Return as a hex color
    return `#${((r << 16) | (g << 8) | b).toString(16).padStart(6, "0")}`;
  }
}
