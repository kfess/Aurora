import "@/styles/globals.css";
import { Badge, MantineProvider, createTheme } from "@mantine/core";
import type { AppProps } from "next/app";
import { Layout } from "@/components/Layout";

const theme = createTheme({
  fontFamily: "inherit",
  breakpoints: {
    xs: "40em", // for mobile size
    sm: "48em",
    md: "64em",
    lg: "80em",
    xl: "96em",
  },
  components: {
    Badge: Badge.extend({
      styles: { root: { textTransform: "none" } },
    }),
  },
});

export default function App({ Component, pageProps }: AppProps) {
  return (
    <MantineProvider theme={theme}>
      <Layout>
        <Component {...pageProps} />
      </Layout>
    </MantineProvider>
  );
}
