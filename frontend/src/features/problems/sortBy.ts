import { ProblemType } from "./types";

export const sortBy = (problems: ProblemType[], key: keyof ProblemType) => {
  switch (key) {
    case "difficulty":
      return problems.sort((a, b) => {
        if (!a.difficulty || !b.difficulty) return 0;
        return a.difficulty - b.difficulty;
      });
  }
};
