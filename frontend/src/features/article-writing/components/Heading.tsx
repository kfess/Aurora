import Image from "next/image";
import { Stack, Text, Title as MantineTitle, Flex, Box } from "@mantine/core";
import { ArticleCategory } from "@/features/articles/types";

const categoryMessageMap: Record<ArticleCategory, string> = {
  Solution:
    "Solution は、競技プログラミングコンテストの問題についての解法をまとめたり、共有する場所です。",
  Algorithm:
    "Algorithm は、アルゴリズムなどに関連したトピックを共有する場所です。",
  Typical:
    "典型問題のパターンを一言で表現し、そのパターンへの解法・類題などをまとめた記事です。",
  Idea: "競技プログラミングに関連するトピックで、個人的な意見などについてまとめた記事には、Idea を選びます。",
};

interface Props {
  articleCategory: ArticleCategory;
}

export function Heading({ articleCategory }: Props) {
  return (
    <Flex gap={5} direction="row">
      <Box miw={50}>
        <Image
          src="/svg/light_bulb.svg"
          width={50}
          height={50}
          alt="solution"
        />
      </Box>
      <Stack gap={1}>
        <MantineTitle order={2} fw="bold">
          New {articleCategory} Article
        </MantineTitle>
        <Text size="sm" c="dimmed">
          {categoryMessageMap[articleCategory]}
        </Text>
      </Stack>
    </Flex>
  );
}
