import { Container, Box, Title, Text, Flex } from "@mantine/core";
import { IoIosInformationCircleOutline } from "react-icons/io";
import { CategoryCards } from "@/features/post-category/components/CategoryCards";

export default function PostCategoryPage() {
  return (
    <Container size="xl" px={{ sm: "md", md: "xl" }}>
      <Box component="header" my="xl">
        <Title order={2} mt="md" mb="md">
          投稿カテゴリー
        </Title>
        <Text size="sm">
          記事を投稿するときに、Solution (解法記事)・Algorithm
          (アルゴリズム記事)・Idea (アイデア記事)・ Typical Pattern
          (典型テクニック記事) のいずれかのカテゴリーを選択してください。
        </Text>
      </Box>
      <CategoryCards />
      <Flex my="lg" c="gray" gap={5}>
        <IoIosInformationCircleOutline size={20} />
        <Text size="sm">判断が難しい場合は、どれを選んでも構いません。</Text>
      </Flex>
    </Container>
  );
}
