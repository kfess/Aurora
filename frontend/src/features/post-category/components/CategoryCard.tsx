import { Card, Group, Text, Button, Title, Flex } from "@mantine/core";
import { List } from "@/features/common/components/List";

export interface PostCategory {
  icon: React.ReactNode;
  title: string;
  description: string;
  examples: { message: string }[];
}

export function CategoryCard({
  icon,
  title,
  description,
  examples,
}: PostCategory) {
  return (
    <Card shadow="sm" padding="lg" radius="md" withBorder>
      <Group mb="xs">
        {icon}
        <Title order={2}>{title}</Title>
      </Group>
      <Text size="sm">{description}</Text>
      <List items={examples} />
      <Flex justify="flex-end">
        <Button variant="filled" color="violet" mt="lg" radius="md" w={"auto"}>
          記事を投稿
        </Button>
      </Flex>
    </Card>
  );
}
