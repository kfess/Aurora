import { Box, Divider, Text } from "@mantine/core";

interface Props {
  readonly items: { message: string }[];
}

export function List({ items }: Props) {
  return (
    <>
      {items.map((item) => (
        <Box component="div" key={item.message}>
          <Divider my="sm" />
          <Text size="sm" c="dimmed">
            {item.message}
          </Text>
        </Box>
      ))}
    </>
  );
}
