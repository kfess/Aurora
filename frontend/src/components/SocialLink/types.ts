export type SocialMediaType = "GitHub" | "X" | "Facebook" | "Discord" | "RSS";
export type GeneralLinkType = "HomePage";

export type SocialLinkProps =
  | { type: SocialMediaType; userName: string; size: number; color: string }
  | { type: GeneralLinkType; url: string; size: number; color: string };
