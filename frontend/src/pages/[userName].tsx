import { Button, Container, Group } from "@mantine/core";
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
      </Container>
    </>
  );
}
