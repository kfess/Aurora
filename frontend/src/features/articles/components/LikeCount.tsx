import { Group, Text } from "@mantine/core";
import { FaRegHeart } from "react-icons/fa";

interface Props {
  count: number;
}

export function LikeCount({ count = 0 }: Props) {
  return (
    <Group gap={5} c="gray">
      <FaRegHeart size="16" />
      <Text fw="bold" size="sm">
        {count}
      </Text>
    </Group>
  );
}
