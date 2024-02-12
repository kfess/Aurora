import {
  Container,
  Avatar,
  Text,
  Group,
  Button,
  Box,
  Title,
  Stack,
} from "@mantine/core";
import { MdOutlineDescription } from "react-icons/md";
import { FaRegCalendarAlt } from "react-icons/fa";
import { SocialLinkIcons } from "@/components/SocialLink/SocialLinkIcons";
import { Likes } from "./Likes";
import { Followings } from "./Followings";
import { CompetitiveUserNames } from "./CompetitiveUserNames";

export function OverlapUserInfo() {
  return (
    <>
      <Box h={100} bg="gray.1" />
      <Container size="xl" px={{ sm: "md", md: "xl" }}>
        <Group justify="space-between">
          <Avatar
            src={null}
            size={100}
            mt={-50}
            style={{ border: "4px solid white" }}
          />
          <Button variant="default" size="xs">
            プロフィールを編集
          </Button>
        </Group>
        <Stack gap="xs">
          <Group justify="space-between">
            <Title order={2}>kfess</Title>
            <SocialLinkIcons
              githubUserName="kfess"
              xUserName="kfess"
              facebookUserName="kfess"
              discordUserName="kfess"
              homePageUrl="https://kfess.com"
            />
          </Group>
          <Group gap={5} c="dark.3">
            <MdOutlineDescription />
            <Text>Software Engineer in Japan</Text>
          </Group>
          <Group gap={5} c="dark.3">
            <FaRegCalendarAlt />
            <Text>2024 年 1 月から利用しています。</Text>
          </Group>
          <CompetitiveUserNames
            atcoderUserName="mugilily"
            codeforcesUserName="applemelon"
            aizuOnlineJudgeUserName="#"
            yukicoderUserName="#"
          />
          <Group gap="xs">
            <Likes likes={1} />
            <Followings followings={1} />
          </Group>
        </Stack>
      </Container>
    </>
  );
}
