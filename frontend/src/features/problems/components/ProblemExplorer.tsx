"use client";

import { Container, Flex } from "@mantine/core";
import { useMemo, useState } from "react";

import { PlatformSegmentedControl } from "./PlatformSegmentedControl";

import { CategoryFilter } from "@/features/problems/components/CategoryFilter";
import { ProblemTable } from "@/features/problems/components/ProblemTable";
import { useFetchProblems } from "@/features/problems/hooks/useFetchProblems";
import { PlatformCategoryMap, Platform } from "@/features/types";

interface Props {}

export const ProblemExplorer = ({}: Props) => {
  const [selectedPlatform, setSelectedPlatform] = useState<Platform>("Atcoder");
  const [selectedCategory, setSelectedCategory] =
    useState<PlatformCategoryMap[typeof selectedPlatform]>("ABC");

  const { problems } = useFetchProblems(selectedPlatform);

  const problemsByCategory = useMemo(() => {
    return problems.filter((p) => p.category === selectedCategory);
  }, [problems, selectedCategory]);

  return (
    <Container>
      <Flex
        my="sm"
        gap="xs"
        justify="flex-start"
        align="flex-start"
        direction="column"
        wrap="wrap"
      >
        <PlatformSegmentedControl
          selectedPlatform={selectedPlatform}
          setSelectedPlatform={setSelectedPlatform}
          setSelectedCategory={setSelectedCategory}
        />
        <CategoryFilter
          // It's so important to specify key prop for the component
          // to re-render when the selectedPlatform changes.
          key={selectedPlatform}
          selectedPlatform={selectedPlatform}
          selectedCategory={selectedCategory}
          setSelectedCategory={setSelectedCategory}
        />
      </Flex>
      <ProblemTable
        selectedPlatform={selectedPlatform}
        selectedCategory={selectedCategory}
        problems={problemsByCategory}
      />
    </Container>
  );
};
