"use client";

import { SegmentedControl } from "@mantine/core";
import { useEffect, useRef } from "react";

import { usePrefetchProblems } from "@/features/problems/hooks/useFetchProblems";
import { Category, Platform, platforms } from "@/features/types";

interface Props {
  selectedPlatform: Platform;
  setSelectedPlatform: (platform: Platform) => void;
  setSelectedCategory: <P extends Platform>(category: Category<P>) => void;
}

export const PlatformSegmentedControl = ({
  selectedPlatform,
  setSelectedPlatform,
  setSelectedCategory,
}: Props) => {
  const { prefetchProblems } = usePrefetchProblems();
  const isMounted = useRef(false);

  // ロード時に一気に prefetch
  useEffect(() => {
    if (!isMounted.current) {
      platforms.forEach((p) => {
        if (p !== selectedPlatform) {
          prefetchProblems(p);
        }
      });
    }
  }, [prefetchProblems, selectedPlatform]);

  return (
    <SegmentedControl
      value={selectedPlatform}
      onChange={(value: string) => {
        setSelectedPlatform(value as Platform);
        switch (value) {
          case "Atcoder":
            setSelectedCategory<"Atcoder">("ABC");
            break;
          case "Codeforces":
            setSelectedCategory<"Codeforces">("Div.1");
            break;
          case "Yukicoder":
            setSelectedCategory<"Yukicoder">("Normal");
            break;
          case "Aizu Online Judge":
            break;
          case "Yosupo Online Judge":
            break;
          default:
            break;
        }
      }}
      data={platforms.map((p) => ({ value: p, label: p }))}
      color="red"
      withItemsBorders={false}
    />
  );
};
