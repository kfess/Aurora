import { Button } from "@mantine/core";
import { noop } from "@/utils/etc";

interface Props {
  likes: number;
}

export function Likes({ likes }: Props) {
  return (
    <Button
      variant="transparent"
      color="gray"
      size="compact-sm"
      onClick={noop}
      px={0}
    >
      {`${likes} Likes`}
    </Button>
  );
}
