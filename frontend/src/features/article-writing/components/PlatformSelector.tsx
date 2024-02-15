import { Select } from "@mantine/core";
import { Platform } from "@/types/types";

interface Props {}

const platforms: Platform[] = [
  "Atcoder",
  "Codeforces",
  "AizuOnlineJudge",
  "yukicoder",
  "Library Checker",
] as const;

export function PlatformSelector({}: Props) {
  return (
    <Select
      checkIconPosition="right"
      placeholder="Select platform"
      data={platforms}
      defaultValue="Atcoder"
      clearable
    />
  );
}
