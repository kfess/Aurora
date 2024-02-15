import { Box, Text, Input } from "@mantine/core";

export function Title() {
  return (
    <Box py={10}>
      <Text fw="bold">Title</Text>
      <Input placeholder="Title" py={3} />
    </Box>
  );
}
