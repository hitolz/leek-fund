import { useEffect, useState } from "react";
import { ListsPanel } from "./components/ListsPanel";
import { ListDetailView } from "./components/ListDetailView";
import { useTauriCommands } from "./hooks/useTauriCommands";
import { FundDetail, FundList, FundTrend, Holding } from "./types";
import { useToast } from "./components/ToastContext";
import { FundDetailPanel } from "./components/FundDetailPanel";
import "./App.css";

interface AppProps {
  globalRefreshMs: number;
}

type FundSortKey =
  | "holding_amount"
  | "daily_change_percent"
  | "daily_change_amount";

function App({ globalRefreshMs }: AppProps) {
  const showToast = useToast();
  const [lists, setLists] = useState<FundList[]>([]);
  const [selectedListId, setSelectedListId] = useState<number | null>(null);
  const [selectedFundCode, setSelectedFundCode] = useState<string | null>(null);
  const [listsError, setListsError] = useState<string | null>(null);
  const [fundDetail, setFundDetail] = useState<FundDetail | null>(null);
  const [fundTrend, setFundTrend] = useState<FundTrend | null>(null);
  const [fundDetailLoading, setFundDetailLoading] = useState(false);
  const [fundDetailError, setFundDetailError] = useState<string | null>(null);
  const [holding, setHolding] = useState<Holding | null>(null);
  const [holdingLoading, setHoldingLoading] = useState(false);
  const [holdingError, setHoldingError] = useState<string | null>(null);
  const [holdingVersion, setHoldingVersion] = useState(0);
  const [leftCollapsed, setLeftCollapsed] = useState(false);
  const [sortKey, setSortKey] =
    useState<FundSortKey>("daily_change_percent");
  const [sortOrder, setSortOrder] = useState<"none" | "asc" | "desc">("desc");
  const {
    getAllLists,
    getFundDetail,
    getFundTrend,
    getStorageWarning,
    getHolding,
    setHolding: saveHolding,
    clearHolding,
  } = useTauriCommands();

  const reloadDetail = async () => {
    if (!selectedFundCode || selectedListId === null) return;
    try {
      const detail = await getFundDetail(selectedListId, selectedFundCode);
      setFundDetail(detail);
    } catch (error) {
      setFundDetailError(String(error));
    }
  };

  // 加载所有列表
  const loadLists = async () => {
    try {
      const allLists = await getAllLists();
      setLists(allLists);
      setSelectedListId((prev) => {
        if (prev !== null) return prev;
        return allLists[0]?.id ?? null;
      });
      setListsError(null);
    } catch (error) {
      console.error("Failed to load lists:", error);
      setLists([]);
      setListsError(
        "数据加载失败，可能已损坏，已使用空列表。请检查备份文件。"
      );
    }
  };

  const handleListsChange = async () => {
    await loadLists();
  };

  useEffect(() => {
    loadLists();
  }, []);

  useEffect(() => {
    const loadWarning = async () => {
      try {
        const warning = await getStorageWarning();
        if (warning) {
          showToast(warning, "error");
        }
      } catch (error) {
        // ignore
      }
    };
    loadWarning();
  }, [getStorageWarning, showToast]);

  const selectedList = lists.find((l) => l.id === selectedListId);

  useEffect(() => {
    setSelectedFundCode(null);
    setFundDetail(null);
    setFundTrend(null);
    setFundDetailError(null);
    setHolding(null);
    setHoldingError(null);
  }, [selectedListId]);

  useEffect(() => {
    if (!selectedFundCode) {
      setFundDetail(null);
      setFundTrend(null);
      setFundDetailError(null);
      setHolding(null);
      setHoldingError(null);
      return;
    }
    if (selectedListId === null) {
      return;
    }

    const loadDetail = async (silent = false) => {
      if (!silent) {
        setFundDetailLoading(true);
        setFundDetailError(null);
      }
      try {
        const detail = await getFundDetail(selectedListId, selectedFundCode);
        setFundDetail(detail);
        if (detail.net_value !== null && detail.net_value !== undefined) {
          setFundTrend((prev) => {
            const timestamp = detail.update_time ?? new Date().toISOString();
            const newPoint = { date: timestamp, value: detail.net_value ?? 0 };
            if (!prev) {
              return {
                code: selectedFundCode,
                window: "实时",
                points: [newPoint],
              };
            }
            const last = prev.points[prev.points.length - 1];
            if (last && last.date === newPoint.date && last.value === newPoint.value) {
              return prev;
            }
            const points = [...prev.points, newPoint].slice(-60);
            return {
              ...prev,
              window: "最近30个交易日 + 实时",
              points,
            };
          });
        }
      } catch (error) {
        if (!silent) {
          setFundDetail(null);
          setFundDetailError(String(error));
        }
      } finally {
        if (!silent) {
          setFundDetailLoading(false);
        }
      }
    };

    const loadTrend = async () => {
      try {
        const trend = await getFundTrend(selectedFundCode);
        setFundTrend(trend);
      } catch (error) {
        setFundTrend(null);
      }
    };

    loadDetail(false);
    loadTrend();

    const timer = setInterval(() => {
      loadDetail(true);
    }, globalRefreshMs);

    return () => clearInterval(timer);
  }, [
    selectedFundCode,
    selectedListId,
    getFundDetail,
    getFundTrend,
    globalRefreshMs,
  ]);

  useEffect(() => {
    if (!selectedFundCode || selectedListId === null) {
      setHolding(null);
      return;
    }
    const loadHolding = async () => {
      setHoldingLoading(true);
      setHoldingError(null);
      try {
        const result = await getHolding(selectedListId, selectedFundCode);
        setHolding(result);
      } catch (error) {
        setHolding(null);
        setHoldingError(String(error));
      } finally {
        setHoldingLoading(false);
      }
    };
    loadHolding();
  }, [selectedListId, selectedFundCode, getHolding, holdingVersion]);

  const handleSaveHolding = async (amount: number, shares: number) => {
    if (!selectedFundCode || selectedListId === null) return;
    try {
      const result = await saveHolding(
        selectedListId,
        selectedFundCode,
        amount,
        shares
      );
      setHolding(result);
      setHoldingVersion((prev) => prev + 1);
      showToast("持仓已保存", "success");
      await reloadDetail();
    } catch (error) {
      showToast(String(error), "error");
    }
  };

  const handleClearHolding = async () => {
    if (!selectedFundCode || selectedListId === null) return;
    try {
      await clearHolding(selectedListId, selectedFundCode);
      setHolding(null);
      setHoldingVersion((prev) => prev + 1);
      showToast("持仓已清空", "success");
      await reloadDetail();
    } catch (error) {
      showToast(String(error), "error");
    }
  };

  return (
    <div className="app-shell">
      <header className="topbar">
        <div className="topbar-left">
          <span className="meta-pill">当前列表：{selectedList?.name || "未选择"}</span>
          <span className="meta-pill ghost">
            当前基金：{fundDetail?.name || selectedFundCode || "未选择"}
          </span>
        </div>
        <div className="topbar-meta">
          <span className="meta-pill">刷新间隔：{globalRefreshMs / 1000}s</span>
          <span className="meta-pill ghost">本地数据 · 自动更新</span>
        </div>
      </header>

      <div className={`layout ${leftCollapsed ? "panel-hidden" : ""}`}>
        <aside className={`panel left-panel ${leftCollapsed ? "hidden" : ""}`}>
          <div className="panel-header">
            <div>
              <div className="panel-title">基金列表</div>
              <div className="panel-subtitle">
                {lists.length} 个列表 · 选择后显示基金
              </div>
            </div>
            <button
              type="button"
              className="button ghost small"
              onClick={() => setLeftCollapsed(true)}
            >
              隐藏
            </button>
          </div>
          <div className="panel-body">
            {listsError && <div className="list-error">{listsError}</div>}
            <ListsPanel
              lists={lists}
              selectedListId={selectedListId}
              onSelectList={setSelectedListId}
              onListsChange={handleListsChange}
              showToast={showToast}
            />
          </div>
          <div className="panel-footer">
            <button
              type="button"
              className="button ghost small collapse-btn"
              onClick={() => setLeftCollapsed(true)}
            >
              收起列表
            </button>
          </div>
        </aside>

        {leftCollapsed && (
          <button
            type="button"
            className="panel-toggle-floating"
            onClick={() => setLeftCollapsed(false)}
            aria-label="展开列表"
          >
            展开列表
          </button>
        )}

        <main className="panel middle-panel">
          <div className="panel-body panel-body-fixed">
            <ListDetailView
              listId={selectedListId}
              listName={selectedList?.name || ""}
              selectedFundCode={selectedFundCode}
              onSelectFund={setSelectedFundCode}
              onListsChange={handleListsChange}
              showToast={showToast}
              refreshIntervalMs={globalRefreshMs}
              holdingVersion={holdingVersion}
              sortKey={sortKey}
              sortOrder={sortOrder}
              onSortKeyChange={setSortKey}
              onSortOrderChange={setSortOrder}
            />
          </div>
        </main>

        <aside className="panel right-panel">
          <div className="panel-body">
            <FundDetailPanel
              detail={fundDetail}
              trend={fundTrend}
              loading={fundDetailLoading}
              error={fundDetailError}
              holding={holding}
              holdingLoading={holdingLoading}
              holdingError={holdingError}
              onSaveHolding={handleSaveHolding}
              onClearHolding={handleClearHolding}
            />
          </div>
        </aside>
      </div>
    </div>
  );
}

export default App;
