"use client";

import { Container, Table, Pagination, Select } from "@mantine/core";
import { useState } from "react";

import { useFetchProblems } from "@/features/problems/hooks/useFetchProblems";

const rowsPerPageOptions: { value: number; label: string }[] = [
  { value: 20, label: "20 / Page" },
  { value: 50, label: "50 / Page" },
  { value: 100, label: "100 / Page" },
];

export const ProblemTable = () => {
  const { problems } = useFetchProblems("codeforces");

  const [activePage, setActivePage] = useState(0);
  const [rowsPerPage, setRowsPerPage] = useState(rowsPerPageOptions[1].value);
  const filteredProblems = problems.slice(
    activePage * rowsPerPage,
    (activePage + 1) * rowsPerPage,
  );

  const totalProblems = problems?.length || 0;

  return (
    <Container>
      <Pagination
        total={totalProblems / rowsPerPage}
        value={activePage}
        onChange={setActivePage}
        siblings={2}
      />
      <Table stickyHeader withTableBorder highlightOnHover>
        <Table.Thead>
          <Table.Tr>
            <Table.Th>Title</Table.Th>
            <Table.Th>Difficulty</Table.Th>
            <Table.Th>Platform</Table.Th>
            <Table.Th>Contest ID</Table.Th>
          </Table.Tr>
        </Table.Thead>
        <Table.Tbody>
          {filteredProblems?.map((p) => {
            return (
              <Table.Tr key={p.id}>
                <Table.Td>{p.title}</Table.Td>
                <Table.Td>{p.difficulty}</Table.Td>
                <Table.Td>{p.platform}</Table.Td>
                <Table.Td>{p.contestId}</Table.Td>
              </Table.Tr>
            );
          })}
        </Table.Tbody>
      </Table>
      <div>
        <Select
          data={rowsPerPageOptions.map((option) => ({
            value: option.value.toString(),
            label: option.label,
          }))}
          value={rowsPerPage.toString()}
          onChange={(value) => {
            const rowsPerPage = Number(value);
            if (rowsPerPage * activePage >= totalProblems) {
              setActivePage(Math.floor(totalProblems / rowsPerPage));
            }
            setRowsPerPage(rowsPerPage);
          }}
          checkIconPosition="right"
        />
      </div>
    </Container>
  );
};
