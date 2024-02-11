import { Html, Head, Main, NextScript } from "next/document";
import { ColorSchemeScript } from "@mantine/core";

export default function Document() {
  return (
    <Html lang="en">
      <Head>
        <meta charSet="UTF-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <ColorSchemeScript />
      </Head>
      <body>
        <Main />
        <NextScript />
      </body>
    </Html>
  );
}
