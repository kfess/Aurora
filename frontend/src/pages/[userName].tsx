import { Button, Container, Group, Tabs } from "@mantine/core";
import { HiOutlinePencilAlt } from "react-icons/hi";
import { OverlapUserInfo } from "@/features/profile/components/OverlapUserInfo";

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
        <Tabs defaultValue="Solution" color="violet" mt="sm">
          <Tabs.List>
            <Tabs.Tab value="Solution">Solution</Tabs.Tab>
            <Tabs.Tab value="Algorithm">Algorithm</Tabs.Tab>
            <Tabs.Tab value="Typical">Typical</Tabs.Tab>
            <Tabs.Tab value="Idea">Idea</Tabs.Tab>
          </Tabs.List>

          <Tabs.Panel value="Solution">Solution tab content</Tabs.Panel>
          <Tabs.Panel value="Algorithm">Algorithm tab content</Tabs.Panel>
          <Tabs.Panel value="Typical">TypicalTechnique tab content</Tabs.Panel>
          <Tabs.Panel value="Idea">Idea tab content</Tabs.Panel>
        </Tabs>
      </Container>
    </>
  );
}
