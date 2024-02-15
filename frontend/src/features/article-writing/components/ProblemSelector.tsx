import { Box, Group, Text } from "@mantine/core";
import { PlatformSelector } from "@/features/article-writing/components/PlatformSelector";
import { ProblemAutocomplete } from "@/features/article-writing/components/ProblemAutocomplete";

export function ProblemSelector() {
  return (
    <Box py={10}>
      <Text fw="bold">Problem</Text>
      <Group gap={5} py={3} wrap="nowrap">
        <PlatformSelector />
        <Box className="flex-grow">
          <ProblemAutocomplete />
        </Box>
      </Group>
    </Box>
  );
}
