import {
  Container,
  Card,
  Avatar,
  Text,
  Group,
  Button,
  Box,
  Title,
  Stack,
} from "@mantine/core";

export function OverlapUserInfo() {
  return (
    <>
      <Box h={100} bg="gray.1" />
      <Container size="xl" px={{ sm: "md", md: "xl" }}>
        <Group justify="space-between">
          <Avatar
            src={null}
            size={100}
            mt={-50}
            style={{ border: "4px solid white" }}
          />
          <Button variant="default" size="xs">
            プロフィールを編集
          </Button>
        </Group>
        <Stack gap="xs">
          <Title order={2}>kfess</Title>
          <Text>Software Engineer in Japan</Text>
        </Stack>
      </Container>
    </>
  );
}
