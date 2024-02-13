import { Container, Stack } from "@mantine/core";
import { ArticleListCard } from "./ArticleListCard";
import { useMediaQuery } from "@mantine/hooks";

interface Props {}

export function ArticleList({}: Props) {
  const isMobile = useMediaQuery("(max-width: 640px)");

  return (
    <Container size="xl" px={isMobile ? 0 : "md"}>
      <Stack
        my={isMobile ? 10 : 20}
        gap={0}
        className={isMobile ? "" : "border  border-gray-200 rounded"}
      >
        {Array.from({ length: 10 }).map((_, i) => (
          <ArticleListCard key={i} isFirst={i === 0} />
        ))}
      </Stack>
    </Container>
  );
}
