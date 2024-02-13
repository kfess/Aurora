import { Button, Container, Group, Tabs } from "@mantine/core";
import { HiOutlinePencilAlt } from "react-icons/hi";
import { OverlapUserInfo } from "@/features/profile/components/OverlapUserInfo";
import { ArticleList } from "@/features/articles/components/ArticleList";

export default function UserPage() {
  return (
    <>
      <OverlapUserInfo />
      <Container size="xl" px={{ sm: "md", md: "xl" }}>
        <Group justify="flex-end">
          <Button
            leftSection={<HiOutlinePencilAlt size="18" />}
            variant="filled"
            color="violet"
            mt="xs"
            radius="md"
            w={"auto"}
          >
            記事を投稿
          </Button>
        </Group>
      </Container>
      <Tabs defaultValue="Solution" color="violet" mt="xs">
        <Tabs.List>
          <Tabs.Tab value="Solution" fw="bold" px="md">
            Solution
          </Tabs.Tab>
          <Tabs.Tab value="Algorithm" fw="bold" px="xs">
            Algorithm
          </Tabs.Tab>
          <Tabs.Tab value="Typical" fw="bold" px="xs">
            Typical
          </Tabs.Tab>
          <Tabs.Tab value="Idea" fw="bold" px="xs">
            Idea
          </Tabs.Tab>
          <Tabs.Tab value="Comment" fw="bold" px="xs">
            Comment
          </Tabs.Tab>
        </Tabs.List>

        <Tabs.Panel value="Solution">
          <ArticleList />
        </Tabs.Panel>
        <Tabs.Panel value="Algorithm">Algorithm tab content</Tabs.Panel>
        <Tabs.Panel value="Typical">TypicalTechnique tab content</Tabs.Panel>
        <Tabs.Panel value="Idea">Idea tab content</Tabs.Panel>
        <Tabs.Panel value="Comment">Comment tab content</Tabs.Panel>
      </Tabs>
    </>
  );
}
