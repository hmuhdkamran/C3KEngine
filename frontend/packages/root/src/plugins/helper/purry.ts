/* eslint-disable @typescript-eslint/no-explicit-any */
/**
 * Creates a function with `data-first` and `data-last` signatures.
 *
 * `purry` is a dynamic function and it's not type safe. It should be wrapped by a function that has proper typings.
 * Refer to the example below for correct usage.
 *
 * @param fn the function to purry.
 * @param args the arguments
 * @signature purry(fn, arguments);
 * @example
 *    function _findIndex<T>(array: T[], fn: (item: T) => boolean): number {
 *      for (let i = 0; i < array.length; i++) {
 *        if (fn(array[i])) {
 *          return i;
 *        }
 *      }
 *      return -1;
 *    }
 *
 *    // data-first
 *    function findIndex<T>(array: T[], fn: (item: T) => boolean): number;
 *
 *    // data-last
 *    function findIndex<T>(fn: (item: T) => boolean): (array: T[]) => number;
 *
 *    function findIndex() {
 *      return purry(_findIndex, arguments);
 *    }
 * @category Function
 */
export function purry<T extends (...args: any[]) => any>(
  fn: T,
  args: IArguments | readonly any[],
  lazy?: boolean
): ReturnType<T> | ((data: any) => ReturnType<T>) {
  const diff = fn.length - args.length;
  const arrayArgs = Array.from(args as any[]); // Ensure args is treated as an array
  
  if (diff === 0) {
    // If the number of arguments match, call the function with the provided arguments
    return fn(...arrayArgs);
  }

  if (diff === 1) {
    // If the function needs one more argument, return a new function that accepts `data`
    const ret = (data: any) => fn(data, ...arrayArgs);

    // Handle lazy execution if applicable
    if (lazy || (fn as any).lazy) {
      (ret as any).lazy = lazy || (fn as any).lazy;
      (ret as any).lazyArgs = args;
    }

    return ret;
  }

  throw new Error('Wrong number of arguments');
}
