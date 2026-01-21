import { useEffect, useState } from "react";
import { ListsPanel } from "./components/ListsPanel";
import { ListDetailView } from "./components/ListDetailView";
import { useTauriCommands } from "./hooks/useTauriCommands";
import { FundDetail, FundList, FundTrend } from "./types";
import { useToast } from "./components/ToastContext";
import { FundDetailPanel } from "./components/FundDetailPanel";
import "./App.css";

interface AppProps {
  globalRefreshMs: number;
  onChangeGlobalRefreshMs: (value: number) => void;
}

function App({ globalRefreshMs }: AppProps) {
  const showToast = useToast();
  const [lists, setLists] = useState<FundList[]>([]);
  const [selectedListId, setSelectedListId] = useState<number | null>(null);
  const [selectedFundCode, setSelectedFundCode] = useState<string | null>(null);
  const [listsError, setListsError] = useState<string | null>(null);
  const [fundDetail, setFundDetail] = useState<FundDetail | null>(null);
  const [fundTrend, setFundTrend] = useState<FundTrend | null>(null);
  const [fundAccumTrend, setFundAccumTrend] = useState<FundTrend | null>(null);
  const [fundDetailLoading, setFundDetailLoading] = useState(false);
  const [fundDetailError, setFundDetailError] = useState<string | null>(null);
  const [leftCollapsed, setLeftCollapsed] = useState(false);
  const {
    getAllLists,
    getFundDetail,
    getFundTrend,
    getFundAccumTrend,
    getStorageWarning,
  } = useTauriCommands();

  // 加载所有列表
  const loadLists = async () => {
    try {
      const allLists = await getAllLists();
      setLists(allLists);
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
    setFundAccumTrend(null);
    setFundDetailError(null);
  }, [selectedListId]);

  useEffect(() => {
    if (!selectedFundCode) {
      setFundDetail(null);
      setFundTrend(null);
      setFundAccumTrend(null);
      setFundDetailError(null);
      return;
    }

    const loadDetail = async (silent = false) => {
      if (!silent) {
        setFundDetailLoading(true);
        setFundDetailError(null);
      }
      try {
        const detail = await getFundDetail(selectedFundCode);
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

    const loadAccumTrend = async () => {
      try {
        const trend = await getFundAccumTrend(selectedFundCode);
        setFundAccumTrend(trend);
      } catch (error) {
        setFundAccumTrend(null);
      }
    };

    loadDetail(false);
    loadTrend();
    loadAccumTrend();

    const timer = setInterval(() => {
      loadDetail(true);
    }, globalRefreshMs);

    return () => clearInterval(timer);
  }, [
    selectedFundCode,
    getFundDetail,
    getFundTrend,
    getFundAccumTrend,
    globalRefreshMs,
  ]);

  return (
    <div className="app-container">
      <div className={`left-section ${leftCollapsed ? "collapsed" : ""}`}>
        <button
          type="button"
          className="left-toggle-btn"
          onClick={() => setLeftCollapsed((prev) => !prev)}
          aria-label={leftCollapsed ? "展开列表" : "隐藏列表"}
          title={leftCollapsed ? "展开列表" : "隐藏列表"}
        >
          {leftCollapsed ? "›" : "‹"}
        </button>
        {!leftCollapsed && (
          <>
            {listsError && <div className="list-error">{listsError}</div>}
            <ListsPanel
              lists={lists}
              selectedListId={selectedListId}
              onSelectList={setSelectedListId}
              onListsChange={handleListsChange}
              showToast={showToast}
            />
          </>
        )}
      </div>

      <div className="middle-section">
        <ListDetailView
          listId={selectedListId}
          listName={selectedList?.name || ""}
          selectedFundCode={selectedFundCode}
          onSelectFund={setSelectedFundCode}
          onListsChange={handleListsChange}
          showToast={showToast}
          refreshIntervalMs={globalRefreshMs}
        />
      </div>

      <div className="right-section">
        <FundDetailPanel
          detail={fundDetail}
          trend={fundTrend}
          accumTrend={fundAccumTrend}
          loading={fundDetailLoading}
          error={fundDetailError}
        />
      </div>
    </div>
  );
}

export default App;
