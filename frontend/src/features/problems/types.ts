import { Platform } from "@/features/types";

export type ProblemType = {
  id: string;
  contestId: string;
  index: string;
  name: string;
  title: string;
  platform: Platform;
  raw_point: number | null;
  difficulty: number | null;
  isExperimental: boolean | null;
  tags: string[];
  url: string;
  solverCount: number | null;
  submissions: number | null;
  successRate: number | null;
};
