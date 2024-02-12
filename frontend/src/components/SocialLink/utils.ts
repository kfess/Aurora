import { SocialMediaType } from "./types";

export function generateSocialMediaLinkUrl(
  type: SocialMediaType,
  userName: string
) {
  switch (type) {
    case "GitHub":
      return `https://github.com/${userName}`;
    case "X":
      return `https://twitter.com/${userName}`;
    case "Facebook":
      return `https://www.facebook.com/${userName}`;
    case "Discord":
      return `https://discordapp.com/users/${userName}`;
    case "RSS":
      return ""; // TODO: RSS の URL を返す
    default:
      return "#";
  }
}
