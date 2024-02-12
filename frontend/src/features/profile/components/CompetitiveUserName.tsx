import { Group, Text } from "@mantine/core";
import { Platform } from "@/types/platformTypes";

interface Props {
  platform: Platform;
  userName: string;
  rating?: number;
}

export function CompetitiveUserName({ platform, userName, rating }: Props) {
  return (
    <Group gap={5}>
      <Text fw="bold">{platform}: </Text>
      <Text fw="bold" c="red">
        {userName}
      </Text>
    </Group>
  );
}
