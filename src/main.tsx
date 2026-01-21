import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { Layout } from "./components/Layout";

const refreshStorageKey = "leekFundGlobalRefreshMs";

const AppShell = () => {
  const [globalRefreshMs, setGlobalRefreshMs] = React.useState(10000);

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

  return (
    <Layout
      headerRight={
        <div className="global-settings">
          <span>全局刷新</span>
          <select
            value={globalRefreshMs}
            onChange={(e) => setGlobalRefreshMs(Number(e.target.value))}
          >
            <option value={10000}>10s</option>
            <option value={30000}>30s</option>
            <option value={60000}>60s</option>
            <option value={120000}>120s</option>
          </select>
        </div>
      }
    >
      <App
        globalRefreshMs={globalRefreshMs}
        onChangeGlobalRefreshMs={setGlobalRefreshMs}
      />
    </Layout>
  );
};
import "./styles.css";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <AppShell />
  </React.StrictMode>
);
