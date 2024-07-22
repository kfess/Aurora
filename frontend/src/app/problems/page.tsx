import {
  dehydrate,
  HydrationBoundary,
  QueryClient,
} from "@tanstack/react-query";

import { ProblemExplorer } from "@/features/problems/components/ProblemExplorer";
import { fetchProblems } from "@/features/problems/hooks/useFetchProblems";

export default async function ProblemsPage() {
  const queryClient = new QueryClient();
  await queryClient.prefetchQuery({
    queryKey: ["problems", "Atcoder"],
    queryFn: () => fetchProblems("Atcoder"),
  });

  return (
    <HydrationBoundary state={dehydrate(queryClient)}>
      <ProblemExplorer />
    </HydrationBoundary>
  );
}
