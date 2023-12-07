import { NextUIProvider } from "@nextui-org/react";
import Layout from "./page/layout";

export default function App() {
  return (
    <NextUIProvider>
      <Layout />
    </NextUIProvider>
  );
}
