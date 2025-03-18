import { Anchor, Flex } from "@mantine/core";

interface Props {
  contestName: string;
  problemUrl: string;
}

export const ContestExternalLink = ({ contestName, problemUrl }: Props) => {
  return (
    <Flex
      gap="xs"
      justify="flex-start"
      align="center"
      direction="row"
      wrap="wrap"
    >
      {problemUrl}
      <Anchor href={""} target="_blank" rel="noopener noreferrer" size="sm">
        {contestName}
      </Anchor>
    </Flex>
  );
};
