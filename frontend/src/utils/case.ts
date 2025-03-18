type SnakeToCamelCase<S extends string> = S extends `${infer T}_${infer U}`
  ? `${T}${Capitalize<SnakeToCamelCase<U>>}`
  : S;

type ConvertKeysToCamelCase<T> = T extends (infer U)[]
  ? ConvertKeysToCamelCase<U>[]
  : T extends object
    ? {
        [K in keyof T as SnakeToCamelCase<
          Extract<K, string>
        >]: ConvertKeysToCamelCase<T[K]>;
      }
    : T;

const toCamelCase = (s: string): string => {
  return s.replace(/_([a-z])/g, (_, c) => (c ? c.toUpperCase() : ""));
};

export function convertToCamelCase<T>(data: T): ConvertKeysToCamelCase<T> {
  if (Array.isArray(data)) {
    return data.map((item) =>
      convertToCamelCase(item),
    ) as ConvertKeysToCamelCase<T>;
  } else if (typeof data === "object" && data !== null) {
    return Object.keys(data).reduce((result, key) => {
      const newKey = toCamelCase(key);
      result[newKey] = convertToCamelCase((data as any)[key]); // eslint-disable-line @typescript-eslint/no-explicit-any
      return result;
    }, {} as any); // eslint-disable-line @typescript-eslint/no-explicit-any
  }
  return data as ConvertKeysToCamelCase<T>;
}
