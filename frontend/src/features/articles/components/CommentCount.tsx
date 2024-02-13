import { Group, Text } from "@mantine/core";
import { FaRegComment } from "react-icons/fa";

interface Props {
  count: number;
}

export function CommentCount({ count = 0 }: Props) {
  return (
    <Group gap={5} c="gray">
      <FaRegComment size="16" />
      <Text fw="bold" size="sm">
        {count}
      </Text>
    </Group>
  );
}
