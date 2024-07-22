"use client";

import { Text } from "@mantine/core";
import { DataTable, DataTableColumn } from "mantine-datatable";
import { useState, useEffect, useRef } from "react";

import { ProblemExternalLink } from "@/features/problems/components/ProblemExternalLink";
import { ProblemType } from "@/features/problems/types";
import { Category, Platform } from "@/features/types";

const PAGE_SIZES = [20, 50, 100];

interface Props {
  selectedPlatform: Platform;
  selectedCategory: Category<Platform>;
  problems: ProblemType<Platform>[];
}

export const ProblemTable = ({
  selectedPlatform,
  selectedCategory,
  problems,
}: Props) => {
  const totalProblems = problems?.length || 0;
  const [activePage, setActivePage] = useState(1);
  const [rowsPerPage, setRowsPerPage] = useState(PAGE_SIZES[1]);
  const records = problems.slice(
    (activePage - 1) * rowsPerPage,
    activePage * rowsPerPage,
  );

  useEffect(() => {
    setActivePage(1);
  }, [rowsPerPage, selectedPlatform, selectedCategory]);

  const viewpointRef = useRef<HTMLDivElement>(null);
  const scrollToTop = () => {
    viewpointRef!.current!.scrollIntoView({
      behavior: "smooth",
      block: "start",
    });
  };

  const columns: DataTableColumn<ProblemType<Platform>>[] = [
    { accessor: "platform", sortable: true },
    {
      accessor: "title",
      title: "Title",
      sortable: true,
      render: (record: ProblemType<Platform>) => {
        return <ProblemExternalLink title={record.title} url={record.url} />;
      },
    },
    {
      accessor: "difficulty",
      sortable: true,
      render: (record: ProblemType<Platform>) => {
        return (
          <Text size="sm" c={record.difficulty === null ? "dimmed" : undefined}>
            {record.difficulty ?? "-"}
          </Text>
        );
      },
    },
    {
      accessor: "raw_point",
      title: "Point",
      sortable: true,
      render: (record: ProblemType<Platform>) => {
        return (
          <Text size="sm" c={record.raw_point === null ? "dimmed" : undefined}>
            {record.raw_point ?? "-"}
          </Text>
        );
      },
    },
    { accessor: "contest ID", sortable: false },
  ];

  return (
    <div ref={viewpointRef}>
      <DataTable
        withTableBorder
        striped
        highlightOnHover
        columns={columns}
        records={records}
        minHeight={250}
        totalRecords={totalProblems}
        paginationActiveBackgroundColor="red"
        recordsPerPage={rowsPerPage}
        page={activePage}
        onPageChange={(value) => {
          setActivePage(value);
          scrollToTop();
        }}
        recordsPerPageOptions={PAGE_SIZES}
        onRecordsPerPageChange={setRowsPerPage}
      />
    </div>
  );
};
