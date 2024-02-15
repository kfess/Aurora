import { Autocomplete } from "@mantine/core";
import { FaSearch } from "react-icons/fa";

interface Props {}

// データは autocomplete で取得する
export function ProblemInput() {
  return (
    <Autocomplete placeholder="Search Problems" leftSection={<FaSearch />} />
  );
}
