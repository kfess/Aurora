import { Group } from "@mantine/core";
import { CompetitiveUserName } from "./CompetitiveUserName";
import { PlatformUserName } from "@/types/platformTypes";

type Props = {
  [K in PlatformUserName]?: string;
};

export function CompetitiveUserNames({
  atcoderUserName,
  codeforcesUserName,
  aizuOnlineJudgeUserName,
  yukicoderUserName,
}: Props) {
  return (
    <Group gap={5}>
      {atcoderUserName && (
        <CompetitiveUserName
          platform="Atcoder"
          userName={atcoderUserName}
          rating={1000}
        />
      )}
      {codeforcesUserName && (
        <CompetitiveUserName
          platform="Codeforces"
          userName={codeforcesUserName}
          rating={1000}
        />
      )}
      {aizuOnlineJudgeUserName && (
        <CompetitiveUserName
          platform="AizuOnlineJudge"
          userName={aizuOnlineJudgeUserName}
          rating={1000}
        />
      )}
      {yukicoderUserName && (
        <CompetitiveUserName
          platform="yukicoder"
          userName={yukicoderUserName}
          rating={1000}
        />
      )}
    </Group>
  );
}
