import { Button, Text } from "@mantine/core";
import { noop } from "@/utils/etc";

interface Props {
  followings: number;
}

export function Followings({ followings }: Props) {
  return (
    <Button
      variant="transparent"
      color="gray"
      size="compact-sm"
      onClick={noop}
      px={0}
    >
      {`${followings} Followings`}
    </Button>
  );
}
