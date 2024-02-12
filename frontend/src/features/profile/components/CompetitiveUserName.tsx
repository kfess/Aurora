import { Box, Group, Text } from "@mantine/core";
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
      <Box
        component="a"
        href="#"
        target="_blank"
        rel="noopener noreferrer"
        style={{ textDecoration: "none" }}
      >
        <Text fw="bold" c="red">
          {userName}
        </Text>
      </Box>
    </Group>
  );
}
