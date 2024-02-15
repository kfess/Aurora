export type ArticleCategory = "Solution" | "Algorithm" | "Typical" | "Idea";

type Article = {
  id: string;
  slug: string;
  title: string;
  postCategory: ArticleCategory;
  topics: string[];
  likedCount: number;
  createdAt: Date;
  updatedAt: Date;
  isOpen: boolean;
  isPrivate: boolean;
  commentedUserIds: string[];
  comments: string[];
  userId: string;
};
