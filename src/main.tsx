import React from "react";
import ReactDOM from "react-dom/client";
import { listen } from "@tauri-apps/api/event";
import App from "./App";
import { Layout } from "./components/Layout";
import { useTauriCommands } from "./hooks/useTauriCommands";

const refreshStorageKey = "leekFundGlobalRefreshMs";

const AppShell = () => {
  const [globalRefreshMs, setGlobalRefreshMs] = React.useState(10000);
  const { setRefreshInterval } = useTauriCommands();

  React.useEffect(() => {
    if (typeof window === "undefined") return;
    const stored = window.localStorage.getItem(refreshStorageKey);
    if (!stored) return;
    const parsed = Number(stored);
    if (!Number.isNaN(parsed) && parsed > 0) {
      setGlobalRefreshMs(parsed);
    }
  }, []);

  React.useEffect(() => {
    if (typeof window === "undefined") return;
    window.localStorage.setItem(refreshStorageKey, String(globalRefreshMs));
  }, [globalRefreshMs]);

  React.useEffect(() => {
    setRefreshInterval(globalRefreshMs).catch(() => null);
  }, [globalRefreshMs, setRefreshInterval]);

  React.useEffect(() => {
    const unlistenPromise = listen<number>(
      "refresh-interval-selected",
      (event) => {
        setGlobalRefreshMs(event.payload);
      }
    );
    return () => {
      void unlistenPromise.then((unlisten) => unlisten());
    };
  }, []);

  return (
    <Layout>
      <App globalRefreshMs={globalRefreshMs} />
    </Layout>
  );
};
import "./styles.css";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <AppShell />
  </React.StrictMode>
);
