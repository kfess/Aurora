import { Box, Group, Title, Badge, Text, Flex } from "@mantine/core";
import { LikeCount } from "./LikeCount";
import { CommentCount } from "./CommentCount";

interface Props {
  isFirst?: boolean;
}

export function ArticleListCard({ isFirst = false }: Props) {
  return (
    <Box p={16} style={{ borderTop: isFirst ? "none" : "1px solid #eee" }}>
      <Flex
        justify="space-between"
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
        <Group gap={10}>
          <LikeCount count={10} />
          <CommentCount count={10} />
        </Group>
      </Flex>
    </Box>
  );
}
