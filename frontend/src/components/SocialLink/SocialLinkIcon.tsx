import { Box } from "@mantine/core";
import { FaGithub } from "react-icons/fa";
import { RiTwitterXFill } from "react-icons/ri";
import { FaFacebook } from "react-icons/fa";
import { FiLink } from "react-icons/fi";
import { FaDiscord } from "react-icons/fa";
import { FaRss } from "react-icons/fa";
import { SocialLinkProps } from "./types";
import { generateSocialMediaLinkUrl } from "./utils";

export function SocialLinkIcon(props: SocialLinkProps) {
  const { type, size = 24, color = "gray" } = props;

  const IconComponent = ({ size, color }: { size: number; color: string }) => {
    switch (type) {
      case "GitHub":
        return <FaGithub size={size} color={color} />;
      case "X":
        return <RiTwitterXFill size={size} color={color} />;
      case "Facebook":
        return <FaFacebook size={size} color={color} />;
      case "HomePage":
        return <FiLink size={size} color={color} />;
      case "Discord":
        return <FaDiscord size={size} color={color} />;
      case "RSS":
        return <FaRss size={size} color={color} />;
      default:
        return <FiLink size={size} color={color} />;
    }
  };

  if (type === "HomePage") {
    return (
      <Box
        component="a"
        href={props.url}
        target="_blank"
        rel="noopener noreferrer"
      >
        <IconComponent size={size} color={color} />
      </Box>
    );
  }

  const url = generateSocialMediaLinkUrl(type, props.userName);
  return (
    <Box component="a" href={url} target="_blank" rel="noopener noreferrer">
      <IconComponent size={size} color={color} />
    </Box>
  );
}
