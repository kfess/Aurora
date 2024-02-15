import { Badge } from "@mantine/core";
import { Platform } from "@/types/types";

interface Props {
  platform: Platform;
  userName: string;
  rating?: number;
}

export function CompetitiveUserName({ platform, userName, rating }: Props) {
  return (
    <Badge
      component="a"
      href="#"
      variant="dot"
      color="red"
      radius="sm"
      size="lg"
      style={{ textTransform: "none" }}
    >
      {`${platform}: ${userName}`}
    </Badge>
  );
}
