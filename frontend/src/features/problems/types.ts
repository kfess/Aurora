import { Category, Platform } from "@/features/types";

export type ProblemType<P extends Platform> = {
  id: string;
  contestId: string;
  contestName: string;
  index: string;
  name: string;
  title: string;
  platform: P;
  rawPoint: number | null;
  difficulty: number | null;
  category: Category<P>;
  isExperimental: boolean | null;
  tags: string[];
  url: string;
  solverCount: number | null;
  submissions: number | null;
  successRate: number | null;
};
