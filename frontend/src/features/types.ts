// Platform
export const platformDetails = {
  Atcoder: { full: "Atcoder", abbr: "atcoder" },
  Codeforces: { full: "Codeforces", abbr: "codeforces" },
  Yukicoder: { full: "Yukicoder", abbr: "yukicoder" },
  "Aizu Online Judge": { full: "Aizu Online Judge", abbr: "aoj" },
  "Yosupo Online Judge": {
    full: "Yosupo Online Judge",
    abbr: "yosupo_online_judge",
  },
} as const;

export type Platform = keyof typeof platformDetails;

export const platforms = Object.keys(platformDetails) as Platform[];

// Contest Category
const atcoderCategories = [
  "ABC",
  "ARC",
  "AGC",
  "AHC",
  "JOI",
  "JAG",
  "ABC-Like",
  "ARC-Like",
  "AGC-Like",
  "Marathon",
  "Other Sponsored",
  "Other",
] as const;
type AtcoderCategory = (typeof atcoderCategories)[number];

export const codeforcesCategories = [
  "Div.1",
  "Div.2",
  "Div.3",
  "Div.4",
  "Div.1+Div.2",
  "Educational",
  "Global",
  "Kotlin",
  "ICPC",
  "Q#",
  "Other",
] as const;
type CodeforcesCategory = (typeof codeforcesCategories)[number];

export const yukicoderCategories = ["Normal", "Other"] as const;
type YukicoderCategory = (typeof yukicoderCategories)[number];

export const categories = {
  Atcoder: atcoderCategories,
  Codeforces: codeforcesCategories,
  Yukicoder: yukicoderCategories,
  "Aizu Online Judge": [],
  "Yosupo Online Judge": [],
} as const;

export type PlatformCategoryMap = {
  Atcoder: AtcoderCategory;
  Codeforces: CodeforcesCategory;
  Yukicoder: YukicoderCategory;
  "Aizu Online Judge": never;
  "Yosupo Online Judge": never;
};

export type Category<P extends Platform> = PlatformCategoryMap[P];
