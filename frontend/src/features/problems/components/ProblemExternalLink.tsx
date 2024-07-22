import { Anchor, Flex } from "@mantine/core";

import { DifficultyColorCircle } from "./DifficultyColorCircle";

interface Props {
  title: string;
  url: string;
  difficulty?: number;
}

export const ProblemExternalLink = ({ title, url, difficulty }: Props) => {
  return (
    <Flex
      gap="xs"
      justify="flex-start"
      align="center"
      direction="row"
      wrap="wrap"
    >
      <DifficultyColorCircle difficulty={difficulty} />
      <Anchor href={url} target="_blank" rel="noopener noreferrer" size="sm">
        {title}
      </Anchor>
    </Flex>
  );
};
