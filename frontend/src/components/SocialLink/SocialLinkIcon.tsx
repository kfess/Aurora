import { useState } from "react";
import { FaGithub } from "react-icons/fa";
import { RiTwitterXFill } from "react-icons/ri";
import { FaFacebook } from "react-icons/fa";
import { FiLink } from "react-icons/fi";
import { FaDiscord } from "react-icons/fa";
import { FaRss } from "react-icons/fa";
import { SocialLinkProps } from "./types";
import { generateSocialMediaLinkUrl } from "./utils";

// Icon コンポーネントの Mapping
const IconMapping = {
  GitHub: FaGithub,
  X: RiTwitterXFill,
  Facebook: FaFacebook,
  HomePage: FiLink,
  Discord: FaDiscord,
  RSS: FaRss,
};

export function SocialLinkIcon(props: SocialLinkProps) {
  const { type, size = 20, color = "gray" } = props;

  // hover 時の色を管理するための state
  const [currentColor, setCurrentColor] = useState(color);

  const href =
    type === "HomePage"
      ? props.url
      : generateSocialMediaLinkUrl(type, props.userName);

  const IconComponent = IconMapping[type] || FiLink;

  return (
    <a
      href={href}
      target="_blank"
      rel="noopener noreferrer"
      onMouseEnter={() => setCurrentColor("black")}
      onMouseLeave={() => setCurrentColor(color)}
    >
      <IconComponent size={size} color={currentColor} />
    </a>
  );
}
