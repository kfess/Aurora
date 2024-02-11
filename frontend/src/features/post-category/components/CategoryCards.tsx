import Image from "next/image";
import { Grid } from "@mantine/core";
import { CategoryCard, PostCategory } from "./CategoryCard";

const postCategories: PostCategory[] = [
  {
    icon: (
      <Image src="/svg/light_bulb.svg" width={50} height={50} alt="solution" />
    ),
    title: "Solution",
    description: "コンテストの問題への解説記事には、Solution を選びます。",
    examples: [
      { message: "(例) ABC338 C 問題の解説" },
      { message: "(例) ABC331 D 問題 - O(n) 解法 " },
      { message: "(例) ABC120 D 問題 行列累乗の解法" },
    ],
  },
  {
    icon: (
      <Image src="/svg/algorithm.svg" width={50} height={50} alt="algorithm" />
    ),
    title: "Algorithm",
    description: "アルゴリズム等に関連したトピックには、Algorithm を選びます。",
    examples: [
      { message: "(例) DFS の動作解説" },
      { message: "(例) 重みつき UnionFind の実装" },
      { message: "(例) 逆順から見る問題まとめ" },
    ],
  },
  {
    icon: (
      <Image
        src="/svg/typical.svg"
        width={50}
        height={50}
        alt="typical pattern"
      />
    ),
    title: "Typical Pattern",
    description:
      "典型問題のパターンを一言で表現し、そのパターンへの解法・類題などをまとめた記事を書くには、Typical Pattern を選びます。",
    examples: [
      { message: "(例) ABC 338 まとめ" },
      { message: "(例) 不明アルゴリズムまとめ" },
      { message: "(例) ヒューリスティックコンテスト途中経過" },
    ],
  },
  {
    icon: <Image src="/svg/idea.svg" width={50} height={50} alt="idea" />,
    title: "Idea",
    description:
      "競技プログラミングに関連するトピックで、個人的な意見などについてまとめた記事には、Idea を選びます。",
    examples: [
      { message: "(例) 3 年かけてやっと青色コーダーになった" },
      { message: "(例) 典型問題の変遷について" },
      { message: "(例) ヒューリスティックコンテストの勧め" },
    ],
  },
];

export function CategoryCards() {
  return (
    <Grid>
      {postCategories.map((category, index) => (
        <Grid.Col span={{ base: 12, md: 6 }} key={index}>
          <CategoryCard
            icon={category.icon}
            title={category.title}
            description={category.description}
            examples={category.examples}
          />
        </Grid.Col>
      ))}
    </Grid>
  );
}
