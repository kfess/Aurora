export const platforms = [
  "atcoder",
  "codeforces",
  "yukicoder",
  "aoj",
  "yoj",
] as const;

export type Platform = (typeof platforms)[number];
