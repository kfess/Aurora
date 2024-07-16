import {
  dehydrate,
  HydrationBoundary,
  QueryClient,
} from "@tanstack/react-query";

import { fetchProblems } from "@/features/problems/hooks/useFetchProblems";
import { ProblemTable } from "@/features/problems/ProblemTable";

export default async function ProblemsPage() {
  const queryClient = new QueryClient();

  await queryClient.prefetchQuery({
    queryKey: ["problems", "codeforces"],
    queryFn: () => fetchProblems("codeforces"),
  });

  return (
    <HydrationBoundary state={dehydrate(queryClient)}>
      <ProblemTable />
    </HydrationBoundary>
  );
}
