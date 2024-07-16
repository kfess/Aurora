import { useQuery } from "@tanstack/react-query";
import axios from "axios";

import { ProblemType } from "@/features/problems/types";
import { Platform } from "@/features/types";

export const fetchProblems = async (
  platform: Platform,
): Promise<ProblemType[]> => {
  const response = await axios.get<ProblemType[]>(
    `http://localhost:8080/api/problems/${platform}?page_size=10000`,
  );

  return response.data;
};

export const useFetchProblems = (platform: Platform) => {
  const { data } = useQuery({
    queryKey: ["problems", platform],
    queryFn: () => fetchProblems(platform),
  });

  return { problems: data || [] };
};
