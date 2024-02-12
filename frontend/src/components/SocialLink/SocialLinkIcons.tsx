import { Group } from "@mantine/core";
import { SocialLinkIcon } from "./SocialLinkIcon";

interface Props {
  githubUserName?: string;
  xUserName?: string;
  facebookUserName?: string;
  discordUserName?: string;
  homePageUrl?: string;
}

export function SocialLinkIcons({
  githubUserName,
  xUserName,
  facebookUserName,
  discordUserName,
  homePageUrl,
}: Props) {
  return (
    <Group gap="xs" align="center" justify="center">
      {githubUserName && (
        <SocialLinkIcon type="GitHub" userName={githubUserName} />
      )}
      {xUserName && <SocialLinkIcon type="X" userName={xUserName} />}
      {facebookUserName && (
        <SocialLinkIcon type="Facebook" userName={facebookUserName} />
      )}
      {discordUserName && (
        <SocialLinkIcon type="Discord" userName={discordUserName} />
      )}
      <SocialLinkIcon type="RSS" userName="#" />
      {homePageUrl && <SocialLinkIcon type="HomePage" url={homePageUrl} />}
    </Group>
  );
}
