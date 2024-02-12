import { Box, Group, Title, Badge, Text, Stack, Flex } from "@mantine/core";

interface Props {
  isFirst?: boolean;
}

export function ArticleListCard({ isFirst = false }: Props) {
  return (
    <Box p={16} style={{ borderTop: isFirst ? "none" : "1px solid #eee" }}>
      <Flex justify="space-between" direction="row">
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
            <div>
              {Array.from({ length: 3 }).map((_, i) => (
                <Badge key={i} color="gray" radius="sm" variant="transparent">
                  # Union Find
                </Badge>
              ))}
            </div>
          </Group>
        </div>
        <div>a</div>
      </Flex>
    </Box>
  );
}
