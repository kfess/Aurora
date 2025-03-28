import { useQuery, useQueryClient } from "@tanstack/react-query";
import axios from "axios";
import { useCallback } from "react";

import { ProblemType } from "@/features/problems/types";
import { Platform, platformDetails } from "@/features/types";
import { convertToCamelCase } from "@/utils/case";

export const fetchProblems = async (
  platform: Platform,
): Promise<ProblemType<Platform>[]> => {
  const response = await axios.get(
    `http://localhost:8080/api/problems/${platformDetails[platform].abbr}?page_size=10000`,
  );

  const problems = convertToCamelCase(response.data);
  return problems;
};

export const useFetchProblems = (platform: Platform) => {
  const { data } = useQuery({
    queryKey: ["problems", platform],
    queryFn: () => fetchProblems(platform),
    staleTime: 24 * 60 * 60 * 1000,
    retry: 3,
  });

  return { problems: data || [] };
};

export const usePrefetchProblems = () => {
  const queryClient = useQueryClient();
  const prefetchProblems = useCallback(
    (platform: Platform) => {
      queryClient.prefetchQuery({
        queryKey: ["problems", platform],
        queryFn: () => fetchProblems(platform),
        staleTime: 24 * 60 * 60 * 1000,
        retry: 3,
      });
    },
    [queryClient],
  );

  return { prefetchProblems };
};
