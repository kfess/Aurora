export type Platform =
  | "Atcoder"
  | "Codeforces"
  | "AizuOnlineJudge"
  | "yukicoder"
  | "Library Checker";

export type PlatformUserName = `${Uncapitalize<Platform>}UserName`;
