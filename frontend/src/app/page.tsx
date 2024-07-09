"use client";

import { DatePickerInput } from "@mantine/dates";
import { useState } from "react";

export default function Home() {
  const [value, setValue] = useState<Date | null>(null);

  return (
    <main>
      <DatePickerInput
        label="Date Picker"
        placeholder="a"
        value={value}
        onChange={(value) => setValue(value)}
      />
    </main>
  );
}
