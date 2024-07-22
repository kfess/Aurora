"use client";

import { SegmentedControl } from "@mantine/core";

import { Category, Platform, categories } from "@/features/types";

interface Props {
  selectedPlatform: Platform;
  selectedCategory: Category<Platform>;
  setSelectedCategory: (category: Category<Platform>) => void;
}

export const CategoryFilter = ({
  selectedPlatform,
  selectedCategory,
  setSelectedCategory,
}: Props) => {
  const categoryList = categories[selectedPlatform];

  return (
    <>
      <SegmentedControl
        value={selectedCategory}
        onChange={(value: string) => {
          setSelectedCategory(value as Category<Platform>);
        }}
        data={categoryList.map((c) => ({ value: c, label: c }))}
        withItemsBorders={false}
      />
    </>
  );
};
