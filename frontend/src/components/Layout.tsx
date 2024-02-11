import React, { ReactElement } from "react";
import { Header } from "./Header";

type LayoutProps = Required<{
  readonly children: ReactElement;
}>;

export function Layout({ children }: LayoutProps) {
  return (
    <>
      <Header />
      {children}
    </>
  );
}
