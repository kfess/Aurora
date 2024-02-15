import { Container } from "@mantine/core";
import { Heading } from "@/features/article-writing/components/Heading";
import { ProblemSelector } from "@/features/article-writing/components/ProblemSelector";
import { Title } from "@/features/article-writing/components/Title";

export default function NewArticlePage() {
  return (
    <Container size="xl" px={{ sm: "md", md: "xl" }} pt={15}>
      <Heading articleCategory="Algorithm" />
      <ProblemSelector />
      <Title />
    </Container>
  );
}
