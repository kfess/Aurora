import { Stack } from "@mantine/core";
import { ArticleListCard } from "./ArticleListCard";

interface Props {}

export function ArticleList({}: Props) {
  return (
    <Stack
      my={20}
      gap={0}
      style={{ border: "1px solid #eee", borderRadius: "4px" }}
    >
      {Array.from({ length: 10 }).map((_, i) => (
        <ArticleListCard key={i} isFirst={i === 0} />
      ))}
    </Stack>
  );
}
