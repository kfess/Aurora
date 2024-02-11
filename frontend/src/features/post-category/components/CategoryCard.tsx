import { Card, Group, Text, Button, Title, Flex, Badge } from "@mantine/core";
import { HiOutlinePencilAlt } from "react-icons/hi";
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
        {(title === "Solution" || title === "Typical Pattern") && (
          <Badge variant="light" color="red">
            Help needed
          </Badge>
        )}
      </Group>
      <Text size="sm" my="xs">
        {description}
      </Text>
      <List items={examples} />
      <Flex justify="flex-end">
        <Button
          leftSection={<HiOutlinePencilAlt size="18" />}
          variant="filled"
          color="violet"
          mt="lg"
          radius="md"
          w={"auto"}
        >
          記事を投稿
        </Button>
      </Flex>
    </Card>
  );
}
