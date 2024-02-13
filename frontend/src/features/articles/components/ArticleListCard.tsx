import { Group, Title, Badge, Text, Flex, Avatar } from "@mantine/core";
import { LikeCount } from "./LikeCount";
import { CommentCount } from "./CommentCount";

interface Props {
  isFirst?: boolean;
  isMine?: boolean;
}

export function ArticleListCard({ isMine = false, isFirst = false }: Props) {
  return (
    <Flex gap={10} p={16} className={isFirst ? "border-t-0" : "border-t"}>
      {!isMine && (
        <Avatar size={40} radius="xl" src="https://i.pravatar.cc/300" />
      )}
      <Flex
        justify="space-between"
        flex={1}
        direction={{ base: "column", sm: "row" }}
        gap={10}
      >
        <div>
          <Group gap={10}>
            <Title mb={4} className="text-base sm:text-lg ">
              データ構造をマージする一般的なテク
            </Title>
          </Group>
          <Group gap={10} align="center">
            <Badge color="violet" radius="sm">
              Open
            </Badge>
            {!isMine && <Text size="xs">kfess</Text>}
            <Text size="xs">1 日前に追加</Text>
            <Group gap={10}>
              {Array.from({ length: 3 }).map((_, i) => (
                <Badge key={i} color="violet" radius="sm" variant="light">
                  # Union Find
                </Badge>
              ))}
            </Group>
          </Group>
        </div>
        <Group gap={12}>
          <LikeCount count={10} />
          <CommentCount count={10} />
        </Group>
      </Flex>
    </Flex>
  );
}
