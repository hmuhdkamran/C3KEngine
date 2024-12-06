export class LocalStorageHelper {
  // Set a value in localStorage
  static set<T>(key: string, value: T): void {
    const serializedValue = typeof value === "object" ? JSON.stringify(value) : String(value);
    localStorage.setItem(key, serializedValue);
  }

  // Get a value from localStorage
  static get<T>(key: string): T | null {
    const serializedValue = localStorage.getItem(key);
    
    if (serializedValue) {
      if(typeof serializedValue === 'object') {
        return JSON.parse(serializedValue) as T;
      } else {
        return serializedValue as T;
      }
    }

    return null;
  }

  // Remove a value from localStorage
  static del(key: string): void {
    localStorage.removeItem(key);
  }
}
