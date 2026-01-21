import React, { useEffect, useState } from "react";
import { FundSummary } from "../types";
import { useTauriCommands } from "../hooks/useTauriCommands";
import { ListDetail } from "./ListDetail";

interface ListDetailViewProps {
  listId: string | null;
  listName: string;
  selectedFundCode: string | null;
  onSelectFund: (code: string) => void;
  showToast?: (message: string, type: "success" | "error") => void;
  onListsChange: () => void;
  refreshIntervalMs: number;
}

export const ListDetailView: React.FC<ListDetailViewProps> = ({
  listId,
  listName,
  selectedFundCode,
  onSelectFund,
  showToast,
  onListsChange,
  refreshIntervalMs,
}) => {
  const [funds, setFunds] = useState<FundSummary[]>([]);
  const [loading, setLoading] = useState(false);
  const [newFundCode, setNewFundCode] = useState("");
  const { getListFundSummaries, addFundToList } = useTauriCommands();

  useEffect(() => {
    if (listId) {
      loadFunds(false);
    } else {
      setFunds([]);
    }
  }, [listId]);

  useEffect(() => {
    if (!listId) return;
    const timer = setInterval(() => {
      loadFunds(true);
    }, refreshIntervalMs);
    return () => clearInterval(timer);
  }, [listId, refreshIntervalMs]);

  const loadFunds = async (silent = false) => {
    if (!listId) return;

    if (!silent) {
      setLoading(true);
    }
    try {
      const fundList = await getListFundSummaries(listId);
      setFunds(fundList);
    } catch (error) {
      if (!silent && showToast) {
        showToast(String(error), "error");
      }
    } finally {
      if (!silent) {
        setLoading(false);
      }
    }
  };

  const handleAddFund = async () => {
    if (!listId) return;
    const code = newFundCode.trim();
    if (!/^[0-9]{6}$/.test(code)) {
      showToast?.("请输入6位基金代码", "error");
      return;
    }

    try {
      await addFundToList(listId, code);
      showToast?.("已添加到列表", "success");
      setNewFundCode("");
      loadFunds(false);
      onListsChange();
    } catch (error) {
      showToast?.(String(error), "error");
    }
  };

  if (!listId) {
    return (
      <div className="list-detail-view empty">
        <p>👈 请选择一个列表查看详情</p>
      </div>
    );
  }

  if (loading) {
    return (
      <div className="list-detail-view loading">
        <p>加载中...</p>
      </div>
    );
  }

  if (funds.length === 0) {
    return (
      <div className="list-detail-view empty">
        <h3>{listName}</h3>
        <div className="list-add-fund">
          <input
            type="text"
            value={newFundCode}
            onChange={(e) =>
              setNewFundCode(e.target.value.replace(/\D/g, "").slice(0, 6))
            }
            onKeyDown={(e) => {
              if (e.key === "Enter") {
                handleAddFund();
              }
            }}
            placeholder="输入基金代码添加"
            maxLength={6}
            className="list-add-input"
          />
          <button onClick={handleAddFund} className="btn-add-inline">
            添加
          </button>
        </div>
        <p>列表为空，添加一些基金吧</p>
      </div>
    );
  }

  return (
    <div className="list-detail-view">
      <div className="detail-header">
        <h3>{listName}</h3>
        <span className="fund-count">{funds.length}只基金</span>
      </div>

      <div className="list-add-fund">
        <input
          type="text"
          value={newFundCode}
          onChange={(e) =>
            setNewFundCode(e.target.value.replace(/\D/g, "").slice(0, 6))
          }
          onKeyDown={(e) => {
            if (e.key === "Enter") {
              handleAddFund();
            }
          }}
          placeholder="输入基金代码添加"
          maxLength={6}
          className="list-add-input"
        />
        <button onClick={handleAddFund} className="btn-add-inline">
          添加
        </button>
      </div>

      <div className="funds-list">
        {funds.map((fund) => (
          <ListDetail
            key={fund.code}
            fund={fund}
            selected={selectedFundCode === fund.code}
            onSelect={onSelectFund}
          />
        ))}
      </div>
    </div>
  );
};
